pub use vm::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod vm {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    ///VM was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
    use std::sync::Arc;
    use ::ethers::core::{
        abi::{Abi, Token, Detokenize, InvalidOutputType, Tokenizable},
        types::*,
    };
    use ::ethers::contract::{
        Contract, builders::{ContractCall, Event},
        Lazy,
    };
    use ::ethers::providers::Middleware;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"command_index\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"message\",\"type\":\"string\",\"components\":[]}],\"type\":\"error\",\"name\":\"ExecutionFailed\",\"outputs\":[]}]";
    /// The parsed JSON-ABI of the contract.
    pub static VM_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi"));
    pub struct VM<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for VM<M> {
        fn clone(&self) -> Self {
            VM(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for VM<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for VM<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(VM)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> VM<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ::ethers::contract::Contract::new(address.into(), VM_ABI.clone(), client)
                .into()
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for VM<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    ///Custom Error type `ExecutionFailed` with signature `ExecutionFailed(uint256,address,string)` and selector `0xef3dcb2f`
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
    )]
    #[etherror(
        name = "ExecutionFailed",
        abi = "ExecutionFailed(uint256,address,string)"
    )]
    pub struct ExecutionFailed {
        pub command_index: ::ethers::core::types::U256,
        pub target: ::ethers::core::types::Address,
        pub message: String,
    }
}
