use soroban_sdk::contracterror;
#[contracterror]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum OracleAggregatorErrors {
    NotImplemented = 100,
    InvalidAssetOracle = 101,
    InvalidMaxAge = 102,
    InvalidBaseAsset = 103,
}
