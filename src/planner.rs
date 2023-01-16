use crate::calls::FunctionCall;
use crate::cmds::{Command, CommandFlags, CommandType, Literal, ReturnValue, Value};
use crate::error::WeirollError;

use ethers::abi::{Detokenize, Token};
use ethers::prelude::builders::ContractCall;
use ethers::prelude::*;
use slotmap::{DefaultKey, HopSlotMap};
use std::collections::{HashMap, HashSet};

type CommandKey = DefaultKey;

#[derive(Debug, Default)]
pub struct Planner {
    commands: HopSlotMap<CommandKey, Command>,
    returns: Vec<ReturnValue>,
}

#[derive(Debug, Default)]
pub struct PlannerState {
    return_slot_map: HashMap<CommandKey, U256>,
    literal_slot_map: HashMap<Literal, U256>,
    free_slots: Vec<U256>,
    state_expirations: HashMap<CommandKey, Vec<U256>>,
    command_visibility: HashMap<CommandKey, CommandKey>,
    state: Vec<U256>,
}

fn u256_bytes(u: U256) -> Bytes {
    let mut bytes = [0u8; 32];
    u.to_little_endian(&mut bytes);
    bytes.into()
}

fn concat_bytes(items: &[Bytes]) -> Bytes {
    let mut result = Vec::<u8>::new();
    for item in items {
        result.extend_from_slice(&item.0)
    }
    result.into()
}

fn pad_array<T>(array: Vec<T>, len: usize, value: T) -> Vec<T>
where
    T: Clone,
{
    let mut out = array;
    out.resize(len, value);
    out
}

impl Planner {
    pub fn call<'a, C: EthCall, R>(
        &mut self,
        address: Address,
        args: Vec<Value>,
    ) -> Option<ReturnValue> {
        let call = FunctionCall {
            address,
            flags: CommandFlags::empty(),
            value: Some(U256::zero()),
            selector: C::selector(),
            args,
        };

        let command = self.commands.insert(Command {
            call,
            kind: CommandType::Call,
        });

        let rv = ReturnValue { command };
        self.returns.push(rv.clone());
        Some(rv)
    }

    pub fn subcall<M: Middleware, R: Detokenize>(
        &mut self,
        _cmd: ContractCall<M, R>,
    ) -> Option<ReturnValue> {
        todo!()
    }

    fn build_command_args(
        &self,
        command: &Command,
        return_slot_map: &HashMap<CommandKey, U256>,
        literal_slot_map: &HashMap<Literal, U256>,
        state: &Vec<U256>,
    ) -> Result<Vec<U256>, WeirollError> {
        let in_args = Vec::from_iter(command.call.args.iter());
        let mut extra_args = vec![];
        if command.call.flags & CommandFlags::CALLTYPE_MASK == CommandFlags::CALL_WITH_VALUE {
            if let Some(value) = command.call.value {
                let value = Value::LiteralValue(Literal(Token::Uint(value)));
                extra_args.push(value);
            } else {
                return Err(WeirollError::MissingValue);
            }
        }

        let mut args = vec![];
        for arg in in_args.into_iter().chain(extra_args.iter()) {
            let mut slot = match arg {
                Value::ReturnValue(val) => {
                    if let Some(slot) = return_slot_map.get(&val.command) {
                        *slot
                    } else {
                        return Err(WeirollError::MissingReturnSlot);
                    }
                }
                Value::LiteralValue(val) => {
                    if let Some(slot) = literal_slot_map.get(val) {
                        *slot
                    } else {
                        return Err(WeirollError::MissingLiteralValue);
                    }
                }
                Value::StateValue(_) => U256::from(0xFE),
                Value::SubplanValue(_, _) => {
                    // buildCommands has already built the subplan and put it in the last state slot
                    U256::from(state.len() + 1)
                }
            };

            // todo- correct??
            if arg.is_dynamic_type() {
                slot |= U256::from(0x80);
            }

            args.push(slot);
        }

        Ok(args)
    }

    fn build_commands(&self, ps: &mut PlannerState) -> Result<Vec<Bytes>, WeirollError> {
        let mut encoded_commands = vec![];

        // Build commands, and add state entries as needed
        for (cmd_key, command) in &self.commands {
            if command.kind == CommandType::SubPlan {
                // Find the subplan
                let subplanner = command
                    .call
                    .args
                    .iter()
                    .find_map(|arg| match arg {
                        Value::SubplanValue(_, planner) => Some(planner),
                        _ => None,
                    })
                    .ok_or(WeirollError::MissingSubplan)?;

                // Build a list of commands
                let _subcommands = subplanner.build_commands(ps)?;

                // Encode them and push them to a new state slot
                // ps.state.push(
                //     hex_data_slice(
                //         default_abi_coder::encode(&["bytes32[]"], &[subcommands]),
                //         32,
                //     )
                //     .to_string(),
                // );
                // The slot is no longer needed after this command
                ps.free_slots.push(U256::from(ps.state.len() - 1));
            }

            let mut flags = command.call.flags;

            let args = self.build_command_args(
                command,
                &ps.return_slot_map,
                &ps.literal_slot_map,
                &ps.state,
            )?;

            if args.len() > 6 {
                flags |= CommandFlags::EXTENDED_COMMAND;
            }

            // Add any newly unused state slots to the list
            ps.free_slots = ps
                .free_slots
                .iter()
                .chain(ps.state_expirations.get(&cmd_key).unwrap())
                .copied()
                .collect();

            // Figure out where to put the return value
            let mut ret = U256::from(0xff);
            if ps.command_visibility.get(&cmd_key).is_some() {
                if let CommandType::RawCall | CommandType::SubPlan = command.kind {
                    return Err(WeirollError::InvalidReturnSlot);
                }

                ret = U256::from(ps.state.len());

                if let Some(slot) = ps.free_slots.pop() {
                    ret = slot;
                }

                ps.return_slot_map.insert(cmd_key, ret);

                let expiry_command = ps.command_visibility.get(&cmd_key).unwrap();
                ps.state_expirations
                    .entry(*expiry_command)
                    .or_insert_with(Vec::new)
                    .push(ret);

                if ret == U256::from(ps.state.len()) {
                    ps.state.push(U256::zero());
                }

                // todo: what's this?
                tracing::warn!("something something call fragment");

                // if command.call.fragment.outputs[0].is_dynamic_type() {
                //     ret |= 0x80;
                // }
            } else if let CommandType::RawCall | CommandType::SubPlan = command.kind {
                // todo: what's this?
                // if command.call.fragment.outputs.len() == 1 {}
                ret = U256::from(0xfe);
            }

            if (flags & CommandFlags::EXTENDED_COMMAND) == CommandFlags::EXTENDED_COMMAND {
                // todo
                tracing::warn!("extended command");
                // Extended command
                // encoded_commands.push(
                //     hex_concat(&[
                //         command
                //             .call
                //             .contract
                //             .interface
                //             .get_sighash(&command.call.fragment),
                //         &[
                //             flags,
                //             0,
                //             0,
                //             0,
                //             0,
                //             0,
                //             0,
                //             ret.low_u64() as u8,
                //             ret.low_u64() as u8,
                //             ret.low_u64() as u8,
                //             ret.low_u64() as u8,
                //             ret.low_u64() as u8,
                //             ret.low_u64() as u8,
                //             ret.low_u64() as u8,
                //             ret.low_u64() as u8,
                //         ],
                //         &command.call.contract,
                //     ])
                //     .to_string(),
                // );
                // encoded_commands.push(hex_concat(&[pad_array(&args, 32, 0xff)]));
            } else {
                // Standard command

                // todo: w.t.f
                let mut bytes = vec![
                    command.call.selector.into(),
                    flags.bits().to_le_bytes()[0..1].to_vec().into(),
                ];
                bytes.extend(
                    pad_array(args, 6, U256::from(0xff))
                        .iter()
                        .map(|o| u256_bytes(*o)[0..1].to_vec().into()),
                );
                bytes.push(u256_bytes(ret)[0..1].to_vec().into());
                bytes.push(command.call.address.to_fixed_bytes().into());
                encoded_commands.push(concat_bytes(&bytes));
            }
        }
        Ok(encoded_commands)
    }

    fn preplan(
        &self,
        literal_visibility: &mut HashMap<Literal, CommandKey>,
        command_visibility: &mut HashMap<CommandKey, CommandKey>,
        seen: &mut HashSet<CommandKey>,
        _planners: &mut HashSet<Planner>,
    ) -> Result<(), WeirollError> {
        for (cmd_key, command) in &self.commands {
            let in_args = &command.call.args;
            let mut extra_args = vec![];

            if command.call.flags & CommandFlags::CALLTYPE_MASK == CommandFlags::CALL_WITH_VALUE {
                if let Some(value) = command.call.value {
                    extra_args.push(Value::LiteralValue(Literal(Token::Int(value))));
                } else {
                    return Err(WeirollError::MissingValue);
                }
            }

            for arg in in_args.iter().chain(extra_args.iter()) {
                match arg {
                    Value::ReturnValue(val) => {
                        if seen.contains(&val.command) {
                            command_visibility.insert(val.command, cmd_key);
                        }
                    }
                    Value::LiteralValue(val) => {
                        literal_visibility.insert(val.clone(), cmd_key);
                    }
                    Value::StateValue(_) => todo!(),
                    Value::SubplanValue(_, _) => todo!(),
                }
            }

            seen.insert(cmd_key);
        }
        Ok(())
    }

    pub fn plan(&self) -> Result<(Vec<Bytes>, Vec<U256>), WeirollError> {
        // Tracks the last time a literal is used in the program
        let mut literal_visibility: HashMap<Literal, CommandKey> = HashMap::new();

        // Tracks the last time a command's output is used in the program
        let mut command_visibility: HashMap<CommandKey, CommandKey> = HashMap::new();

        // Populate visibility maps
        self.preplan(
            &mut literal_visibility,
            &mut command_visibility,
            &mut HashSet::new(),
            &mut HashSet::new(),
        )?;

        dbg!(&literal_visibility, &command_visibility);

        // Maps from commands to the slots that expire on execution (if any)
        let mut state_expirations: HashMap<CommandKey, Vec<U256>> = Default::default();

        // Tracks the state slot each literal is stored in
        let mut literal_slot_map: HashMap<Literal, U256> = Default::default();

        let mut state: Vec<U256> = Default::default();

        // Prepopulate the state and state expirations with literals
        for (literal, last_command) in literal_visibility {
            let slot = state.len();
            state.push(literal.clone().into());
            state_expirations
                .entry(last_command)
                .or_insert_with(Vec::new)
                .push(slot.into());
            literal_slot_map.insert(literal, slot.into());
        }

        let mut ps = PlannerState {
            return_slot_map: Default::default(),
            literal_slot_map,
            free_slots: Default::default(),
            state_expirations,
            command_visibility,
            state: state.clone(),
        };

        let encoded_commands = self.build_commands(&mut ps)?;

        Ok((encoded_commands, state))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bindings::math::AddCall;

    #[tokio::test]
    async fn test_planner_add() {
        let mut planner = Planner::default();
        planner
            .call::<AddCall, U256>(
                "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                    .parse()
                    .unwrap(),
                vec![U256::from(1).into(), U256::from(2).into()],
            )
            .expect("can plan simple 1 + 2");
        let (commands, state) = planner.plan().expect("plan");

        assert_eq!(commands.len(), 1);
        assert_eq!(
            commands[0],
            "0x771602f7000001ffffffffffeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                .parse::<Bytes>()
                .unwrap()
        );

        assert_eq!(state.len(), 2);
        assert_eq!(state[0], U256::from(1));
        assert_eq!(state[1], U256::from(2));
    }
}
