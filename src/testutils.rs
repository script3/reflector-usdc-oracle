#![cfg(test)]

use crate::contract::OracleAggregatorClient;
use sep_40_oracle::{
    testutils::{Asset as MockAsset, MockPriceOracleClient, MockPriceOracleWASM},
    Asset,
};
use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    vec, Address, Env, Vec,
};
pub mod oracle_aggregator {
    soroban_sdk::contractimport!(
        file = "./target/wasm32-unknown-unknown/optimized/oracle_aggregator.wasm"
    );
}

const ONE_DAY_LEDGERS: u32 = 24 * 60 * 60 / 5;

pub trait EnvTestUtils {
    /// Jump the env by the given amount of ledgers. Assumes 5 seconds per ledger.
    fn jump(&self, ledgers: u32);

    /// Set the ledger to the default LedgerInfo
    ///
    /// Time -> 1441065600 (Sept 1st, 2015 12:00:00 AM UTC)
    /// Sequence -> 100
    fn set_default_info(&self);
}

impl EnvTestUtils for Env {
    fn jump(&self, ledgers: u32) {
        self.ledger().set(LedgerInfo {
            timestamp: self.ledger().timestamp().saturating_add(ledgers as u64 * 5),
            protocol_version: 22,
            sequence_number: self.ledger().sequence().saturating_add(ledgers),
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 50 * ONE_DAY_LEDGERS,
            min_persistent_entry_ttl: 50 * ONE_DAY_LEDGERS,
            max_entry_ttl: 365 * ONE_DAY_LEDGERS,
        });
    }

    fn set_default_info(&self) {
        self.ledger().set(LedgerInfo {
            timestamp: 1441065600, // Sept 1st, 2015 12:00:00 AM UTC
            protocol_version: 22,
            sequence_number: 100,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 50 * ONE_DAY_LEDGERS,
            min_persistent_entry_ttl: 50 * ONE_DAY_LEDGERS,
            max_entry_ttl: 365 * ONE_DAY_LEDGERS,
        });
    }
}

/// Deploy an oracle aggreator contract
pub fn create_oracle_aggregator<'a>(
    e: &Env,
    oracle: &Address,
    usdc: &Address,
    decimals: &u32,
    max_age: &u64,
) -> (Address, OracleAggregatorClient<'a>) {
    let oracle_aggregator_address = Address::generate(&e);
    e.register_at(
        &oracle_aggregator_address,
        oracle_aggregator::WASM,
        (oracle, usdc.clone(), decimals, max_age),
    );
    let oracle_aggregator_client: OracleAggregatorClient<'a> =
        OracleAggregatorClient::new(&e, &oracle_aggregator_address);
    return (oracle_aggregator_address, oracle_aggregator_client);
}

/// Setup an oracle aggregator with default test setttings based on the current env timestamp.
///
/// ### Returns
/// Two oracle aggegator clients:
/// - (0 and 1 oracle, 2 oracle)
pub fn setup_default_aggregator<'a>(
    e: &Env,
    usdc: &Address,
    asset_0: &Address,
    asset_1: &Address,
) -> (OracleAggregatorClient<'a>, MockPriceOracleClient<'a>) {
    // setup oracle with XLM and USDC price
    let oracle_id = Address::generate(&e);
    e.register_at(&oracle_id, MockPriceOracleWASM, ());
    let oracle = MockPriceOracleClient::new(&e, &oracle_id);
    oracle.set_data(
        &Address::generate(&e),
        &MockAsset::Stellar(usdc.clone()),
        &Vec::from_array(
            &e,
            [
                MockAsset::Stellar(asset_0.clone()),
                MockAsset::Stellar(asset_1.clone()),
            ],
        ),
        &9,
        &300,
    );

    oracle.set_price(&vec![&e, 0i128, 0i128], &0);

    let (_, aggregator_client) = create_oracle_aggregator(e, &oracle_id, usdc, &7, &900);

    return (aggregator_client, oracle);
}

pub fn assert_assets_equal(a: Asset, b: Asset) -> bool {
    match (a, b) {
        (Asset::Stellar(a), Asset::Stellar(b)) => a == b,
        (Asset::Other(a), Asset::Other(b)) => a == b,
        _ => false,
    }
}
