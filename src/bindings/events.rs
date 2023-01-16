pub use events::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod events {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    ///Events was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
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
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"message\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LogAddress\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"message\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LogBytes\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"message\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LogBytes32\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"message\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LogString\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"message\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"LogUint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"message\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"logAddress\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"message\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"logBytes\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"message\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"logBytes32\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"message\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"logString\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"message\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"logUint\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"message\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"logUintPayable\",\"outputs\":[]}]";
    /// The parsed JSON-ABI of the contract.
    pub static EVENTS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi"));
    /// Bytecode of the #name contract
    pub static EVENTS_BYTECODE: ::ethers::contract::Lazy<::ethers::core::types::Bytes> = ::ethers::contract::Lazy::new(||
    {
        "0x608060405234801561001057600080fd5b50610366806100206000396000f3fe6080604052600436106100555760003560e01c80630bb563d61461005a5780632d21d6f71461007c578063360db92d1461009c5780635f91b0af146100af5780639905b744146100cf578063e17bf956146100ea575b600080fd5b34801561006657600080fd5b5061007a610075366004610260565b61010a565b005b34801561008857600080fd5b5061007a6100973660046102a2565b610147565b61007a6100aa3660046102a2565b61017d565b3480156100bb57600080fd5b5061007a6100ca3660046102bb565b6101ad565b3480156100db57600080fd5b5061007a6100aa3660046102a2565b3480156100f657600080fd5b5061007a610105366004610260565b6101e6565b7fa95e6e2a182411e7a6f9ed114a85c3761d87f9b8f453d842c71235aa64fff99f828260405161013b929190610314565b60405180910390a15050565b6040518181527e9fd52f05c0ded31d6fb0ee580b923f85e99cf1a5a1da342f25e73c45829c83906020015b60405180910390a150565b6040518181527f0ac68d08c5119b8cdb4058edbf0d4168f208ec3935d26a8f1f0d92eb9d4de8bf90602001610172565b6040516001600160a01b03821681527fb123f68b8ba02b447d91a6629e121111b7dd6061ff418a60139c8bf00522a28490602001610172565b7f532fd6ea96cfb78bb46e09279a26828b8b493de1a2b8b1ee1face527978a15a5828260405161013b929190610314565b60008083601f84011261022957600080fd5b50813567ffffffffffffffff81111561024157600080fd5b60208301915083602082850101111561025957600080fd5b9250929050565b6000806020838503121561027357600080fd5b823567ffffffffffffffff81111561028a57600080fd5b61029685828601610217565b90969095509350505050565b6000602082840312156102b457600080fd5b5035919050565b6000602082840312156102cd57600080fd5b81356001600160a01b03811681146102e457600080fd5b9392505050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6020815260006103286020830184866102eb565b94935050505056fea2646970667358221220c95064621a60f0a9bd9dc5d66a3dfbe0254e42f8184789106758eba2740791db64736f6c63430008110033"
            .parse()
            .expect("invalid bytecode")
    });
    pub struct Events<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for Events<M> {
        fn clone(&self) -> Self {
            Events(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Events<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Events<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Events)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Events<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ::ethers::contract::Contract::new(address.into(), EVENTS_ABI.clone(), client)
                .into()
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// 1. If there are no constructor arguments, you should pass `()` as the argument.
        /// 1. The default poll duration is 7 seconds.
        /// 1. The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter,"../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                EVENTS_ABI.clone(),
                EVENTS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `logAddress` (0x5f91b0af) function
        pub fn log_address(
            &self,
            message: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 145, 176, 175], message)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes` (0xe17bf956) function
        pub fn log_bytes(
            &self,
            message: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 123, 249, 86], message)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes32` (0x2d21d6f7) function
        pub fn log_bytes_32(
            &self,
            message: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 33, 214, 247], message)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logString` (0x0bb563d6) function
        pub fn log_string(
            &self,
            message: String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 181, 99, 214], message)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logUint` (0x9905b744) function
        pub fn log_uint(
            &self,
            message: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 5, 183, 68], message)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logUintPayable` (0x360db92d) function
        pub fn log_uint_payable(
            &self,
            message: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 13, 185, 45], message)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `LogAddress` event
        pub fn log_address_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, LogAddressFilter> {
            self.0.event()
        }
        ///Gets the contract's `LogBytes` event
        pub fn log_bytes_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, LogBytesFilter> {
            self.0.event()
        }
        ///Gets the contract's `LogBytes32` event
        pub fn log_bytes_32_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, LogBytes32Filter> {
            self.0.event()
        }
        ///Gets the contract's `LogString` event
        pub fn log_string_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, LogStringFilter> {
            self.0.event()
        }
        ///Gets the contract's `LogUint` event
        pub fn log_uint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, LogUintFilter> {
            self.0.event()
        }
        /// Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract
        pub fn events(&self) -> ::ethers::contract::builders::Event<M, EventsEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Events<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(name = "LogAddress", abi = "LogAddress(address)")]
    pub struct LogAddressFilter {
        pub message: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(name = "LogBytes", abi = "LogBytes(bytes)")]
    pub struct LogBytesFilter {
        pub message: ::ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(name = "LogBytes32", abi = "LogBytes32(bytes32)")]
    pub struct LogBytes32Filter {
        pub message: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(name = "LogString", abi = "LogString(string)")]
    pub struct LogStringFilter {
        pub message: String,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethevent(name = "LogUint", abi = "LogUint(uint256)")]
    pub struct LogUintFilter {
        pub message: ::ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum EventsEvents {
        LogAddressFilter(LogAddressFilter),
        LogBytesFilter(LogBytesFilter),
        LogBytes32Filter(LogBytes32Filter),
        LogStringFilter(LogStringFilter),
        LogUintFilter(LogUintFilter),
    }
    impl ::ethers::contract::EthLogDecode for EventsEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = LogAddressFilter::decode_log(log) {
                return Ok(EventsEvents::LogAddressFilter(decoded));
            }
            if let Ok(decoded) = LogBytesFilter::decode_log(log) {
                return Ok(EventsEvents::LogBytesFilter(decoded));
            }
            if let Ok(decoded) = LogBytes32Filter::decode_log(log) {
                return Ok(EventsEvents::LogBytes32Filter(decoded));
            }
            if let Ok(decoded) = LogStringFilter::decode_log(log) {
                return Ok(EventsEvents::LogStringFilter(decoded));
            }
            if let Ok(decoded) = LogUintFilter::decode_log(log) {
                return Ok(EventsEvents::LogUintFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for EventsEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                EventsEvents::LogAddressFilter(element) => element.fmt(f),
                EventsEvents::LogBytesFilter(element) => element.fmt(f),
                EventsEvents::LogBytes32Filter(element) => element.fmt(f),
                EventsEvents::LogStringFilter(element) => element.fmt(f),
                EventsEvents::LogUintFilter(element) => element.fmt(f),
            }
        }
    }
    ///Container type for all input parameters for the `logAddress` function with signature `logAddress(address)` and selector `0x5f91b0af`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "logAddress", abi = "logAddress(address)")]
    pub struct LogAddressCall {
        pub message: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `logBytes` function with signature `logBytes(bytes)` and selector `0xe17bf956`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "logBytes", abi = "logBytes(bytes)")]
    pub struct LogBytesCall {
        pub message: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `logBytes32` function with signature `logBytes32(bytes32)` and selector `0x2d21d6f7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "logBytes32", abi = "logBytes32(bytes32)")]
    pub struct LogBytes32Call {
        pub message: [u8; 32],
    }
    ///Container type for all input parameters for the `logString` function with signature `logString(string)` and selector `0x0bb563d6`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "logString", abi = "logString(string)")]
    pub struct LogStringCall {
        pub message: String,
    }
    ///Container type for all input parameters for the `logUint` function with signature `logUint(uint256)` and selector `0x9905b744`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "logUint", abi = "logUint(uint256)")]
    pub struct LogUintCall {
        pub message: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `logUintPayable` function with signature `logUintPayable(uint256)` and selector `0x360db92d`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "logUintPayable", abi = "logUintPayable(uint256)")]
    pub struct LogUintPayableCall {
        pub message: ::ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum EventsCalls {
        LogAddress(LogAddressCall),
        LogBytes(LogBytesCall),
        LogBytes32(LogBytes32Call),
        LogString(LogStringCall),
        LogUint(LogUintCall),
        LogUintPayable(LogUintPayableCall),
    }
    impl ::ethers::core::abi::AbiDecode for EventsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded)
                = <LogAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(EventsCalls::LogAddress(decoded));
            }
            if let Ok(decoded)
                = <LogBytesCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(EventsCalls::LogBytes(decoded));
            }
            if let Ok(decoded)
                = <LogBytes32Call as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(EventsCalls::LogBytes32(decoded));
            }
            if let Ok(decoded)
                = <LogStringCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(EventsCalls::LogString(decoded));
            }
            if let Ok(decoded)
                = <LogUintCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(EventsCalls::LogUint(decoded));
            }
            if let Ok(decoded)
                = <LogUintPayableCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(EventsCalls::LogUintPayable(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for EventsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                EventsCalls::LogAddress(element) => element.encode(),
                EventsCalls::LogBytes(element) => element.encode(),
                EventsCalls::LogBytes32(element) => element.encode(),
                EventsCalls::LogString(element) => element.encode(),
                EventsCalls::LogUint(element) => element.encode(),
                EventsCalls::LogUintPayable(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for EventsCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                EventsCalls::LogAddress(element) => element.fmt(f),
                EventsCalls::LogBytes(element) => element.fmt(f),
                EventsCalls::LogBytes32(element) => element.fmt(f),
                EventsCalls::LogString(element) => element.fmt(f),
                EventsCalls::LogUint(element) => element.fmt(f),
                EventsCalls::LogUintPayable(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<LogAddressCall> for EventsCalls {
        fn from(var: LogAddressCall) -> Self {
            EventsCalls::LogAddress(var)
        }
    }
    impl ::std::convert::From<LogBytesCall> for EventsCalls {
        fn from(var: LogBytesCall) -> Self {
            EventsCalls::LogBytes(var)
        }
    }
    impl ::std::convert::From<LogBytes32Call> for EventsCalls {
        fn from(var: LogBytes32Call) -> Self {
            EventsCalls::LogBytes32(var)
        }
    }
    impl ::std::convert::From<LogStringCall> for EventsCalls {
        fn from(var: LogStringCall) -> Self {
            EventsCalls::LogString(var)
        }
    }
    impl ::std::convert::From<LogUintCall> for EventsCalls {
        fn from(var: LogUintCall) -> Self {
            EventsCalls::LogUint(var)
        }
    }
    impl ::std::convert::From<LogUintPayableCall> for EventsCalls {
        fn from(var: LogUintPayableCall) -> Self {
            EventsCalls::LogUintPayable(var)
        }
    }
}
