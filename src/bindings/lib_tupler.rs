pub use lib_tupler::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod lib_tupler {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"tuple\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"extractElement\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static LIBTUPLER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
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
        97,
        1,
        100,
        128,
        97,
        0,
        32,
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
        4,
        54,
        16,
        97,
        0,
        43,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        155,
        211,
        178,
        39,
        20,
        97,
        0,
        48,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        67,
        97,
        0,
        62,
        54,
        96,
        4,
        97,
        0,
        121,
        86,
        91,
        97,
        0,
        85,
        86,
        91,
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
        91,
        96,
        0,
        96,
        32,
        128,
        96,
        1,
        132,
        1,
        2,
        132,
        1,
        243,
        91,
        99,
        78,
        72,
        123,
        113,
        96,
        224,
        27,
        96,
        0,
        82,
        96,
        65,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        96,
        0,
        128,
        96,
        64,
        131,
        133,
        3,
        18,
        21,
        97,
        0,
        140,
        87,
        96,
        0,
        128,
        253,
        91,
        130,
        53,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        128,
        130,
        17,
        21,
        97,
        0,
        164,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        133,
        1,
        145,
        80,
        133,
        96,
        31,
        131,
        1,
        18,
        97,
        0,
        184,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        129,
        129,
        17,
        21,
        97,
        0,
        202,
        87,
        97,
        0,
        202,
        97,
        0,
        99,
        86,
        91,
        96,
        64,
        81,
        96,
        31,
        130,
        1,
        96,
        31,
        25,
        144,
        129,
        22,
        96,
        63,
        1,
        22,
        129,
        1,
        144,
        131,
        130,
        17,
        129,
        131,
        16,
        23,
        21,
        97,
        0,
        242,
        87,
        97,
        0,
        242,
        97,
        0,
        99,
        86,
        91,
        129,
        96,
        64,
        82,
        130,
        129,
        82,
        136,
        96,
        32,
        132,
        135,
        1,
        1,
        17,
        21,
        97,
        1,
        11,
        87,
        96,
        0,
        128,
        253,
        91,
        130,
        96,
        32,
        134,
        1,
        96,
        32,
        131,
        1,
        55,
        96,
        0,
        96,
        32,
        147,
        130,
        1,
        132,
        1,
        82,
        152,
        150,
        144,
        145,
        1,
        53,
        150,
        80,
        80,
        80,
        80,
        80,
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
        134,
        177,
        176,
        176,
        235,
        195,
        84,
        149,
        155,
        167,
        208,
        201,
        245,
        47,
        221,
        139,
        34,
        226,
        176,
        38,
        153,
        127,
        195,
        150,
        134,
        66,
        98,
        202,
        53,
        76,
        78,
        208,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        19,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static LIBTUPLER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
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
        4,
        54,
        16,
        97,
        0,
        43,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        155,
        211,
        178,
        39,
        20,
        97,
        0,
        48,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        67,
        97,
        0,
        62,
        54,
        96,
        4,
        97,
        0,
        121,
        86,
        91,
        97,
        0,
        85,
        86,
        91,
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
        91,
        96,
        0,
        96,
        32,
        128,
        96,
        1,
        132,
        1,
        2,
        132,
        1,
        243,
        91,
        99,
        78,
        72,
        123,
        113,
        96,
        224,
        27,
        96,
        0,
        82,
        96,
        65,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        96,
        0,
        128,
        96,
        64,
        131,
        133,
        3,
        18,
        21,
        97,
        0,
        140,
        87,
        96,
        0,
        128,
        253,
        91,
        130,
        53,
        103,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        255,
        128,
        130,
        17,
        21,
        97,
        0,
        164,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        133,
        1,
        145,
        80,
        133,
        96,
        31,
        131,
        1,
        18,
        97,
        0,
        184,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        53,
        129,
        129,
        17,
        21,
        97,
        0,
        202,
        87,
        97,
        0,
        202,
        97,
        0,
        99,
        86,
        91,
        96,
        64,
        81,
        96,
        31,
        130,
        1,
        96,
        31,
        25,
        144,
        129,
        22,
        96,
        63,
        1,
        22,
        129,
        1,
        144,
        131,
        130,
        17,
        129,
        131,
        16,
        23,
        21,
        97,
        0,
        242,
        87,
        97,
        0,
        242,
        97,
        0,
        99,
        86,
        91,
        129,
        96,
        64,
        82,
        130,
        129,
        82,
        136,
        96,
        32,
        132,
        135,
        1,
        1,
        17,
        21,
        97,
        1,
        11,
        87,
        96,
        0,
        128,
        253,
        91,
        130,
        96,
        32,
        134,
        1,
        96,
        32,
        131,
        1,
        55,
        96,
        0,
        96,
        32,
        147,
        130,
        1,
        132,
        1,
        82,
        152,
        150,
        144,
        145,
        1,
        53,
        150,
        80,
        80,
        80,
        80,
        80,
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
        134,
        177,
        176,
        176,
        235,
        195,
        84,
        149,
        155,
        167,
        208,
        201,
        245,
        47,
        221,
        139,
        34,
        226,
        176,
        38,
        153,
        127,
        195,
        150,
        134,
        66,
        98,
        202,
        53,
        76,
        78,
        208,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        19,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static LIBTUPLER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct LibTupler<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LibTupler<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LibTupler<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LibTupler<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LibTupler<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(LibTupler))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LibTupler<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                LIBTUPLER_ABI.clone(),
                client,
            ))
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
                LIBTUPLER_ABI.clone(),
                LIBTUPLER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `extractElement` (0x9bd3b227) function
        pub fn extract_element(
            &self,
            tuple: ::ethers::core::types::Bytes,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([155, 211, 178, 39], (tuple, index))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for LibTupler<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `extractElement` function with signature `extractElement(bytes,uint256)` and selector `0x9bd3b227`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "extractElement", abi = "extractElement(bytes,uint256)")]
    pub struct ExtractElementCall {
        pub tuple: ::ethers::core::types::Bytes,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `extractElement` function with signature `extractElement(bytes,uint256)` and selector `0x9bd3b227`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ExtractElementReturn(pub [u8; 32]);
}
