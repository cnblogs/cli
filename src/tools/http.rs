use anyhow::Result;
// use owo_colors::OwoColorize;
use reqwest::Response;

use crate::models::NoParseResult;

// #[allow(async_fn_in_trait)]
// pub trait ResponseExt {
//     async fn ensure_success(self) -> Result<Response>;
// }

// impl ResponseExt for Response {
//     async fn ensure_success(self) -> Result<Self> {
//         let status = self.status();

//         if status.is_success() {
//             Ok(self)
//         } else {
//             let body = self.text().await?;
//             anyhow::bail!(
//                 "HTTP Error: status={} {}, body: {}",
//                 status.as_u16(),
//                 status.canonical_reason().unwrap_or("Unknown"),
//                 body
//             )
//         }
//     }
// }

// #[allow(async_fn_in_trait)]
// pub trait ResponseWrite {
//     async fn write_to_stdout(self) -> Result<()>;
// }

// impl ResponseWrite for Response {
//     async fn write_to_stdout(self) -> Result<()> {
//         let status = self.status();
//         if status.is_success() {
//             println!("{}", format!("Ok: {}", status).bright_green());
//         } else {
//             let body = self.text().await?;
//             println!(
//                 "{}",
//                 format!(
//                     "HTTP Error: status={} {}, body: {}",
//                     status.as_u16(),
//                     status.canonical_reason().unwrap_or("Unknown"),
//                     body
//                 )
//                 .red()
//             );
//         }
//         Ok(())
//     }
// }

#[allow(async_fn_in_trait)]
pub trait IntoNoParseResult {
    async fn into_no_parse_result(self) -> Result<NoParseResult>;
}

impl IntoNoParseResult for Response {
    async fn into_no_parse_result(self) -> Result<NoParseResult> {
        let mut res = NoParseResult::default();
        if self.status().is_success() {
            res.status_code = self.status().as_u16();
            res.is_success = self.status().is_success();
        } else {
            let status = self.status();
            res = self.json().await?;
            res.status_code = status.as_u16();
            res.is_success = status.is_success();
        }

        Ok(res)
    }
}
