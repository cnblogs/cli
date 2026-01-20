pub mod fav;
pub mod ing;
pub mod news;
pub mod post;
pub mod user;

use owo_colors::OwoColorize;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct NoParseResult {
    #[serde(skip)]
    pub status_code: u16,
    #[serde(skip)]
    pub is_success: bool,
    // pub target_site: String,
    pub message: String,
    // pub data: String,
    // pub inner_exception: String,
    // pub help_link: String,
    // pub source: String,
    // pub h_result: u64,
    // pub stack_trace: String,
}

impl NoParseResult {
    pub fn into_format(self) -> String {
        if self.is_success {
            format!(
                "[{info}] {status_code}",
                info = "Ok".bright_green(),
                status_code = self.status_code,
            )
        } else {
            format!(
                "[{info}] {status_code}: {msg}",
                info = "Error".red(),
                status_code = self.status_code,
                msg = self.message
            )
        }
    }
}
