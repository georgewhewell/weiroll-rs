pub use lib_tupler::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod lib_tupler {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    ///LibTupler was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
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
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"tuple\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"extractElement\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]}]";
    /// The parsed JSON-ABI of the contract.
    pub static LIBTUPLER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi"));
    /// Bytecode of the #name contract
    pub static LIBTUPLER_BYTECODE: ::ethers::contract::Lazy<
        ::ethers::core::types::Bytes,
    > = ::ethers::contract::Lazy::new(|| {
        "0x608060405234801561001057600080fd5b50610164806100206000396000f3fe608060405234801561001057600080fd5b506004361061002b5760003560e01c80639bd3b22714610030575b600080fd5b61004361003e366004610079565b610055565b60405190815260200160405180910390f35b600060208060018401028401f35b634e487b7160e01b600052604160045260246000fd5b6000806040838503121561008c57600080fd5b823567ffffffffffffffff808211156100a457600080fd5b818501915085601f8301126100b857600080fd5b8135818111156100ca576100ca610063565b604051601f8201601f19908116603f011681019083821181831017156100f2576100f2610063565b8160405282815288602084870101111561010b57600080fd5b82602086016020830137600060209382018401529896909101359650505050505056fea2646970667358221220a05eb904307d2a64777e17772c2ab1a7a0ec9628f7222fc8a1e734d38e42389764736f6c63430008110033"
            .parse()
            .expect("invalid bytecode")
    });
    pub struct LibTupler<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for LibTupler<M> {
        fn clone(&self) -> Self {
            LibTupler(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for LibTupler<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for LibTupler<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(LibTupler)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LibTupler<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ::ethers::contract::Contract::new(
                    address.into(),
                    LIBTUPLER_ABI.clone(),
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
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LibTupler<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    ///Container type for all input parameters for the `extractElement` function with signature `extractElement(bytes,uint256)` and selector `0x9bd3b227`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "extractElement", abi = "extractElement(bytes,uint256)")]
    pub struct ExtractElementCall {
        pub tuple: ::ethers::core::types::Bytes,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `extractElement` function with signature `extractElement(bytes,uint256)` and selector `0x9bd3b227`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct ExtractElementReturn(pub [u8; 32]);
}
