pub use command_builder_harness::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod command_builder_harness {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    ///CommandBuilderHarness was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs
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
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"basecall\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes[]\",\"name\":\"state\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"bytes4\",\"name\":\"selector\",\"type\":\"bytes4\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"indices\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"testBuildInputs\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes[]\",\"name\":\"state\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"bytes4\",\"name\":\"selector\",\"type\":\"bytes4\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"indices\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"testBuildInputsBaseGas\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"out\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes[]\",\"name\":\"state\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"bytes1\",\"name\":\"index\",\"type\":\"bytes1\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"output\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"testWriteOutputs\",\"outputs\":[{\"internalType\":\"bytes[]\",\"name\":\"\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes[]\",\"name\":\"state\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"bytes1\",\"name\":\"index\",\"type\":\"bytes1\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"output\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"testWriteOutputsBaseGas\",\"outputs\":[{\"internalType\":\"bytes[]\",\"name\":\"\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]}]";
    /// The parsed JSON-ABI of the contract.
    pub static COMMANDBUILDERHARNESS_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
    });
    /// Bytecode of the #name contract
    pub static COMMANDBUILDERHARNESS_BYTECODE: ::ethers::contract::Lazy<
        ::ethers::core::types::Bytes,
    > = ::ethers::contract::Lazy::new(|| {
        "0x608060405234801561001057600080fd5b50610b16806100206000396000f3fe608060405234801561001057600080fd5b50600436106100575760003560e01c8063171f98cb1461005c5780637f180321146100865780637ff6f34f14610099578063a402609a146100b9578063edff49f7146100d0575b600080fd5b61006f61006a366004610794565b6100d2565b60405161007d9291906108be565b60405180910390f35b61006f610094366004610794565b6100eb565b6100ac6100a73660046108ec565b61011a565b60405161007d9190610953565b6100ac6100c73660046108ec565b60609392505050565b005b6060806100e0858585610134565b959294509192505050565b604080516020808252818301909252606091829186916020820181803683370190505091509150935093915050565b60606000610129858585610291565b9150505b9392505050565b606060f883901c60fe19810161014d578491505061012d565b60808116156102045760fe810361017957828060200190518101906101729190610966565b9450610288565b6020838101519081146101e65760405162461bcd60e51b815260206004820152602a60248201527f4f6e6c79206f6e652072657475726e2076616c7565207065726d697474656420604482015269287661726961626c652960b01b60648201526084015b60405180910390fd5b508251601f19016020848101918252607f8316810287010152610288565b82516020146102665760405162461bcd60e51b815260206004820152602860248201527f4f6e6c79206f6e652072657475726e2076616c7565207065726d697474656420604482015267287374617469632960c01b60648201526084016101dd565b8285607f83168151811061027c5761027c610a53565b60200260200101819052505b50929392505050565b606060008060606000805b6020811015610466578681602081106102b7576102b7610a53565b1a915060fe198201156104665760808216156103cc5760fe820361031457825160000361030157886040516020016102ef9190610a69565b60405160208183030381529060405292505b825161030d9086610a92565b9450610457565b600089607f84168151811061032b5761032b610a53565b60200260200101515190506020816103439190610aab565b156103af5760405162461bcd60e51b815260206004820152603660248201527f44796e616d6963207374617465207661726961626c6573206d7573742062652060448201527561206d756c7469706c65206f6620333220627974657360501b60648201526084016101dd565b6103ba816020610a92565b6103c49087610a92565b955050610457565b88607f8316815181106103e1576103e1610a53565b6020026020010151516020146104495760405162461bcd60e51b815260206004820152602760248201527f537461746963207374617465207661726961626c6573206d75737420626520336044820152663220627974657360c81b60648201526084016101dd565b610454602086610a92565b94505b6020939093019260010161029c565b50610472846004610a92565b67ffffffffffffffff81111561048a5761048a610620565b6040519080825280601f01601f1916602001820160405280156104b4576020820181803683370190505b5094508660208601526000935060005b60208110156105fa578681602081106104df576104df610a53565b1a915060fe198201156105fa5760808216156105bc5760fe820361054a5785850160240184905261052b83602088610518886004610a92565b602088516105269190610acd565b610606565b602083516105399190610acd565b6105439085610a92565b93506105eb565b600089607f84168151811061056157610561610a53565b602002602001015151905084866024890101526105aa8a607f85168151811061058c5761058c610a53565b60200260200101516000898860046105a49190610a92565b85610606565b6105b48186610a92565b9450506105eb565b600089607f8416815181106105d3576105d3610a53565b60200260200101519050602081015186602489010152505b602094909401936001016104c4565b50505050509392505050565b808260208501018286602089010160045afa505050505050565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f1916810167ffffffffffffffff8111828210171561065f5761065f610620565b604052919050565b600067ffffffffffffffff82111561068157610681610620565b5060051b60200190565b600067ffffffffffffffff8211156106a5576106a5610620565b50601f01601f191660200190565b600082601f8301126106c457600080fd5b81356106d76106d28261068b565b610636565b8181528460208386010111156106ec57600080fd5b816020850160208301376000918101602001919091529392505050565b600082601f83011261071a57600080fd5b8135602061072a6106d283610667565b82815260059290921b8401810191818101908684111561074957600080fd5b8286015b8481101561078957803567ffffffffffffffff81111561076d5760008081fd5b61077b8986838b01016106b3565b84525091830191830161074d565b509695505050505050565b6000806000606084860312156107a957600080fd5b833567ffffffffffffffff808211156107c157600080fd5b6107cd87838801610709565b9450602086013591506001600160f81b0319821682146107ec57600080fd5b9092506040850135908082111561080257600080fd5b5061080f868287016106b3565b9150509250925092565b60005b8381101561083457818101518382015260200161081c565b50506000910152565b60008151808452610855816020860160208601610819565b601f01601f19169290920160200192915050565b600081518084526020808501808196508360051b8101915082860160005b858110156108b157828403895261089f84835161083d565b98850198935090840190600101610887565b5091979650505050505050565b6040815260006108d16040830185610869565b82810360208401526108e3818561083d565b95945050505050565b60008060006060848603121561090157600080fd5b833567ffffffffffffffff81111561091857600080fd5b61092486828701610709565b93505060208401356001600160e01b03198116811461094257600080fd5b929592945050506040919091013590565b60208152600061012d602083018461083d565b6000602080838503121561097957600080fd5b825167ffffffffffffffff8082111561099157600080fd5b818501915085601f8301126109a557600080fd5b81516109b36106d282610667565b81815260059190911b830184019084810190888311156109d257600080fd5b8585015b83811015610a46578051858111156109ee5760008081fd5b8601603f81018b13610a005760008081fd5b878101516040610a126106d28361068b565b8281528d82848601011115610a275760008081fd5b610a36838c8301848701610819565b86525050509186019186016109d6565b5098975050505050505050565b634e487b7160e01b600052603260045260246000fd5b60208152600061012d6020830184610869565b634e487b7160e01b600052601160045260246000fd5b80820180821115610aa557610aa5610a7c565b92915050565b600082610ac857634e487b7160e01b600052601260045260246000fd5b500690565b81810381811115610aa557610aa5610a7c56fea2646970667358221220ae01c308a341bf218ee107c96205fd3d0d0059caf53ce5f5408c0998cf73ab5864736f6c63430008110033"
            .parse()
            .expect("invalid bytecode")
    });
    pub struct CommandBuilderHarness<M>(::ethers::contract::Contract<M>);
    impl<M> Clone for CommandBuilderHarness<M> {
        fn clone(&self) -> Self {
            CommandBuilderHarness(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for CommandBuilderHarness<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for CommandBuilderHarness<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(CommandBuilderHarness))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CommandBuilderHarness<M> {
        /// Creates a new contract instance with the specified `ethers`
        /// client at the given `Address`. The contract derefs to a `ethers::Contract`
        /// object
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ::ethers::contract::Contract::new(
                    address.into(),
                    COMMANDBUILDERHARNESS_ABI.clone(),
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
                COMMANDBUILDERHARNESS_ABI.clone(),
                COMMANDBUILDERHARNESS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `basecall` (0xedff49f7) function
        pub fn basecall(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 255, 73, 247], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `testBuildInputs` (0x7ff6f34f) function
        pub fn test_build_inputs(
            &self,
            state: ::std::vec::Vec<::ethers::core::types::Bytes>,
            selector: [u8; 4],
            indices: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([127, 246, 243, 79], (state, selector, indices))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `testBuildInputsBaseGas` (0xa402609a) function
        pub fn test_build_inputs_base_gas(
            &self,
            state: ::std::vec::Vec<::ethers::core::types::Bytes>,
            selector: [u8; 4],
            indices: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([164, 2, 96, 154], (state, selector, indices))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `testWriteOutputs` (0x171f98cb) function
        pub fn test_write_outputs(
            &self,
            state: ::std::vec::Vec<::ethers::core::types::Bytes>,
            index: [u8; 1],
            output: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<::ethers::core::types::Bytes>, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([23, 31, 152, 203], (state, index, output))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `testWriteOutputsBaseGas` (0x7f180321) function
        pub fn test_write_outputs_base_gas(
            &self,
            state: ::std::vec::Vec<::ethers::core::types::Bytes>,
            index: [u8; 1],
            output: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<::ethers::core::types::Bytes>, ::ethers::core::types::Bytes),
        > {
            self.0
                .method_hash([127, 24, 3, 33], (state, index, output))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CommandBuilderHarness<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    ///Container type for all input parameters for the `basecall` function with signature `basecall()` and selector `0xedff49f7`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "basecall", abi = "basecall()")]
    pub struct BasecallCall;
    ///Container type for all input parameters for the `testBuildInputs` function with signature `testBuildInputs(bytes[],bytes4,bytes32)` and selector `0x7ff6f34f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "testBuildInputs", abi = "testBuildInputs(bytes[],bytes4,bytes32)")]
    pub struct TestBuildInputsCall {
        pub state: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub selector: [u8; 4],
        pub indices: [u8; 32],
    }
    ///Container type for all input parameters for the `testBuildInputsBaseGas` function with signature `testBuildInputsBaseGas(bytes[],bytes4,bytes32)` and selector `0xa402609a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(
        name = "testBuildInputsBaseGas",
        abi = "testBuildInputsBaseGas(bytes[],bytes4,bytes32)"
    )]
    pub struct TestBuildInputsBaseGasCall {
        pub state: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub selector: [u8; 4],
        pub indices: [u8; 32],
    }
    ///Container type for all input parameters for the `testWriteOutputs` function with signature `testWriteOutputs(bytes[],bytes1,bytes)` and selector `0x171f98cb`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(name = "testWriteOutputs", abi = "testWriteOutputs(bytes[],bytes1,bytes)")]
    pub struct TestWriteOutputsCall {
        pub state: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub index: [u8; 1],
        pub output: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `testWriteOutputsBaseGas` function with signature `testWriteOutputsBaseGas(bytes[],bytes1,bytes)` and selector `0x7f180321`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
    )]
    #[derive(Default)]
    #[ethcall(
        name = "testWriteOutputsBaseGas",
        abi = "testWriteOutputsBaseGas(bytes[],bytes1,bytes)"
    )]
    pub struct TestWriteOutputsBaseGasCall {
        pub state: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub index: [u8; 1],
        pub output: ::ethers::core::types::Bytes,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ::ethers::contract::EthAbiType)]
    pub enum CommandBuilderHarnessCalls {
        Basecall(BasecallCall),
        TestBuildInputs(TestBuildInputsCall),
        TestBuildInputsBaseGas(TestBuildInputsBaseGasCall),
        TestWriteOutputs(TestWriteOutputsCall),
        TestWriteOutputsBaseGas(TestWriteOutputsBaseGasCall),
    }
    impl ::ethers::core::abi::AbiDecode for CommandBuilderHarnessCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ::ethers::core::abi::AbiError> {
            if let Ok(decoded)
                = <BasecallCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(CommandBuilderHarnessCalls::Basecall(decoded));
            }
            if let Ok(decoded)
                = <TestBuildInputsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(CommandBuilderHarnessCalls::TestBuildInputs(decoded));
            }
            if let Ok(decoded)
                = <TestBuildInputsBaseGasCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(CommandBuilderHarnessCalls::TestBuildInputsBaseGas(decoded));
            }
            if let Ok(decoded)
                = <TestWriteOutputsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(CommandBuilderHarnessCalls::TestWriteOutputs(decoded));
            }
            if let Ok(decoded)
                = <TestWriteOutputsBaseGasCall as ::ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                ) {
                return Ok(CommandBuilderHarnessCalls::TestWriteOutputsBaseGas(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CommandBuilderHarnessCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                CommandBuilderHarnessCalls::Basecall(element) => element.encode(),
                CommandBuilderHarnessCalls::TestBuildInputs(element) => element.encode(),
                CommandBuilderHarnessCalls::TestBuildInputsBaseGas(element) => {
                    element.encode()
                }
                CommandBuilderHarnessCalls::TestWriteOutputs(element) => element.encode(),
                CommandBuilderHarnessCalls::TestWriteOutputsBaseGas(element) => {
                    element.encode()
                }
            }
        }
    }
    impl ::std::fmt::Display for CommandBuilderHarnessCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                CommandBuilderHarnessCalls::Basecall(element) => element.fmt(f),
                CommandBuilderHarnessCalls::TestBuildInputs(element) => element.fmt(f),
                CommandBuilderHarnessCalls::TestBuildInputsBaseGas(element) => {
                    element.fmt(f)
                }
                CommandBuilderHarnessCalls::TestWriteOutputs(element) => element.fmt(f),
                CommandBuilderHarnessCalls::TestWriteOutputsBaseGas(element) => {
                    element.fmt(f)
                }
            }
        }
    }
    impl ::std::convert::From<BasecallCall> for CommandBuilderHarnessCalls {
        fn from(var: BasecallCall) -> Self {
            CommandBuilderHarnessCalls::Basecall(var)
        }
    }
    impl ::std::convert::From<TestBuildInputsCall> for CommandBuilderHarnessCalls {
        fn from(var: TestBuildInputsCall) -> Self {
            CommandBuilderHarnessCalls::TestBuildInputs(var)
        }
    }
    impl ::std::convert::From<TestBuildInputsBaseGasCall>
    for CommandBuilderHarnessCalls {
        fn from(var: TestBuildInputsBaseGasCall) -> Self {
            CommandBuilderHarnessCalls::TestBuildInputsBaseGas(var)
        }
    }
    impl ::std::convert::From<TestWriteOutputsCall> for CommandBuilderHarnessCalls {
        fn from(var: TestWriteOutputsCall) -> Self {
            CommandBuilderHarnessCalls::TestWriteOutputs(var)
        }
    }
    impl ::std::convert::From<TestWriteOutputsBaseGasCall>
    for CommandBuilderHarnessCalls {
        fn from(var: TestWriteOutputsBaseGasCall) -> Self {
            CommandBuilderHarnessCalls::TestWriteOutputsBaseGas(var)
        }
    }
    ///Container type for all return fields from the `testBuildInputs` function with signature `testBuildInputs(bytes[],bytes4,bytes32)` and selector `0x7ff6f34f`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct TestBuildInputsReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `testBuildInputsBaseGas` function with signature `testBuildInputsBaseGas(bytes[],bytes4,bytes32)` and selector `0xa402609a`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct TestBuildInputsBaseGasReturn {
        pub out: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `testWriteOutputs` function with signature `testWriteOutputs(bytes[],bytes1,bytes)` and selector `0x171f98cb`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct TestWriteOutputsReturn(
        pub ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all return fields from the `testWriteOutputsBaseGas` function with signature `testWriteOutputsBaseGas(bytes[],bytes1,bytes)` and selector `0x7f180321`
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
    )]
    #[derive(Default)]
    pub struct TestWriteOutputsBaseGasReturn(
        pub ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub ::ethers::core::types::Bytes,
    );
}
