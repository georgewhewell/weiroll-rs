pub use strings::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod strings {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    ///Strings was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
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
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"string\",\"name\":\"a\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"b\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"strcat\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"x\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"strlen\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    /// The parsed JSON-ABI of the contract.
    pub static STRINGS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi"));
    /// Bytecode of the #name contract
    pub static STRINGS_BYTECODE: ::ethers::contract::Lazy<
        ::ethers::core::types::Bytes,
    > = ::ethers::contract::Lazy::new(|| {
        "0x608060405234801561001057600080fd5b50610250806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c8063367bbd781461003b578063d824ccf314610061575b600080fd5b61004e6100493660046100fc565b919050565b6040519081526020015b60405180910390f35b61007461006f36600461013e565b610081565b60405161005891906101aa565b60608484848460405160200161009a94939291906101f8565b6040516020818303038152906040529050949350505050565b60008083601f8401126100c557600080fd5b50813567ffffffffffffffff8111156100dd57600080fd5b6020830191508360208285010111156100f557600080fd5b9250929050565b6000806020838503121561010f57600080fd5b823567ffffffffffffffff81111561012657600080fd5b610132858286016100b3565b90969095509350505050565b6000806000806040858703121561015457600080fd5b843567ffffffffffffffff8082111561016c57600080fd5b610178888389016100b3565b9096509450602087013591508082111561019157600080fd5b5061019e878288016100b3565b95989497509550505050565b600060208083528351808285015260005b818110156101d7578581018301518582016040015282016101bb565b506000604082860101526040601f19601f8301168501019250505092915050565b838582376000848201600081528385823760009301928352509094935050505056fea264697066735822122002511d97f2cc959c7d6a4c2db3f57266061a5875c2e8c353cf7bdc9c39dfc17664736f6c63430008110033"
            .parse()
            .expect("invalid bytecode")
    });
    pub struct Strings<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for Strings<M> {
        fn clone(&self) -> Self {
            Strings(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Strings<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Strings<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Strings)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Strings<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ::ethers::contract::Contract::new(
                    address.into(),
                    STRINGS_ABI.clone(),
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
                STRINGS_ABI.clone(),
                STRINGS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `strcat` (0xd824ccf3) function
        pub fn strcat(
            &self,
            a: String,
            b: String,
        ) -> ::ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([216, 36, 204, 243], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `strlen` (0x367bbd78) function
        pub fn strlen(
            &self,
            x: String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([54, 123, 189, 120], x)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Strings<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    ///Container type for all input parameters for the `strcat` function with signature `strcat(string,string)` and selector `0xd824ccf3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "strcat", abi = "strcat(string,string)")]
    pub struct StrcatCall {
        pub a: String,
        pub b: String,
    }
    ///Container type for all input parameters for the `strlen` function with signature `strlen(string)` and selector `0x367bbd78`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "strlen", abi = "strlen(string)")]
    pub struct StrlenCall {
        pub x: String,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum StringsCalls {
        Strcat(StrcatCall),
        Strlen(StrlenCall),
    }
    impl ::ethers::core::abi::AbiDecode for StringsCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded)
                = <StrcatCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(StringsCalls::Strcat(decoded));
            }
            if let Ok(decoded)
                = <StrlenCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(StringsCalls::Strlen(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for StringsCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                StringsCalls::Strcat(element) => element.encode(),
                StringsCalls::Strlen(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for StringsCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                StringsCalls::Strcat(element) => element.fmt(f),
                StringsCalls::Strlen(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<StrcatCall> for StringsCalls {
        fn from(var: StrcatCall) -> Self {
            StringsCalls::Strcat(var)
        }
    }
    impl ::std::convert::From<StrlenCall> for StringsCalls {
        fn from(var: StrlenCall) -> Self {
            StringsCalls::Strlen(var)
        }
    }
    ///Container type for all return fields from the `strcat` function with signature `strcat(string,string)` and selector `0xd824ccf3`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct StrcatReturn(pub String);
    ///Container type for all return fields from the `strlen` function with signature `strlen(string)` and selector `0x367bbd78`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct StrlenReturn(pub ::ethers::core::types::U256);
}
