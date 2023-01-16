use crate::calls::FunctionCall;
use crate::Planner;
use bitflags::bitflags;
use ethers::abi::{Token, Tokenizable};
use ethers::prelude::*;
use ethers::utils::keccak256;
use slotmap::DefaultKey;

use std::hash::Hash;

bitflags! {
    #[repr(transparent)]
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

#[derive(Debug, PartialEq)]
pub enum CommandType {
    Call,
    RawCall,
    SubPlan,
}

// Wrap Token so we can implement Hash and Eq
#[derive(Clone, Debug, PartialEq)]
pub struct Literal(pub Token);

impl Hash for Literal {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // todo: will this work?
        keccak256(self.0.to_string().as_bytes()).hash(state)
    }
}

impl Eq for Literal {
    fn assert_receiver_is_total_eq(&self) {
        // ???
    }
}

// todo: other types
impl Into<U256> for Literal {
    fn into(self) -> U256 {
        match self.0 {
            Token::Uint(u) => u,
            _ => panic!("Expected uint"),
        }
    }
}

#[derive(Debug)]
pub enum Value {
    LiteralValue(Literal),
    ReturnValue(ReturnValue),
    StateValue(Bytes),
    SubplanValue(Bytes, Planner),
}

impl<T> From<T> for Value
where
    T: Tokenizable,
{
    fn from(value: T) -> Self {
        Self::LiteralValue(Literal(value.into_token()))
    }
}

impl<'a> From<ReturnValue> for Value {
    fn from(value: ReturnValue) -> Self {
        Self::ReturnValue(value)
    }
}

impl Value {
    pub fn is_dynamic_type(&self) -> bool {
        match self {
            Value::LiteralValue(l) => l.0.is_dynamic(),
            Value::ReturnValue(_) => false,
            Value::StateValue(_) => false,
            Value::SubplanValue(_, _) => false,
        }
    }
}

#[derive(Debug)]
pub struct Command {
    pub(crate) call: FunctionCall,
    pub(crate) kind: CommandType,
}

#[derive(Clone, Debug)]
pub struct ReturnValue {
    // pub(crate) param: String,
    pub(crate) command: DefaultKey,
    // pub(crate) _return_type: PhantomData<R>,
}
