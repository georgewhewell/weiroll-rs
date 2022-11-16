pub use payable::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod payable {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "Payable was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"fallback\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"pay\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"payable\",\"type\":\"receive\",\"outputs\":[]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static PAYABLE_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static PAYABLE_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x6080604052348015600f57600080fd5b5060848061001e6000396000f3fe60806040526004361060275760003560e01c80631b9265b814602d578063b69ef8a814602f57005b36602d57005b005b348015603a57600080fd5b504760405190815260200160405180910390f3fea2646970667358221220b71354511eca8117c3f49b1f3df9dba10179caf609c07148996d95598bd2ebc864736f6c634300080f0033" . parse () . expect ("invalid bytecode")
        });
    pub struct Payable<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Payable<M> {
        fn clone(&self) -> Self {
            Payable(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Payable<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Payable<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Payable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Payable<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), PAYABLE_ABI.clone(), client).into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::std::result::Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                PAYABLE_ABI.clone(),
                PAYABLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `balance` (0xb69ef8a8) function"]
        pub fn balance(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([182, 158, 248, 168], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pay` (0x1b9265b8) function"]
        pub fn pay(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([27, 146, 101, 184], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Payable<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Container type for all input parameters for the `balance` function with signature `balance()` and selector `[182, 158, 248, 168]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "balance", abi = "balance()")]
    pub struct BalanceCall;
    #[doc = "Container type for all input parameters for the `pay` function with signature `pay()` and selector `[27, 146, 101, 184]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "pay", abi = "pay()")]
    pub struct PayCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PayableCalls {
        Balance(BalanceCall),
        Pay(PayCall),
    }
    impl ethers::core::abi::AbiDecode for PayableCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BalanceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PayableCalls::Balance(decoded));
            }
            if let Ok(decoded) = <PayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(PayableCalls::Pay(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PayableCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PayableCalls::Balance(element) => element.encode(),
                PayableCalls::Pay(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PayableCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PayableCalls::Balance(element) => element.fmt(f),
                PayableCalls::Pay(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BalanceCall> for PayableCalls {
        fn from(var: BalanceCall) -> Self {
            PayableCalls::Balance(var)
        }
    }
    impl ::std::convert::From<PayCall> for PayableCalls {
        fn from(var: PayCall) -> Self {
            PayableCalls::Pay(var)
        }
    }
    #[doc = "Container type for all return fields from the `balance` function with signature `balance()` and selector `[182, 158, 248, 168]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BalanceReturn(pub ethers::core::types::U256);
}
