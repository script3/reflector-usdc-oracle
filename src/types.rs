use sep_40_oracle::Asset;
use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone)]
pub struct OracleConfig {
    /// The address of the source oracle
    pub oracle_id: Address,
    /// The decimals of the source oracle
    pub decimals: u32,
    /// The resolution of the source oracle (in seconds)
    pub resolution: u32,
    /// The base asset of the source oracle
    pub base: Asset,
}
