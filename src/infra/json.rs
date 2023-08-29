use anyhow::{anyhow, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

pub fn serialize<T>(val: T) -> Result<String>
where
    T: Serialize,
{
    serde_json::to_value::<T>(val)
        .map_err(|e| anyhow!(e))
        .map(|v| v.to_string())
}

pub fn deserialize<T>(json: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let val: Value = serde_json::from_str(json)?;
    serde_json::from_value::<T>(val).map_err(|e| anyhow!(e))
}
