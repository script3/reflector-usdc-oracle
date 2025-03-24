use crate::{errors::OracleAggregatorErrors, price_data::get_price, storage, types::OracleConfig};
use sep_40_oracle::{Asset, PriceData, PriceFeedClient, PriceFeedTrait};
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, Env, Vec};

#[contract]
pub struct OracleAggregator;

#[contractimpl]
impl PriceFeedTrait for OracleAggregator {
    fn resolution(e: Env) -> u32 {
        panic_with_error!(e, OracleAggregatorErrors::NotImplemented);
    }

    fn price(e: Env, _asset: Asset, _timestamp: u64) -> Option<PriceData> {
        panic_with_error!(e, OracleAggregatorErrors::NotImplemented);
    }

    fn prices(e: Env, _asset: Asset, _records: u32) -> Option<Vec<PriceData>> {
        panic_with_error!(e, OracleAggregatorErrors::NotImplemented);
    }

    fn base(e: Env) -> Asset {
        storage::get_oracle_config(&e).base
    }

    fn decimals(e: Env) -> u32 {
        storage::get_decimals(&e)
    }

    fn assets(e: Env) -> Vec<Asset> {
        let config = storage::get_oracle_config(&e);
        let usdc = storage::get_usdc(&e);
        let oracle = PriceFeedClient::new(&e, &config.oracle_id);
        let mut assets = oracle.assets();
        for asset in &assets {
            match asset {
                Asset::Stellar(addr) if addr == usdc => {
                    // if the asset is USDC, return the price in base asset
                    return assets;
                }
                _ => {}
            }
        }
        assets.push_back(Asset::Stellar(usdc));
        assets
    }

    fn lastprice(e: Env, asset: Asset) -> Option<PriceData> {
        let usdc = storage::get_usdc(&e);
        match asset {
            Asset::Stellar(addr) if addr == usdc => {
                // if the asset is USDC, return the price in base asset
                return Some(PriceData {
                    price: 10i128.pow(storage::get_decimals(&e) as u32),
                    timestamp: e.ledger().timestamp(),
                });
            }
            _ => {}
        }

        get_price(&e, &asset)
    }
}

#[contractimpl]
impl OracleAggregator {
    /// Initialize the oracle aggregator contract.
    ///
    /// ### Arguments
    /// * `oracle_id` - The address of the oracle
    /// * `usdc_id` - The address of the USDC asset
    /// * `decimals` - The decimals the oracle will report in
    /// * `max_age` - The maximum time the oracle will look back for a price (in seconds)
    ///
    /// ### Errors
    /// * `InvalidMaxAge` - The max age is not between 360 (6m) and 3600 (60m)
    /// * `InvalidBaseAsset` - The base asset of the oracle is not USDC
    pub fn __constructor(
        e: Env,
        oracle_id: Address,
        usdc_id: Address,
        decimals: u32,
        max_age: u64,
    ) {
        storage::extend_instance(&e);
        storage::set_decimals(&e, &decimals);
        if max_age < 360 || max_age > 3600 {
            panic_with_error!(&e, OracleAggregatorErrors::InvalidMaxAge);
        }
        storage::set_max_age(&e, &max_age);

        let oracle = PriceFeedClient::new(&e, &oracle_id);
        let base = oracle.base();
        match base.clone() {
            Asset::Stellar(addr) => {
                if addr != usdc_id {
                    panic_with_error!(&e, OracleAggregatorErrors::InvalidBaseAsset);
                    // otherwise, set the base to the address of the base asset
                }
            }

            _ => panic_with_error!(&e, OracleAggregatorErrors::InvalidBaseAsset),
        }

        storage::set_usdc(&e, &usdc_id);
        storage::set_oracle_config(
            &e,
            &OracleConfig {
                oracle_id,
                resolution: oracle.resolution(),
                decimals: oracle.decimals(),
                base,
            },
        );
    }

    /// Fetch the max age of a price
    pub fn max_age(e: Env) -> u64 {
        storage::get_max_age(&e)
    }

    pub fn config(e: Env) -> OracleConfig {
        storage::get_oracle_config(&e)
    }

    pub fn usdc(e: Env) -> Address {
        storage::get_usdc(&e)
    }
}
