# Reflector USDC Oracle

This Oracle Aggregator is intended to be used with a Reflector Oracle that reports price in USDC. The price of USDC is hard coded to return 1 with the desired number of decimals. 

### Supported Oracles

This Reflector USDC Oracle Aggregator contract makes a few assumptions about the oracles it can support:

* Oracle must report price in usdc
* Oracle must support SEP-40 `lastprice(asset: Address)`, and it should return the most recently reported price by the oracle
* If `lastprice(asset: Address)` can return `None` intermittently, like in the event of the most recent round being missed, the oracle must support SEP-40 `price(asset: Address, timestamp: u64)`, and it should return the most recently reported price on or before the timestamp given

### Config

The oracle aggregator uses some global configuration defined through the constructor:

* oracle_id `Address` - The address of the reflector oracle
* usdc_id `Address` - The address of the USDC asset
* decimals `u32` - The decimals the oracle aggregator will report prices in
* max_age `u64` - The maximum age (in seconds) of a fetched price the oracle aggregator will return from the current ledger timestamp. This must be between 360s (6m) and 3600s (60m).

### Last Price Method

The aggregator will attempt to fetch the assets price via `lastprice` first. Some oracles opt to return `None` if the latest round did not reach consensus, or there was an issue. In this case, the aggregator will attempt to call `price` for each `resolution` period since the current timestamp, up to the `max_age` of a price. If no price can be resolved that is at most `max_age` old, the aggregator will return `None`.

## Safety

Oracle Aggregator has not had an audit conducted. If an audit is conducted, it will appear here.

Oracle Aggregator is made available under the MIT License, which disclaims all warranties in relation to the project and which limits the liability of those that contribute and maintain the project, including Script3. You acknowledge that you are solely responsible for any use of Oracle Aggregator and you assume all risks associated with any such use.
