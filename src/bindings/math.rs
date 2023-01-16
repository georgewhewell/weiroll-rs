pub use math::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod math {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    ///Math was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
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
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"a\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"b\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"add\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"a\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"b\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"mul\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"a\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"b\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"sub\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"values\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"sum\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"ret\",\"type\":\"uint256\",\"components\":[]}]}]";
    /// The parsed JSON-ABI of the contract.
    pub static MATH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi"));
    /// Bytecode of the #name contract
    pub static MATH_BYTECODE: ::ethers::contract::Lazy<::ethers::core::types::Bytes> = ::ethers::contract::Lazy::new(||
    {
        "0x608060405234801561001057600080fd5b50610276806100206000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c80630194db8e14610051578063771602f714610076578063b67d77c514610089578063c8a4ac9c1461009c575b600080fd5b61006461005f366004610127565b6100af565b60405190815260200160405180910390f35b61006461008436600461019c565b6100fa565b61006461009736600461019c565b61010f565b6100646100aa36600461019c565b61011b565b600081815b818110156100f2578484828181106100ce576100ce6101be565b90506020020135836100e091906101ea565b92506100eb816101fd565b90506100b4565b505092915050565b600061010682846101ea565b90505b92915050565b60006101068284610216565b60006101068284610229565b6000806020838503121561013a57600080fd5b823567ffffffffffffffff8082111561015257600080fd5b818501915085601f83011261016657600080fd5b81358181111561017557600080fd5b8660208260051b850101111561018a57600080fd5b60209290920196919550909350505050565b600080604083850312156101af57600080fd5b50508035926020909101359150565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b80820180821115610109576101096101d4565b60006001820161020f5761020f6101d4565b5060010190565b81810381811115610109576101096101d4565b8082028115828204841417610109576101096101d456fea2646970667358221220695482a06d2ee197146ffd2c0b830aca02a5a520bb1d3e057b664e606ca482b864736f6c63430008110033"
            .parse()
            .expect("invalid bytecode")
    });
    pub struct Math<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for Math<M> {
        fn clone(&self) -> Self {
            Math(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Math<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for Math<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Math)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Math<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ::ethers::contract::Contract::new(address.into(), MATH_ABI.clone(), client)
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
                MATH_ABI.clone(),
                MATH_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `add` (0x771602f7) function
        pub fn add(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([119, 22, 2, 247], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mul` (0xc8a4ac9c) function
        pub fn mul(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([200, 164, 172, 156], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sub` (0xb67d77c5) function
        pub fn sub(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([182, 125, 119, 197], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sum` (0x0194db8e) function
        pub fn sum(
            &self,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([1, 148, 219, 142], values)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Math<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    ///Container type for all input parameters for the `add` function with signature `add(uint256,uint256)` and selector `0x771602f7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "add", abi = "add(uint256,uint256)")]
    pub struct AddCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `mul` function with signature `mul(uint256,uint256)` and selector `0xc8a4ac9c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "mul", abi = "mul(uint256,uint256)")]
    pub struct MulCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `sub` function with signature `sub(uint256,uint256)` and selector `0xb67d77c5`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "sub", abi = "sub(uint256,uint256)")]
    pub struct SubCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `sum` function with signature `sum(uint256[])` and selector `0x0194db8e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "sum", abi = "sum(uint256[])")]
    pub struct SumCall {
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum MathCalls {
        Add(AddCall),
        Mul(MulCall),
        Sub(SubCall),
        Sum(SumCall),
    }
    impl ::ethers::core::abi::AbiDecode for MathCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded)
                = <AddCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MathCalls::Add(decoded));
            }
            if let Ok(decoded)
                = <MulCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MathCalls::Mul(decoded));
            }
            if let Ok(decoded)
                = <SubCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MathCalls::Sub(decoded));
            }
            if let Ok(decoded)
                = <SumCall as ::ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(MathCalls::Sum(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MathCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MathCalls::Add(element) => element.encode(),
                MathCalls::Mul(element) => element.encode(),
                MathCalls::Sub(element) => element.encode(),
                MathCalls::Sum(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MathCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MathCalls::Add(element) => element.fmt(f),
                MathCalls::Mul(element) => element.fmt(f),
                MathCalls::Sub(element) => element.fmt(f),
                MathCalls::Sum(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AddCall> for MathCalls {
        fn from(var: AddCall) -> Self {
            MathCalls::Add(var)
        }
    }
    impl ::std::convert::From<MulCall> for MathCalls {
        fn from(var: MulCall) -> Self {
            MathCalls::Mul(var)
        }
    }
    impl ::std::convert::From<SubCall> for MathCalls {
        fn from(var: SubCall) -> Self {
            MathCalls::Sub(var)
        }
    }
    impl ::std::convert::From<SumCall> for MathCalls {
        fn from(var: SumCall) -> Self {
            MathCalls::Sum(var)
        }
    }
    ///Container type for all return fields from the `add` function with signature `add(uint256,uint256)` and selector `0x771602f7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct AddReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `mul` function with signature `mul(uint256,uint256)` and selector `0xc8a4ac9c`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct MulReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `sub` function with signature `sub(uint256,uint256)` and selector `0xb67d77c5`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct SubReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `sum` function with signature `sum(uint256[])` and selector `0x0194db8e`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct SumReturn {
        pub ret: ::ethers::core::types::U256,
    }
}
