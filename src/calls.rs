use ethers::{abi::ParamType, prelude::*};

use crate::cmds::{CommandFlags, Value};

// pub trait FunctionInput: AbiEncode + std::fmt::Debug {}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub(crate) address: Address,
    pub(crate) selector: [u8; 4],
    pub(crate) flags: CommandFlags,
    pub(crate) value: Option<U256>,
    pub(crate) args: Vec<Value>,
    pub(crate) return_type: ParamType,
}

impl FunctionCall {
    pub fn with_value(mut self, value: U256) -> Self {
        self.flags = (self.flags & !CommandFlags::CALLTYPE_MASK) | CommandFlags::CALL_WITH_VALUE;
        self.value = Some(value);
        self
    }

    pub fn raw_value(mut self) -> Self {
        self.flags |= CommandFlags::TUPLE_RETURN;
        self
    }

    pub fn static_call(mut self) -> Self {
        if (self.flags & CommandFlags::CALLTYPE_MASK) != CommandFlags::CALL {
            panic!("Only CALL operations can be made static");
        }
        self.flags |= CommandFlags::TUPLE_RETURN;
        self
    }
}

// impl<M: Middleware, D: Detokenize> From<ContractCall<M, D>> for FunctionCall {
//     fn from(call: ContractCall<M, D>) -> Self {
//         let args = Vec::new();
//         Self {
//             contract: *call.tx.to_addr().unwrap(),
//             flags: CommandFlags::empty(),
//             args,
//             value: call.tx.value().cloned(),
//         }
//     }
// }
