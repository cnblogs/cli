pub mod http;
pub mod strings;
pub mod timer;

use anyhow::Result;

pub trait IntoAnyhowResult<T> {
    fn into_anyhow_result(self) -> Result<T>;
}

impl<T> IntoAnyhowResult<T> for reqwest::Result<T> {
    fn into_anyhow_result(self) -> Result<T> {
        Ok(self?)
    }
}
