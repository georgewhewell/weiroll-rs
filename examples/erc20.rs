use std::sync::Arc;

use ethers::abi::RawLog;
use ethers::{abi::ParamType, prelude::*, utils::Anvil};
use weiroll::bindings::events::{Events, EventsEvents};
use weiroll::{
    bindings::{
        erc20::BalanceOfCall,
        events::{LogStringCall, LogUintCall},
        testable_vm::TestableVM,
    },
    Planner,
};

const WETH_ADDR: &str = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2";
const VIT_ADDR: &str = "0xab5801a7d398351b8be11c439e05c5b3259aec9b";
const PROVIDER_URL: &str = "http://localhost:8545";

// planner should prob return [u8; 32]
fn bytes_to_array(bytes: impl Into<Bytes>) -> [u8; 32] {
    let bytes = bytes.into();
    let mut array = [0u8; 32];
    array.copy_from_slice(&bytes[..]);
    array
}

fn state_to_array(state: Vec<Bytes>) -> Vec<[u8; 32]> {
    state
        .into_iter()
        .map(|bytes| bytes_to_array(bytes))
        .collect()
}

#[tokio::main]
pub async fn main() {
    println!("Spawning anvil..");
    let anvil = Anvil::new().fork(PROVIDER_URL).spawn();
    let wallet: LocalWallet = anvil.keys().first().unwrap().clone().into();
    let client = Arc::new(
        Provider::<Http>::try_from(anvil.endpoint())
            .unwrap()
            .with_signer(wallet),
    );

    println!("Deploying contracts..");
    let events = Events::deploy(client.clone(), ())
        .unwrap()
        .send()
        .await
        .unwrap();
    let vm = TestableVM::deploy(client.clone(), ())
        .unwrap()
        .send()
        .await
        .unwrap();

    println!("Planner..");
    let mut planner = Planner::default();
    planner
        .call::<LogStringCall>(
            events.address(),
            vec![String::from("Checking balance..").into()],
            ParamType::Uint(256),
        )
        .unwrap();
    let balance = planner
        .call::<BalanceOfCall>(
            WETH_ADDR.parse::<Address>().unwrap().into(),
            vec![VIT_ADDR.parse::<Address>().unwrap().into()],
            ParamType::Uint(256),
        )
        .unwrap();
    planner
        .call::<LogUintCall>(events.address(), vec![balance.into()], ParamType::Uint(256))
        .unwrap();
    let (commands, state) = planner.plan().unwrap();

    println!("Executing..");
    let receipt = vm
        .execute(state_to_array(commands), state)
        .send()
        .await
        .unwrap()
        .await
        .unwrap()
        .unwrap();

    println!("Logs:");
    for log in receipt.logs {
        let raw = RawLog {
            topics: log.topics,
            data: log.data.to_vec(),
        };
        let call = EventsEvents::decode_log(&raw).unwrap();
        println!("{:?}", call);
    }
}
