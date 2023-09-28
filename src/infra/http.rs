use crate::infra::result::WrapResult;
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

pub trait RequestBuilderExt {
    fn pat_auth(self, pat: &str) -> RequestBuilder;
}

impl RequestBuilderExt for RequestBuilder {
    fn pat_auth(self, pat: &str) -> RequestBuilder {
        let builder = self.header(AUTHORIZATION, bearer!(pat));
        builder.header(AUTHORIZATION_TYPE, PAT)
    }
}

pub trait VecExt {
    fn into_query_string(self) -> String;
}

impl<K: ToString, V: ToString> VecExt for Vec<(K, V)> {
    fn into_query_string(self) -> String {
        self.into_iter()
            .map(|(k, v)| {
                let s_k = k.to_string();
                let s_v = v.to_string();
                format!("{}={}", s_k, s_v)
            })
            .fold(String::new(), |acc, q| format!("{acc}&{q}"))
    }
}

pub async fn unit_or_err(resp: Response) -> Result<()> {
    let code = resp.status();
    let body = resp.text().await?;

    if code.is_success().not() {
        bail!("{}: {}", code, body);
    }

    ().wrap_ok()
}

pub async fn body_or_err(resp: Response) -> Result<String> {
    let code = resp.status();
    let body = resp.text().await?;

    if code.is_success() {
        body.wrap_ok()
    } else {
        bail!("{}: {}", code, body)
    }
}
