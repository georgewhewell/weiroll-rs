use std::sync::Arc;

use ethers::prelude::*;
use weiroll::Planner;

abigen!(
    UniswapV2Pair,
    r#"[
        approve(address,uint256)(bool)
        getReserves()(uint112,uint112,uint32)
        token0()(address)
        token1()(address)
    ]"#
);

const PROVIDER_URL: &str = "http://localhost:8545";

#[tokio::main]
pub async fn main() {
    let provider = Provider::<Http>::try_from(PROVIDER_URL).unwrap();
    let _pair = UniswapV2Pair::new(Address::zero(), Arc::new(provider));

    let _planner = Planner::default();
    // planner.call::<ApproveCall, _>((Address::zero(), U256::zero()));
    // let call = pair.token_0();

    todo!()
}
