#![cfg(test)]
use crate::testutils::{create_oracle_aggregator, EnvTestUtils};
use sep_40_oracle::{
    testutils::{Asset as MockAsset, MockPriceOracleClient, MockPriceOracleWASM},
    Asset,
};
use soroban_sdk::{testutils::Address as _, Address, Env, Vec};

#[test]
fn test_init() {
    let e = Env::default();
    e.set_default_info();
    e.mock_all_auths();
    e.cost_estimate().budget().reset_unlimited();

    let usdc = Address::generate(&e);
    let decimals = 7;
    let max_age = 360;

    // deploy mock oracle
    let oracle_id = Address::generate(&e);
    e.register_at(&oracle_id, MockPriceOracleWASM, ());
    let oracle = MockPriceOracleClient::new(&e, &oracle_id);
    oracle.set_data(
        &Address::generate(&e),
        &MockAsset::Stellar(usdc.clone()),
        &Vec::from_array(&e, []),
        &9,
        &300,
    );
    let (_, oracle_aggregator) =
        create_oracle_aggregator(&e, &oracle_id, &usdc, &decimals, &max_age);
    let config = oracle_aggregator.config();
    match config.base {
        Asset::Stellar(addr) => assert_eq!(addr, usdc),
        _ => panic!("Expected base asset to be USDC"),
    }
    assert_eq!(config.decimals, 9);
    assert_eq!(config.resolution, 300);
    assert_eq!(config.oracle_id, oracle_id);
    assert_eq!(max_age, oracle_aggregator.max_age());
    assert_eq!(usdc, oracle_aggregator.usdc());
}

#[test]
#[should_panic(expected = "Error(Contract, #103)")]
fn test_init_oracle_base_not_usdc() {
    let e = Env::default();
    e.set_default_info();
    e.mock_all_auths();
    e.cost_estimate().budget().reset_unlimited();

    let usdc = Address::generate(&e);
    let decimals = 7;
    let max_age = 360;

    // deploy mock oracle
    let oracle_id = Address::generate(&e);
    e.register_at(&oracle_id, MockPriceOracleWASM, ());
    let oracle = MockPriceOracleClient::new(&e, &oracle_id);
    oracle.set_data(
        &Address::generate(&e),
        &MockAsset::Stellar(Address::generate(&e)),
        &Vec::from_array(&e, []),
        &9,
        &300,
    );
    create_oracle_aggregator(&e, &oracle_id, &usdc, &decimals, &max_age);
}

#[test]
#[should_panic(expected = "Error(Contract, #102)")]
fn test_init_max_age_too_small() {
    let e = Env::default();
    e.set_default_info();
    e.mock_all_auths();
    e.cost_estimate().budget().reset_unlimited();

    let usdc = Address::generate(&e);
    let decimals = 7;
    let max_age = 359;

    // deploy mock oracle
    let oracle_id = Address::generate(&e);
    e.register_at(&oracle_id, MockPriceOracleWASM, ());
    let oracle = MockPriceOracleClient::new(&e, &oracle_id);
    oracle.set_data(
        &Address::generate(&e),
        &MockAsset::Stellar(usdc.clone()),
        &Vec::from_array(&e, []),
        &9,
        &300,
    );
    create_oracle_aggregator(&e, &oracle_id, &usdc, &decimals, &max_age);
}

#[test]
#[should_panic(expected = "Error(Contract, #102)")]
fn test_init_max_age_too_large() {
    let e = Env::default();
    e.set_default_info();
    e.mock_all_auths();
    e.cost_estimate().budget().reset_unlimited();

    let usdc = Address::generate(&e);
    let decimals = 7;
    let max_age = 3601;

    // deploy mock oracle
    let oracle_id = Address::generate(&e);
    e.register_at(&oracle_id, MockPriceOracleWASM, ());
    let oracle = MockPriceOracleClient::new(&e, &oracle_id);
    oracle.set_data(
        &Address::generate(&e),
        &MockAsset::Stellar(usdc.clone()),
        &Vec::from_array(&e, []),
        &9,
        &300,
    );
    create_oracle_aggregator(&e, &oracle_id, &usdc, &decimals, &max_age);
}
