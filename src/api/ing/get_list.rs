use crate::api::ing::{Ing, IngSendFrom, IngType};
use crate::infra::http::{body_or_err, RequestBuilderExt};
use crate::infra::iter::IntoIteratorExt;
use crate::infra::json;
use crate::infra::result::WrapResult;
use crate::infra::vec::VecExt;
use crate::openapi;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::ops::ControlFlow;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IngEntry {
    pub id: usize,
    pub content: String,
    pub is_private: bool,
    pub is_lucky: bool,
    pub comment_count: usize,
    #[serde(rename = "DateAdded")]
    pub create_time: String,
    pub user_alias: String,
    #[serde(rename = "UserDisplayName")]
    pub user_name: String,
    pub user_icon_url: String,
    pub user_id: usize,
    pub user_guid: String,
    pub send_from: IngSendFrom,
    #[serde(rename = "Icons")]
    pub icons: String,
}

impl Ing {
    pub async fn get_list(
        &self,
        skip: usize,
        take: usize,
        ing_type: &IngType,
    ) -> Result<Vec<IngEntry>> {
        let client = &reqwest::Client::new();

        let range = (skip + 1)..=(skip + take);
        let cf = range
            .map(|i| async move {
                let req = {
                    let url = openapi!("/statuses/@{}", ing_type.clone() as usize);
                    let query = [("pageIndex", i), ("pageSize", 1)];
                    client.get(url).query(&query).pat_auth(&self.pat)
                };

                let resp = req.send().await?;

                let body = body_or_err(resp).await?;

                json::deserialize::<Vec<IngEntry>>(&body)?.pop().wrap_ok()
            })
            .join_all()
            .await
            .into_iter()
            .try_fold(vec![], |acc, it| match it {
                Ok(maybe) => match maybe {
                    Some(entry) => ControlFlow::Continue(acc.chain_push(entry)),
                    None => ControlFlow::Break(Ok(acc)),
                },
                Err(e) => ControlFlow::Break(Err(e)),
            });

        match cf {
            ControlFlow::Continue(vec) => Ok(vec),
            ControlFlow::Break(result) => result,
        }
    }
}
