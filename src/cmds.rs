use crate::calls::FunctionCall;
use crate::Planner;
use bitflags::bitflags;
use ethers::abi::Detokenize;
use ethers::prelude::*;
use std::marker::PhantomData;

bitflags! {
    pub struct CommandFlags: u32 {
        // Specifies that a call should be made using the DELEGATECALL opcode
        const DELEGATECALL = 0x00;
        // Specifies that a call should be made using the CALL opcode
        const CALL = 0x01;
        // Specifies that a call should be made using the STATICCALL opcode
        const STATICCALL = 0x02;
        // Specifies that a call should be made using the CALL opcode, and that the first argument will be the value to send
        const CALL_WITH_VALUE = 0x03;
        // A bitmask that selects calltype flags
        const CALLTYPE_MASK = 0x03;
        // Specifies that this is an extended command, with an additional command word for indices. Internal use only.
        const EXTENDED_COMMAND = 0x40;
        // Specifies that the return value of this call should be wrapped in a `bytes`. Internal use only.
        const TUPLE_RETURN = 0x80;
    }
}

pub enum CommandType {
    Call,
    RawCall,
    SubPlan,
}

pub enum Value {
    Value,
    LiteralValue(String, String),
    StateValue(Bytes),
    SubplanValue(Bytes, Planner),
}

pub struct Command {
    pub(crate) call: FunctionCall,
    pub(crate) kind: CommandType,
}

pub struct ReturnValue<R: Detokenize> {
    param: String,
    command: Command,
    output: PhantomData<R>,
}
