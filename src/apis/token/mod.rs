//! Token
//!
//! TokenApi的封装
//!
//! OAuth认证，提供两种方式接口。
//!
//! 1. Client_Credentials
//! 2. Authorization_Code
//!

use super::{OAUTH_CLIENT, OAUTH_TOKEN};
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// 认证授权后的授权
///
/// # Filed
///
/// - access_token: Token String
/// - expires_in: 过期时间
/// - token_type: Token认证方式
/// - refresh_token: 过期后刷新Token，如果是ClientCredentials，此字段无用。
/// - id_token: id，如果是ClientCredentials，此字段无用
/// - scope: 客户端权限
///
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct OAuthToken {
    pub id_token: String,
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct ClientCredentialsReq {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: String,
}

pub async fn client_credentials(req: ClientCredentialsReq) -> Result<OAuthToken> {
    let c = Client::new().post(OAUTH_CLIENT);
    let r = c
        .form(&req)
        .send()
        .await?
        .error_for_status()?
        .json::<OAuthToken>()
        .await?;
    Ok(r)
}

/// OAuth 获取Token结构体
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OauthTokenReq {
    // pub client_id: String,     // string	是	授权ID	client_id
    // pub client_secret: String, //	string	是	密钥	client_secret
    // pub grant_type: String,    //	string	是	授权模式	authorization_code
    #[serde(flatten)]
    pub cc: ClientCredentialsReq,
    pub code: String,         //	string	是	授权码	code
    pub redirect_uri: String, //	string	是	回调地址(默认)	https://oauth.cnblogs.com/auth/callback
}

impl OauthTokenReq {
    pub fn new(client_id: String, client_secret: String, code: String) -> Self {
        OauthTokenReq {
            cc: ClientCredentialsReq {
                client_id,
                client_secret,
                grant_type: "authorization_code".to_string(),
            },
            code,
            redirect_uri: "https://oauth.cnblogs.com/auth/callback".to_string(),
        }
    }
}

/// 获取令牌
pub async fn authorization_code(req: OauthTokenReq) -> Result<OAuthToken> {
    let c = Client::new().post(OAUTH_TOKEN);
    let r = c
        .form(&req)
        .send()
        .await?
        .error_for_status()?
        .json::<OAuthToken>()
        .await?;
    Ok(r)
}

/// OAuth 获取Token结构体
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RefreshTokenReq {
    // pub client_id: String,
    // pub client_secret: String,
    // pub grant_type: String,
    #[serde(flatten)]
    pub cc: ClientCredentialsReq,
    pub refresh_token: String,
}

impl RefreshTokenReq {
    pub fn new(client_id: String, client_secret: String, refresh_token: String) -> Self {
        return RefreshTokenReq {
            cc: ClientCredentialsReq {
                client_id,
                client_secret,
                grant_type: "refresh_token".to_string(),
            },
            refresh_token,
        };
    }
}

/// 刷新令牌
///
/// 令牌过期后重新获取。
pub async fn refresh_token(req: RefreshTokenReq) -> Result<OAuthToken> {
    let c = Client::new().post(OAUTH_TOKEN);
    let r = c
        .form(&req)
        .send()
        .await?
        .error_for_status()?
        .json::<OAuthToken>()
        .await?;
    Ok(r)
}
