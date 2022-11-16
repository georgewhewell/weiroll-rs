use ethers::abi::Detokenize;
use ethers::prelude::builders::ContractCall;
use ethers::prelude::*;

use crate::cmds::{CommandFlags, Value};

pub struct FunctionCall {
    contract: Address,
    flags: CommandFlags,
    value: U256,
    args: Vec<Value>,
}

impl FunctionCall {
    fn with_value(mut self, value: U256) -> Self {
        self.flags = (self.flags & !CommandFlags::CALLTYPE_MASK) | CommandFlags::CALL_WITH_VALUE;
        self.value = value;
        self
    }

    fn raw_value(mut self) -> Self {
        self.flags |= CommandFlags::TUPLE_RETURN;
        self
    }

    fn static_call(mut self) -> Self {
        if (self.flags & CommandFlags::CALLTYPE_MASK) != CommandFlags::CALL {
            panic!("Only CALL operations can be made static");
        }
        self.flags |= CommandFlags::TUPLE_RETURN;
        self
    }
}

impl<M: Middleware, D: Detokenize> From<ContractCall<M, D>> for FunctionCall {
    fn from(call: ContractCall<M, D>) -> Self {
        let args = Vec::new();
        Self {
            contract: *call.tx.to_addr().unwrap(),
            flags: CommandFlags::empty(),
            args,
            value: call.tx.value().cloned().unwrap_or_default(),
        }
    }
}
