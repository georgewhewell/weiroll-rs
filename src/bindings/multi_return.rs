pub use multi_return::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod multi_return {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    ///MultiReturn was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
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
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"j\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Calculated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"intTuple\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"arg\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"tupleConsumer\",\"outputs\":[]}]";
    /// The parsed JSON-ABI of the contract.
    pub static MULTIRETURN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi"));
    /// Bytecode of the #name contract
    pub static MULTIRETURN_BYTECODE: ::ethers::contract::Lazy<
        ::ethers::core::types::Bytes,
    > = ::ethers::contract::Lazy::new(|| {
        "0x608060405234801561001057600080fd5b5060f08061001f6000396000f3fe6080604052348015600f57600080fd5b506004361060325760003560e01c806338271fbf1460375780635bd54e42146048575b600080fd5b6046604236600460a2565b606c565b005b60408051610bad815261deed602082015261cafe8183015290519081900360600190f35b6040518181527f2c257d436c72ddfcf6860baf0bde6e7deca235585f3cd5a7e1da579a04c98ab19060200160405180910390a150565b60006020828403121560b357600080fd5b503591905056fea2646970667358221220c3fcbbf02d3e3e7125e1d33ec9c90b6d7b5a7f018ecfe2775c0eed10bf97a9bf64736f6c63430008110033"
            .parse()
            .expect("invalid bytecode")
    });
    pub struct MultiReturn<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for MultiReturn<M> {
        fn clone(&self) -> Self {
            MultiReturn(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MultiReturn<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for MultiReturn<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MultiReturn)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MultiReturn<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ::ethers::contract::Contract::new(
                    address.into(),
                    MULTIRETURN_ABI.clone(),
                    client,
                )
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
                MULTIRETURN_ABI.clone(),
                MULTIRETURN_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `intTuple` (0x5bd54e42) function
        pub fn int_tuple(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([91, 213, 78, 66], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tupleConsumer` (0x38271fbf) function
        pub fn tuple_consumer(
            &self,
            arg: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 39, 31, 191], arg)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Calculated` event
        pub fn calculated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<M, CalculatedFilter> {
            self.0.event()
        }
        /// Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<M, CalculatedFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MultiReturn<M> {
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
    #[ethevent(name = "Calculated", abi = "Calculated(uint256)")]
    pub struct CalculatedFilter {
        pub j: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `intTuple` function with signature `intTuple()` and selector `0x5bd54e42`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "intTuple", abi = "intTuple()")]
    pub struct IntTupleCall;
    ///Container type for all input parameters for the `tupleConsumer` function with signature `tupleConsumer(uint256)` and selector `0x38271fbf`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "tupleConsumer", abi = "tupleConsumer(uint256)")]
    pub struct TupleConsumerCall {
        pub arg: ::ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum MultiReturnCalls {
        IntTuple(IntTupleCall),
        TupleConsumer(TupleConsumerCall),
    }
    impl ::ethers::core::abi::AbiDecode for MultiReturnCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded)
                = <IntTupleCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MultiReturnCalls::IntTuple(decoded));
            }
            if let Ok(decoded)
                = <TupleConsumerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(MultiReturnCalls::TupleConsumer(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MultiReturnCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MultiReturnCalls::IntTuple(element) => element.encode(),
                MultiReturnCalls::TupleConsumer(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MultiReturnCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MultiReturnCalls::IntTuple(element) => element.fmt(f),
                MultiReturnCalls::TupleConsumer(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<IntTupleCall> for MultiReturnCalls {
        fn from(var: IntTupleCall) -> Self {
            MultiReturnCalls::IntTuple(var)
        }
    }
    impl ::std::convert::From<TupleConsumerCall> for MultiReturnCalls {
        fn from(var: TupleConsumerCall) -> Self {
            MultiReturnCalls::TupleConsumer(var)
        }
    }
    ///Container type for all return fields from the `intTuple` function with signature `intTuple()` and selector `0x5bd54e42`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct IntTupleReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
}
