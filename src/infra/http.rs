use crate::infra::result::IntoResult;
use anyhow::bail;
use anyhow::Result;
use reqwest::header::AUTHORIZATION;
use reqwest::{RequestBuilder, Response};
use std::ops::Not;

pub const AUTHORIZATION_TYPE: &str = "Authorization-Type";
pub const PAT: &str = "pat";

#[macro_export]
macro_rules! bearer {
    ($token:expr) => {{
        format!("Bearer {}", $token)
    }};
}

#[macro_export]
macro_rules! basic {
    ($token:expr) => {{
        format!("Basic {}", $token)
    }};
}

pub fn setup_auth(builder: RequestBuilder, pat: &str) -> RequestBuilder {
    let builder = builder.header(AUTHORIZATION, bearer!(pat));
    builder.header(AUTHORIZATION_TYPE, PAT)
}

pub fn cons_query_string(queries: Vec<(impl ToString, impl ToString)>) -> String {
    queries
        .into_iter()
        .map(|(k, v)| {
            let s_k = k.to_string();
            let s_v = v.to_string();
            format!("{}={}", s_k, s_v)
        })
        .fold("".to_string(), |acc, q| format!("{acc}&{q}"))
}

pub async fn unit_or_err(resp: Response) -> Result<()> {
    let code = resp.status();
    let body = resp.text().await?;

    if code.is_success().not() {
        bail!("{}: {}", code, body);
    }

    Ok(())
}

pub async fn body_or_err(resp: Response) -> Result<String> {
    let code = resp.status();
    let body = resp.text().await?;

    if code.is_success() {
        body.into_ok()
    } else {
        bail!("{}: {}", code, body)
    }
}
