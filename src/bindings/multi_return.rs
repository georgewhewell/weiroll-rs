pub use multi_return::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod multi_return {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"j\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Calculated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"intTuple\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"arg\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"tupleConsumer\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static MULTIRETURN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        240,
        128,
        97,
        0,
        31,
        96,
        0,
        57,
        96,
        0,
        243,
        254,
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        96,
        15,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        96,
        50,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        56,
        39,
        31,
        191,
        20,
        96,
        55,
        87,
        128,
        99,
        91,
        213,
        78,
        66,
        20,
        96,
        72,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        96,
        70,
        96,
        66,
        54,
        96,
        4,
        96,
        162,
        86,
        91,
        96,
        108,
        86,
        91,
        0,
        91,
        96,
        64,
        128,
        81,
        97,
        11,
        173,
        129,
        82,
        97,
        222,
        237,
        96,
        32,
        130,
        1,
        82,
        97,
        202,
        254,
        129,
        131,
        1,
        82,
        144,
        81,
        144,
        129,
        144,
        3,
        96,
        96,
        1,
        144,
        243,
        91,
        96,
        64,
        81,
        129,
        129,
        82,
        127,
        44,
        37,
        125,
        67,
        108,
        114,
        221,
        252,
        246,
        134,
        11,
        175,
        11,
        222,
        110,
        125,
        236,
        162,
        53,
        88,
        95,
        60,
        213,
        167,
        225,
        218,
        87,
        154,
        4,
        201,
        138,
        177,
        144,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        161,
        80,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        96,
        179,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        53,
        145,
        144,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        195,
        252,
        187,
        240,
        45,
        62,
        62,
        113,
        37,
        225,
        211,
        62,
        201,
        201,
        11,
        109,
        123,
        90,
        127,
        1,
        142,
        207,
        226,
        119,
        92,
        14,
        237,
        16,
        191,
        151,
        169,
        191,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        17,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static MULTIRETURN_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        96,
        15,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        96,
        50,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        56,
        39,
        31,
        191,
        20,
        96,
        55,
        87,
        128,
        99,
        91,
        213,
        78,
        66,
        20,
        96,
        72,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        96,
        70,
        96,
        66,
        54,
        96,
        4,
        96,
        162,
        86,
        91,
        96,
        108,
        86,
        91,
        0,
        91,
        96,
        64,
        128,
        81,
        97,
        11,
        173,
        129,
        82,
        97,
        222,
        237,
        96,
        32,
        130,
        1,
        82,
        97,
        202,
        254,
        129,
        131,
        1,
        82,
        144,
        81,
        144,
        129,
        144,
        3,
        96,
        96,
        1,
        144,
        243,
        91,
        96,
        64,
        81,
        129,
        129,
        82,
        127,
        44,
        37,
        125,
        67,
        108,
        114,
        221,
        252,
        246,
        134,
        11,
        175,
        11,
        222,
        110,
        125,
        236,
        162,
        53,
        88,
        95,
        60,
        213,
        167,
        225,
        218,
        87,
        154,
        4,
        201,
        138,
        177,
        144,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        161,
        80,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        96,
        179,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        53,
        145,
        144,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        195,
        252,
        187,
        240,
        45,
        62,
        62,
        113,
        37,
        225,
        211,
        62,
        201,
        201,
        11,
        109,
        123,
        90,
        127,
        1,
        142,
        207,
        226,
        119,
        92,
        14,
        237,
        16,
        191,
        151,
        169,
        191,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        17,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static MULTIRETURN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MultiReturn<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MultiReturn<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MultiReturn<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MultiReturn<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MultiReturn<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(MultiReturn)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MultiReturn<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MULTIRETURN_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
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
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
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
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CalculatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CalculatedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MultiReturn<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Calculated", abi = "Calculated(uint256)")]
    pub struct CalculatedFilter {
        pub j: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `intTuple` function with signature `intTuple()` and selector `0x5bd54e42`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "intTuple", abi = "intTuple()")]
    pub struct IntTupleCall;
    ///Container type for all input parameters for the `tupleConsumer` function with signature `tupleConsumer(uint256)` and selector `0x38271fbf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "tupleConsumer", abi = "tupleConsumer(uint256)")]
    pub struct TupleConsumerCall {
        pub arg: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MultiReturnCalls {
        IntTuple(IntTupleCall),
        TupleConsumer(TupleConsumerCall),
    }
    impl ::ethers::core::abi::AbiDecode for MultiReturnCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <IntTupleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IntTuple(decoded));
            }
            if let Ok(decoded)
                = <TupleConsumerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TupleConsumer(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MultiReturnCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::IntTuple(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TupleConsumer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MultiReturnCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::IntTuple(element) => ::core::fmt::Display::fmt(element, f),
                Self::TupleConsumer(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<IntTupleCall> for MultiReturnCalls {
        fn from(value: IntTupleCall) -> Self {
            Self::IntTuple(value)
        }
    }
    impl ::core::convert::From<TupleConsumerCall> for MultiReturnCalls {
        fn from(value: TupleConsumerCall) -> Self {
            Self::TupleConsumer(value)
        }
    }
    ///Container type for all return fields from the `intTuple` function with signature `intTuple()` and selector `0x5bd54e42`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IntTupleReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
}
