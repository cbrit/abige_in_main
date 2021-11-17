use ethers::contract::abigen;
use ethers::prelude::*;

abigen!(
    AaveCellar,
    "./src/aave_lend_cellar_abi.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

fn main() {

}
