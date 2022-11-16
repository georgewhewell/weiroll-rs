use crate::calls::FunctionCall;
use crate::cmds::{Command, CommandType, ReturnValue};
use bitflags::bitflags;
use ethers::abi::{Detokenize, Function, Tokenizable};
use ethers::prelude::builders::ContractCall;
use ethers::prelude::*;
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Default)]
pub struct Planner {
    commands: Vec<Command>,
}

impl Planner {
    pub fn call<T: Tokenizable>(&mut self, f: T) -> Option<ReturnValue<T>> {
        let token = f.into_token();
        println!("tokens: {token:?}");
        let command = Command {
            call: f.encode(),
            kind: CommandType::Call,
        };
        // f.
        // self.commands.push(Command {
        //     call: cmd.into(),
        //     kind: CommandType::Call,
        // });
        Some(ReturnValue {
            param: todo!(),
            command: f,
            output: PhantomData,
        })
    }

    pub fn subcall<M: Middleware, R: Detokenize>(
        &mut self,
        _cmd: ContractCall<M, R>,
    ) -> Option<ReturnValue<R>> {
        todo!()
    }

    fn preplan(self) -> (HashMap<String, Command>, HashMap<Command, Command>) {
        // let mut literal_visibility: HashMap<String, Command> = HashMap::new();
        // let mut command_visibility: HashMap<Command, Command> = HashMap::new();

        // let mut seen = HashSet::new();
        // let mut planners: HashSet::new();

        // for command in self.commands {
        //     let args =
        // }
        todo!()
    }

    fn plan(self) {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::bindings::math::AddCall;
    use crate::bindings::events::LogUintCall;

    #[tokio::test]
    async fn test_planner_add() {
        let mut planner = Planner::default();
        let ret = planner.call(AddCall {
            a: U256::one(),
            b: U256::one(),
        });
        // planner.call(LogUintCall { message: ret });
        // let (commands, state) = planner.plan()?;
    }
}