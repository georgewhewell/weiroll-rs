pub use payable::*;
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
pub mod payable {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"fallback\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"pay\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static PAYABLE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
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
        132,
        128,
        97,
        0,
        30,
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
        96,
        4,
        54,
        16,
        96,
        39,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        27,
        146,
        101,
        184,
        20,
        96,
        45,
        87,
        128,
        99,
        182,
        158,
        248,
        168,
        20,
        96,
        47,
        87,
        0,
        91,
        54,
        96,
        45,
        87,
        0,
        91,
        0,
        91,
        52,
        128,
        21,
        96,
        58,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        71,
        96,
        64,
        81,
        144,
        129,
        82,
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
        243,
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
        181,
        87,
        234,
        175,
        63,
        120,
        176,
        202,
        123,
        200,
        224,
        242,
        57,
        170,
        149,
        129,
        164,
        162,
        161,
        108,
        78,
        44,
        179,
        54,
        0,
        120,
        100,
        32,
        109,
        108,
        153,
        1,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        18,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static PAYABLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        96,
        4,
        54,
        16,
        96,
        39,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        27,
        146,
        101,
        184,
        20,
        96,
        45,
        87,
        128,
        99,
        182,
        158,
        248,
        168,
        20,
        96,
        47,
        87,
        0,
        91,
        54,
        96,
        45,
        87,
        0,
        91,
        0,
        91,
        52,
        128,
        21,
        96,
        58,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        71,
        96,
        64,
        81,
        144,
        129,
        82,
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
        243,
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
        181,
        87,
        234,
        175,
        63,
        120,
        176,
        202,
        123,
        200,
        224,
        242,
        57,
        170,
        149,
        129,
        164,
        162,
        161,
        108,
        78,
        44,
        179,
        54,
        0,
        120,
        100,
        32,
        109,
        108,
        153,
        1,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        18,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static PAYABLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Payable<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Payable<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Payable<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Payable<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Payable<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(Payable)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Payable<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    PAYABLE_ABI.clone(),
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
                PAYABLE_ABI.clone(),
                PAYABLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `balance` (0xb69ef8a8) function
        pub fn balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([182, 158, 248, 168], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pay` (0x1b9265b8) function
        pub fn pay(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([27, 146, 101, 184], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Payable<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `balance` function with signature `balance()` and selector `0xb69ef8a8`
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
    #[ethcall(name = "balance", abi = "balance()")]
    pub struct BalanceCall;
    ///Container type for all input parameters for the `pay` function with signature `pay()` and selector `0x1b9265b8`
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
    #[ethcall(name = "pay", abi = "pay()")]
    pub struct PayCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PayableCalls {
        Balance(BalanceCall),
        Pay(PayCall),
    }
    impl ::ethers::core::abi::AbiDecode for PayableCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <BalanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Balance(decoded));
            }
            if let Ok(decoded)
                = <PayCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pay(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PayableCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Balance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pay(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PayableCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Balance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pay(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BalanceCall> for PayableCalls {
        fn from(value: BalanceCall) -> Self {
            Self::Balance(value)
        }
    }
    impl ::core::convert::From<PayCall> for PayableCalls {
        fn from(value: PayCall) -> Self {
            Self::Pay(value)
        }
    }
    ///Container type for all return fields from the `balance` function with signature `balance()` and selector `0xb69ef8a8`
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
    pub struct BalanceReturn(pub ::ethers::core::types::U256);
}
