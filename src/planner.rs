use crate::calls::FunctionCall;
use crate::cmds::{Command, CommandFlags, CommandType, Literal, ReturnValue, Value};
use crate::error::WeirollError;


use ethers::abi::{AbiEncode, ParamType};
use ethers::prelude::*;
use slotmap::{DefaultKey, HopSlotMap};
use std::collections::{BTreeMap, BTreeSet};

type CommandKey = DefaultKey;


#[derive(Debug, Default, Clone)]

pub struct Planner<'a> {
    commands: HopSlotMap<CommandKey, Command<'a>>,
}

#[derive(Debug, Default)]
pub struct PlannerState {
    return_slot_map: BTreeMap<CommandKey, u8>,
    literal_slot_map: BTreeMap<Literal, u8>,
    free_slots: Vec<u8>,
    state_expirations: BTreeMap<CommandKey, Vec<u8>>,
    command_visibility: BTreeMap<CommandKey, CommandKey>,
    state: Vec<Bytes>,
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
    pub fn call<'a>(
        &mut self,
        address: Address,
        selector: [u8; 4],
        args: Vec<Value<'a>>,
        return_type: ParamType,
    ) -> Result<ReturnValue, WeirollError> {
        let dynamic = return_type.is_dynamic();
        let call = FunctionCall {
            address,
            flags: CommandFlags::CALL,
            value: Some(U256::zero()),
            selector,
            args,
            return_type,
        };

        let command = self.commands.insert(Command {
            call,
            kind: CommandType::Call,
        });

        Ok(ReturnValue { command, dynamic })
    }


    pub fn add_subplan<'a>(
        &mut self,
        address: Address,
        selector: [u8; 4],
        args: Vec<Value<'a>>,
        return_type: ParamType,
    ) -> Result<ReturnValue, WeirollError> {
        let dynamic = return_type.is_dynamic();

        let mut has_subplan = false;
        let mut has_state = false;

        if args.len() != 2 {
            return Err(WeirollError::ArgumentCountMismatch);
        }

        for arg in args.iter() {
            match arg {
                Value::Subplan(_planner) => {
                    if has_subplan {
                        return Err(WeirollError::MultipleSubplans);
                    }
                    has_subplan = true;
                }
                Value::State(_state) => {
                    if has_state {
                        return Err(WeirollError::MultipleState);
                    }
                    has_state = true;
                }
                _ => {}
            }
        }

        if !has_subplan || !has_state {
            return Err(WeirollError::MissingStateOrSubplan);
        }

        let command = self.commands.insert(Command {
            call: FunctionCall {
                address,
                flags: CommandFlags::DELEGATECALL,
                value: None,
                selector,
                args,
                return_type,
            },
            kind: CommandType::SubPlan,
        });

        Ok(ReturnValue { dynamic, command })
    }

    pub fn replace_state<C: EthCall>(&mut self, address: Address, args: Vec<Value<'a>>) {
        let call = FunctionCall {
            address,
            flags: CommandFlags::DELEGATECALL,
            value: None,
            selector: C::selector(),
            args,
            return_type: ParamType::Array(Box::new(ParamType::Bytes)),
        };
        self.commands.insert(Command {
            call,
            kind: CommandType::RawCall,
        });
    }

    fn build_command_args(
        &self,
        command: &Command,
        return_slot_map: &BTreeMap<CommandKey, u8>,
        literal_slot_map: &BTreeMap<Literal, u8>,
        state: &Vec<Bytes>,
    ) -> Result<Vec<u8>, WeirollError> {
        let in_args = Vec::from_iter(command.call.args.iter());
        let mut extra_args: Vec<Value> = vec![];
        if command.call.flags & CommandFlags::CALLTYPE_MASK == CommandFlags::CALL_WITH_VALUE {
            if let Some(value) = command.call.value {
                extra_args.push(Value::Literal(value.into()));
            } else {
                return Err(WeirollError::MissingValue);
            }
        }

        let mut args = vec![];
        for arg in in_args.into_iter().chain(extra_args.iter()) {
            let mut slot = match arg {
                Value::Return(val) => {
                    if let Some(slot) = return_slot_map.get(&val.command) {
                        *slot
                    } else {
                        return Err(WeirollError::MissingReturnSlot);
                    }
                }
                Value::Literal(val) => {
                    if let Some(slot) = literal_slot_map.get(val) {
                        *slot
                    } else {
                        return Err(WeirollError::MissingLiteralValue);
                    }
                }
                Value::State(_) => {
                    tracing::debug!("added state value, using 0xfe return slot");
                    0xFE
                }
                Value::Subplan(_) => {
                    tracing::debug!("added state value {state:?}");
                    // buildCommands has already built the subplan and put it in the last state slot
                    (state.len() - 1).try_into()?
                }
            };
            // todo- correct??
            if arg.is_dynamic_type() {
                slot |= 0x80;
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
                        Value::Subplan(planner) => Some(planner),
                        _ => None,
                    })
                    .ok_or(WeirollError::MissingSubplan)?;

                // Build a list of commands
                let subcommands = subplanner.build_commands(ps)?;

                // Push the commands onto the state
                ps.state.push(subcommands[0].clone().to_vec().into());

                // The slot is no longer needed after this command
                ps.free_slots.push((ps.state.len() - 1).try_into()?);
            }

            let mut flags = command.call.flags;

            let mut args = self.build_command_args(
                command,
                &ps.return_slot_map,
                &ps.literal_slot_map,
                &ps.state,
            )?;

            if args.len() > 6 {
                flags |= CommandFlags::EXTENDED_COMMAND;
            }

            // Add any expired state entries to free slots
            if let Some(expr) = ps.state_expirations.get(&cmd_key) {
                ps.free_slots.extend(expr.iter().copied())
            };

            // Figure out where to put the return value
            let mut ret = 0xff;
            if let Some(expiry) = ps.command_visibility.get(&cmd_key) {
                if let CommandType::RawCall | CommandType::SubPlan = command.kind {
                    return Err(WeirollError::InvalidReturnSlot);
                }

                ret = ps.state.len().try_into()?;
                if let Some(slot) = ps.free_slots.pop() {
                    ret = slot;
                }

                ps.return_slot_map.insert(cmd_key, ret);

                ps.state_expirations
                    .entry(*expiry)
                    .or_insert_with(Vec::new)
                    .push(ret);

                if ret == u8::try_from(ps.state.len())? {
                    ps.state.push(Bytes::default());
                }

                // todo: what's this?
                if command.call.return_type.is_dynamic() {
                    tracing::debug!("ret type is dynamic, set ret to 0x80");
                    ret |= 0x80;
                }
            } else if let CommandType::RawCall | CommandType::SubPlan = command.kind {
                // todo: what's this?
                // if command.call.fragment.outputs.len() == 1 {}
                tracing::debug!("call is raw or subplan, set ret to 0xfe");
                ret = 0xfe;
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

                let mut bytes: Vec<Bytes> = vec![
                    command.call.selector.into(),
                    flags.bits().to_le_bytes().to_vec().into(),
                ];

                // pad args to 6
                args.resize(6, 0xff);

                bytes.push(args.into());
                bytes.push([ret.to_le()].into());
                bytes.push(command.call.address.to_fixed_bytes().into());

                encoded_commands.push(bytes.concat().into());
            }
        }

        Ok(encoded_commands)
    }

    fn preplan(
        &self,
        literal_visibility: &mut Vec<(Literal, CommandKey)>,
        command_visibility: &mut BTreeMap<CommandKey, CommandKey>,
        seen: &mut BTreeSet<CommandKey>,
    ) -> Result<(), WeirollError> {
        for (cmd_key, command) in &self.commands {
            let in_args = &command.call.args;
            let mut extra_args = vec![];

            if command.call.flags & CommandFlags::CALLTYPE_MASK == CommandFlags::CALL_WITH_VALUE {
                if let Some(value) = command.call.value {
                    extra_args.push(value.into());
                } else {
                    return Err(WeirollError::MissingValue);
                }
            }

            for arg in in_args.iter().chain(extra_args.iter()) {
                match arg {
                    Value::Return(val) => {
                        if !seen.contains(&val.command) {
                            return Err(WeirollError::CommandNotVisible);
                        }
                        command_visibility.insert(val.command, cmd_key);
                    }
                    Value::Literal(val) => {
                        // Remove old visibility (if exists)
                        literal_visibility.retain(|(l, _)| *l != *val);
                        literal_visibility.push((val.clone(), cmd_key));
                    }
                    Value::State(_) => {}
                    Value::Subplan(subplan) => {
                        // let mut subplan_seen = Default::default();
                        if command.call.return_type.is_dynamic() {
                            subplan.preplan(literal_visibility, command_visibility, seen)?;
                        }
                    }
                }
            }

            seen.insert(cmd_key);
        }

        dbg!(&command_visibility, &literal_visibility);

        Ok(())
    }

    pub fn plan(&self) -> Result<(Vec<Bytes>, Vec<Bytes>), WeirollError> {
        // Tracks the last time a literal is used in the program
        let mut literal_visibility = Default::default();

        // Tracks the last time a command's output is used in the program
        let mut command_visibility = Default::default();

        // Populate visibility maps
        self.preplan(
            &mut literal_visibility,
            &mut command_visibility,
            &mut BTreeSet::new(),
        )?;


        // Maps from commands to the slots that expire on execution (if any)
        let mut state_expirations: BTreeMap<CommandKey, Vec<u8>> = Default::default();

        // Tracks the state slot each literal is stored in
        let mut literal_slot_map: BTreeMap<Literal, u8> = Default::default();

        // empty initial state
        let mut state: Vec<Bytes> = Default::default();

        // Prepopulate the state and state expirations with literals
        for (literal, last_command) in literal_visibility {
            let slot = state.len() as u8;
            state.push(literal.bytes());
            state_expirations
                .entry(last_command)
                .or_insert_with(Vec::new)
                .push(slot);
            literal_slot_map.insert(literal, slot);
        }

        let mut ps = PlannerState {
            return_slot_map: Default::default(),
            literal_slot_map,
            free_slots: Default::default(),
            state_expirations,
            command_visibility,
            state,
        };

        let encoded_commands = self.build_commands(&mut ps)?;

        Ok((encoded_commands, ps.state))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bindings::{
        math::AddCall,
        strings::{StrcatCall, StrlenCall},
    };
    use ethers::abi::AbiEncode;

    abigen!(
        SampleContract,
        r#"[
            function useState(bytes[] state) returns(bytes[])
        ]"#,
    );

    abigen!(
        SubplanContract,
        r#"[
            function execute(bytes32[] commands, bytes[] state) returns(bytes[])
        ]"#,
    );

    abigen!(
        ReadOnlySubplanContract,
        r#"[
            function execute(bytes32[] commands, bytes[] state)
        ]"#,
    );

    fn addr() -> Address {
        "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
            .parse()
            .unwrap()
    }

    #[test]
    fn test_planner_add() {
        let mut planner = Planner::default();
        planner
            .call(
                addr(),
                AddCall::selector(),
                vec![U256::from(1).into(), U256::from(2).into()],
                ParamType::Uint(256),
            )
            .expect("can add call");
        let (commands, state) = planner.plan().expect("plan");

        assert_eq!(commands.len(), 1);
        assert_eq!(
            commands[0],
            "0x771602f7000001ffffffffffeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                .parse::<Bytes>()
                .unwrap()
        );

        assert_eq!(state.len(), 2);
        assert_eq!(state[0], U256::from(1).encode());
        assert_eq!(state[1], U256::from(2).encode());
    }

    #[test]
    fn test_planner_deduplicates_literals() {
        let mut planner = Planner::default();
        planner
            .call(
                addr(),
                AddCall::selector(),
                vec![U256::from(1).into(), U256::from(1).into()],
                ParamType::Uint(256),
            )
            .expect("can add call");
        let (_, state) = planner.plan().expect("plan");
        assert_eq!(state.len(), 1);
    }

    #[test]
    fn test_planner_return_values() {
        let mut planner = Planner::default();
        let ret = planner
            .call(
                addr(),
                AddCall::selector(),
                vec![U256::from(1).into(), U256::from(2).into()],
                ParamType::Uint(256),
            )
            .expect("can add call");
        planner
            .call(
                addr(),
                AddCall::selector(),
                vec![ret.into(), U256::from(3).into()],
                ParamType::Uint(256),
            )
            .expect("can add call with return val");
        let (commands, state) = planner.plan().expect("plan");
        assert_eq!(commands.len(), 2);
        assert_eq!(
            commands[0],
            "0x771602f7000001ffffffff01eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                .parse::<Bytes>()
                .unwrap()
        );
        assert_eq!(
            commands[1],
            "0x771602f7000102ffffffffffeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                .parse::<Bytes>()
                .unwrap()
        );
        assert_eq!(state.len(), 3);
        assert_eq!(state[0], U256::from(1).encode());
        assert_eq!(state[1], U256::from(2).encode());
        assert_eq!(state[2], U256::from(3).encode());
    }

    #[test]
    fn test_planner_intermediate_state_slots() {
        // todo: how is this different from test_planner_return_values?
        let mut planner = Planner::default();
        let ret = planner
            .call(
                addr(),
                AddCall::selector(),
                vec![U256::from(1).into(), U256::from(1).into()],
                ParamType::Uint(256),
            )
            .expect("can add call");
        planner
            .call(
                addr(),
                AddCall::selector(),
                vec![U256::from(1).into(), ret.into()],
                ParamType::Uint(256),
            )
            .expect("can add call with return val");
        let (commands, state) = planner.plan().expect("plan");
        assert_eq!(commands.len(), 2);
        assert_eq!(
            commands[0],
            "0x771602f7000000ffffffff01eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                .parse::<Bytes>()
                .unwrap()
        );
        assert_eq!(
            commands[1],
            "0x771602f7000001ffffffffffeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                .parse::<Bytes>()
                .unwrap()
        );
        assert_eq!(state.len(), 2);
        assert_eq!(state[0], U256::from(1).encode());
        assert_eq!(state[1], Bytes::default());
    }

    #[test]
    fn test_planner_dynamic_arguments() {
        let mut planner = Planner::default();
        planner
            .call(
                addr(),
                StrlenCall::selector(),
                vec![String::from("Hello, world!").into()],
                ParamType::Uint(256),
            )
            .expect("can add call");
        let (commands, state) = planner.plan().expect("plan");
        assert_eq!(commands.len(), 1);
        assert_eq!(
            commands[0],
            "0x367bbd780080ffffffffffffeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                .parse::<Bytes>()
                .unwrap()
        );
        assert_eq!(state.len(), 1);
        assert_eq!(state[0], "Hello, world!".to_string().encode());
    }

    #[test]
    fn test_planner_dynamic_return_values() {
        let mut planner = Planner::default();

        let ret = planner
            .call(
                addr(),
                StrcatCall::selector(),
                vec![
                    String::from("Hello, ").into(),
                    String::from("world!").into(),
                ],
                ParamType::String,
            )
            .expect("can add call");
        let (commands, state) = planner.plan().expect("plan");
        assert_eq!(commands.len(), 1);
        assert_eq!(
            commands[0],
            "0xd824ccf3008081ffffffffffeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                .parse::<Bytes>()
                .unwrap()
        );
        assert_eq!(state.len(), 2);
        assert_eq!(state[0], "Hello, ".to_string().encode());
        assert_eq!(state[1], "world!".to_string().encode());
    }

    #[test]
    fn test_planner_dynamic_return_values_with_dynamic_arguments() {
        let mut planner = Planner::default();
        let ret = planner
            .call(
                addr(),
                StrcatCall::selector(),
                vec![
                    String::from("Hello, ").into(),
                    String::from("world!").into(),
                ],
                ParamType::String,
            )
            .expect("can add call");
        planner
            .call(addr(),StrlenCall::selector(),  vec![ret.into()], ParamType::Uint(256))
            .expect("can add call with return val");
        let (commands, state) = planner.plan().expect("plan");
        assert_eq!(commands.len(), 2);
        assert_eq!(
            commands[0],
            "0xd824ccf3008081ffffffff81eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                .parse::<Bytes>()
                .unwrap()
        );
        assert_eq!(
            commands[1],
            "0x367bbd780081ffffffffffffeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                .parse::<Bytes>()
                .unwrap()
        );
        assert_eq!(state.len(), 2);
        assert_eq!(state[0], "Hello, ".to_string().encode());
        assert_eq!(state[1], "world!".to_string().encode());
    }

    #[test]
    fn test_planner_argument_count_mismatch() {
        let mut planner = Planner::default();

        let ret = planner.add_subplan(
            addr(),
            AddCall::selector(),
            vec![U256::from(1).into()],
            ParamType::Uint(256),
        );
        assert_eq!(ret.err(), Some(WeirollError::MissingStateOrSubplan));

    }

    #[test]
    fn test_planner_replace_state() {
        let mut planner = Planner::default();
        planner.replace_state::<UseStateCall>(addr(), vec![Value::State(Default::default())]);
        let (commands, state) = planner.plan().expect("plan");
        assert_eq!(commands.len(), 1);
        assert_eq!(
            commands[0],
            "0x08f389c800fefffffffffffeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                .parse::<Bytes>()
                .unwrap()
        );
        assert_eq!(state.len(), 0);
    }

    #[test]
    fn test_planner_supports_subplans() {
        let mut subplanner = Planner::default();
        subplanner
            .call(
                addr(),
                AddCall::selector(),
                vec![U256::from(1).into(), U256::from(2).into()],
                ParamType::Uint(256),
            )
            .expect("can add call");
        let mut planner = Planner::default();

        let ret = planner
            .add_subplan(
                addr(),
                subplan_contract::ExecuteCall::selector(),
                vec![Value::Subplan(subplanner), Value::State(Default::default())],
                ParamType::Array(Box::new(ParamType::Bytes)),
            )
            .expect("can add subplan");
        let (commands, state) = planner.plan().expect("plan");
        assert_eq!(commands.len(), 1);
        assert_eq!(
            commands[0],
            "0xde792d5f0082fefffffffffeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                .parse::<Bytes>()
                .unwrap()
        );
        assert_eq!(state.len(), 3);
        assert_eq!(state[0], U256::from(1).encode());
        assert_eq!(state[1], U256::from(2).encode());

        // not sure what we are checking here
        let subcommands_bytes = concat_bytes(&vec![
            "0x0000000000000000000000000000000000000000000000000000000000000020"
                .parse()
                .unwrap(),
            state[2].clone(),
        ]);

        // let decoded = Vec::<Bytes>::decode(subcommands_bytes).unwrap();
        println!("state: {:?}", state);
        assert_eq!(
            state[2].clone(),
            "0x771602f7000001ffffffffffeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                .parse::<Bytes>()
                .unwrap()
        );
        // let subcommands = &Vec::<Bytes>::decode(subcommands_bytes).unwrap()[0];
        // let decoded: Vec<Vec<u8>> = Vec::<Vec<u8>>::decode(subcommands_bytes).unwrap();
        // assert_eq!(decoded.len(), 1);
        // assert_eq!(
        //     Bytes::from(decoded[0].clone()),
        //     "0x771602f7000001ffffffffffeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
        //         .parse::<Bytes>()
        //         .unwrap()
        // );
    }

    #[test]
    fn test_planner_allows_return_value_access_in_parent_scope() {
        let mut subplanner = Planner::default();
        let sum = subplanner
            .call(
                addr(),
                AddCall::selector(),
                vec![U256::from(1).into(), U256::from(2).into()],
                ParamType::Uint(256),
            )
            .expect("can add call");
        let mut planner = Planner::default();
        planner
            .add_subplan(
                addr(),
                subplan_contract::ExecuteCall::selector(),
                vec![Value::Subplan(subplanner), Value::State(Default::default())],
                ParamType::Array(Box::new(ParamType::Bytes)),
            )
            .expect("can add subplan");
        planner
            .call(
                addr(),
                AddCall::selector(),
                vec![sum.into(), U256::from(3).into()],
                ParamType::Uint(256),
            )
            .expect("can add call");
        let (commands, _) = planner.plan().expect("plan");
        assert_eq!(commands.len(), 2);
        assert_eq!(
            commands[0],
            // Invoke subplanner
            "0xde792d5f0083fefffffffffeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                .parse::<Bytes>()
                .unwrap()
        );
        assert_eq!(
            commands[1],
            // sum + 3
            "0x771602f7000102ffffffffffeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
                .parse::<Bytes>()
                .unwrap()
        );
    }

    #[test]
    fn test_planner_allows_return_value_access_across_scopes() {
        let mut subplanner1 = Planner::default();
        let sum = subplanner1
            .call::<AddCall>(
                addr(),
                vec![U256::from(1).into(), U256::from(2).into()],
                ParamType::Uint(256),
            )
            .expect("can add call");

        let mut subplanner2 = Planner::default();
        subplanner2
            .call::<AddCall>(
                addr(),
                vec![sum.into(), U256::from(3).into()],
                ParamType::Uint(256),
            )
            .expect("can add call");

        let mut planner = Planner::default();
        planner
            .add_subplan::<subplan_contract::ExecuteCall>(
                addr(),
                vec![
                    Value::Subplan(&subplanner1),
                    Value::State(Default::default()),
                ],
                ParamType::Array(Box::new(ParamType::Bytes)),
            )
            .expect("can add subplan");
        planner
            .add_subplan::<subplan_contract::ExecuteCall>(
                addr(),
                vec![
                    Value::Subplan(&subplanner2),
                    Value::State(Default::default()),
                ],
                ParamType::Array(Box::new(ParamType::Bytes)),
            )
            .expect("can add subplan");

        let (commands, state) = planner.plan().expect("plan");
    }
}
