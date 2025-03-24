#![cfg(test)]
use std::println;

use crate::testutils::{setup_default_aggregator, EnvTestUtils};
use sep_40_oracle::Asset;
use soroban_sdk::{testutils::Address as _, vec, Address, Env, Vec};

#[test]
fn test_lastprice() {
    let e = Env::default();
    e.set_default_info();
    e.mock_all_auths();
    let usdc = Address::generate(&e);
    let asset_0 = Address::generate(&e);
    let asset_1 = Address::generate(&e);

    let (oracle_aggregator_client, oracle_1) =
        setup_default_aggregator(&e, &usdc, &asset_0, &asset_1);

    oracle_1.set_price(
        &Vec::from_array(&e, [0_110000000, 1_000000000]),
        &e.ledger().timestamp(),
    );

    let price_0 = oracle_aggregator_client
        .lastprice(&Asset::Stellar(asset_0))
        .unwrap();
    assert_eq!(price_0.price, 0_1100000);
    assert_eq!(price_0.timestamp, e.ledger().timestamp());

    let price_1 = oracle_aggregator_client
        .lastprice(&Asset::Stellar(asset_1))
        .unwrap();
    println!("{:?}", e.cost_estimate().resources());
    assert_eq!(price_1.price, 1_0000000);
    assert_eq!(price_1.timestamp, e.ledger().timestamp());
}

#[test]
fn test_lastprice_exceeds_max_timestamp() {
    let e = Env::default();
    e.set_default_info();
    e.mock_all_auths();
    let usdc = Address::generate(&e);
    let asset_0 = Address::generate(&e);
    let asset_1 = Address::generate(&e);

    let (oracle_aggregator_client, oracle_1) =
        setup_default_aggregator(&e, &usdc, &asset_0, &asset_1);

    let recent_norm_time = e.ledger().timestamp() / 300 * 300;
    oracle_1.set_price(
        &Vec::from_array(&e, [0_120000000, 1_010000000]),
        &(recent_norm_time - 1200),
    );
    oracle_1.set_price(
        &Vec::from_array(&e, [0_120000000, 1_010000000]),
        &(recent_norm_time - 900),
    );

    // jump 1 block to ensure the most recent price is > 900 seconds old
    e.jump(1);

    let price_0 = oracle_aggregator_client.lastprice(&Asset::Stellar(asset_0));
    assert!(price_0.is_none());
}

#[test]
fn test_lastprice_retries_with_timestamp() {
    let e = Env::default();
    e.set_default_info();
    e.mock_all_auths();
    let usdc = Address::generate(&e);
    let asset_0 = Address::generate(&e);
    let asset_1 = Address::generate(&e);

    let (oracle_aggregator_client, oracle_1) =
        setup_default_aggregator(&e, &usdc, &asset_0, &asset_1);

    let recent_norm_time = e.ledger().timestamp() / 300 * 300;
    oracle_1.set_price(
        &Vec::from_array(&e, [0_120000000, 1_010000000]),
        &(recent_norm_time - 600),
    );
    oracle_1.set_price(&vec![&e], &(recent_norm_time - 300));
    oracle_1.set_price(&vec![&e], &recent_norm_time);

    e.jump(10);

    let price_0 = oracle_aggregator_client
        .lastprice(&Asset::Stellar(asset_0))
        .unwrap();
    assert_eq!(price_0.price, 0_1200000);
    assert_eq!(price_0.timestamp, recent_norm_time - 600);
}

#[test]
fn test_lastprice_retry_exceeds_max_timestamp() {
    let e = Env::default();
    e.set_default_info();
    e.mock_all_auths();
    let usdc = Address::generate(&e);
    let asset_0 = Address::generate(&e);
    let asset_1 = Address::generate(&e);

    let (oracle_aggregator_client, oracle_1) =
        setup_default_aggregator(&e, &usdc, &asset_0, &asset_1);

    let recent_norm_time = e.ledger().timestamp() / 300 * 300;
    oracle_1.set_price(
        &Vec::from_array(&e, [0_120000000, 1_010000000]),
        &(recent_norm_time - 1200),
    );
    oracle_1.set_price(
        &Vec::from_array(&e, [0_120000000, 1_010000000]),
        &(recent_norm_time - 900),
    );
    oracle_1.set_price(&Vec::from_array(&e, []), &e.ledger().timestamp());

    // jump 1 block to ensure the most recent price is > 900 seconds old
    e.jump(1);

    let price_0 = oracle_aggregator_client.lastprice(&Asset::Stellar(asset_0));
    assert!(price_0.is_none());
}

#[test]
fn test_lastprice_retry_stops_if_over_max_age() {
    let e = Env::default();
    e.set_default_info();
    e.mock_all_auths();
    let usdc = Address::generate(&e);
    let asset_0 = Address::generate(&e);
    let asset_1 = Address::generate(&e);

    let (oracle_aggregator_client, oracle_1) =
        setup_default_aggregator(&e, &usdc, &asset_0, &asset_1);

    let recent_norm_time = e.ledger().timestamp() / 300 * 300;
    oracle_1.set_price(&Vec::from_array(&e, []), &(recent_norm_time - 1200));
    oracle_1.set_price(&Vec::from_array(&e, []), &(recent_norm_time - 900));
    oracle_1.set_price(&Vec::from_array(&e, []), &(recent_norm_time - 600));
    oracle_1.set_price(&Vec::from_array(&e, []), &(recent_norm_time - 300));
    oracle_1.set_price(&Vec::from_array(&e, []), &recent_norm_time);

    // jump 1 block to ensure the most recent price is > 900 seconds old
    e.jump(1);

    // validate price is not found and ledger entries are less than 10
    let price_0 = oracle_aggregator_client.lastprice(&Asset::Stellar(asset_0));
    let read_entries_0 = e.cost_estimate().resources().read_entries;
    assert!(price_0.is_none());
    // 4 read for usdc, oracle config, decimals, and max age
    // 1 read for oracle contract
    // 4 reads for price data from oracle contract
    assert!(read_entries_0 < 10);
}

#[test]
fn test_lastprice_checks_if_asset_is_usdc() {
    let e = Env::default();
    e.set_default_info();
    e.mock_all_auths();
    let usdc = Address::generate(&e);
    let asset_0 = Address::generate(&e);
    let asset_1 = Address::generate(&e);

    let (oracle_aggregator_client, oracle_1) =
        setup_default_aggregator(&e, &usdc, &asset_0, &asset_1);

    oracle_1.set_price(
        &Vec::from_array(&e, [0_110000000, 1_000000000]),
        &e.ledger().timestamp(),
    );

    let price_0 = oracle_aggregator_client
        .lastprice(&Asset::Stellar(usdc))
        .unwrap();
    assert_eq!(price_0.price, 1_0000000);
    assert_eq!(price_0.timestamp, e.ledger().timestamp());
}
