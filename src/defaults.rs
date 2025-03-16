use std::time::Duration;

pub const API_BASE_URL: &str = "https://api-v2.flipsidecrypto.xyz/json-rpc";
pub const TTL_MINUTES: u64 = 60;
pub const MAX_AGE_MINUTES: u64 = 0;
pub const CACHED: bool = true;
pub const DATA_PROVIDER: &str = "flipside";
pub const DATA_SOURCE: &str = "snowflake-default";
pub const TIMEOUT: Duration = Duration::from_secs(20 * 60);
pub const RETRY_INTERVAL: Duration = Duration::from_millis(500);
pub const PAGE_SIZE: usize = 100000;
pub const PAGE_NUMBER: usize = 1;
//pub const sdkPackage: "js";
//pub const sdkVersion: version;
