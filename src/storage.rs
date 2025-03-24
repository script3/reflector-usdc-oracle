use crate::types::OracleConfig;
use sep_40_oracle::Asset;
use soroban_sdk::{contracttype, unwrap::UnwrapOptimized, Address, Env, Symbol};

const ORACLE_KEY: &str = "Oracle";
const DECIMALS_KEY: &str = "Decimals";
const MAX_AGE_KEY: &str = "MaxAge";
const USDC_KEY: &str = "USDC";

const ONE_DAY_LEDGERS: u32 = 17280; // assumes 5 seconds per ledger on average
const LEDGER_THRESHOLD: u32 = 30 * ONE_DAY_LEDGERS;
const LEDGER_BUMP: u32 = 31 * ONE_DAY_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub enum AggregatorDataKey {
    Asset(Asset),
}

//********** Storage Utils **********//

/// Bump the instance lifetime by the defined amount
pub fn extend_instance(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(LEDGER_THRESHOLD, LEDGER_BUMP);
}

/********** Instance **********/

/// Set the max age of a price, in seconds
pub fn set_max_age(e: &Env, max_age: &u64) {
    e.storage()
        .instance()
        .set::<Symbol, u64>(&Symbol::new(e, MAX_AGE_KEY), max_age);
}

/// Set the max age of a price, in seconds
pub fn get_max_age(e: &Env) -> u64 {
    e.storage()
        .instance()
        .get::<Symbol, u64>(&Symbol::new(e, MAX_AGE_KEY))
        .unwrap_optimized()
}

/// Set the number of decimals the oracle will report prices in
pub fn set_decimals(e: &Env, decimals: &u32) {
    e.storage()
        .instance()
        .set::<Symbol, u32>(&Symbol::new(e, DECIMALS_KEY), decimals);
}

/// Get the number of decimals the oracle will report prices in
pub fn get_decimals(e: &Env) -> u32 {
    e.storage()
        .instance()
        .get::<Symbol, u32>(&Symbol::new(e, DECIMALS_KEY))
        .unwrap()
}

/// Set the USDC asset address
pub fn set_usdc(e: &Env, usdc: &Address) {
    e.storage()
        .instance()
        .set::<Symbol, Address>(&Symbol::new(e, USDC_KEY), usdc);
}

/// Get the USDC asset address
pub fn get_usdc(e: &Env) -> Address {
    e.storage()
        .instance()
        .get::<Symbol, Address>(&Symbol::new(e, USDC_KEY))
        .unwrap()
}

/// Set the oracle config
pub fn set_oracle_config(e: &Env, config: &OracleConfig) {
    e.storage()
        .instance()
        .set::<Symbol, OracleConfig>(&Symbol::new(e, ORACLE_KEY), config);
}

/// Get the oracle config
pub fn get_oracle_config(e: &Env) -> OracleConfig {
    e.storage()
        .instance()
        .get::<Symbol, OracleConfig>(&Symbol::new(e, ORACLE_KEY))
        .unwrap()
}
