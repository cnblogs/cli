use crate::api::post::Post;
use crate::blog_backend;
use crate::infra::http::{body_or_err, setup_auth};
use crate::infra::result::IntoResult;
use anyhow::Result;
use serde_json::Value;

/*
Fields only available over blog_backend!("/posts/{}", id):
  postBody: string
  categoryIds: []
  collectionIds: []
  inSiteCandidate: boolean
  inSiteHome: boolean
  siteCategoryId: null
  blogTeamIds: []
  displayOnHomePage: boolean
  isAllowComments: boolean
  includeInMainSyndication: boolean
  isOnlyForRegisterUser: boolean
  isUpdateDateAdded: boolean
  description: string
  featuredImage: null
  tags: []
  password: null
  autoDesc: string
  changePostType: boolean
  blogId: number
  author: string
  removeScript: boolean
  clientInfo: null
  changeCreatedTime: boolean
  canChangeCreatedTime: boolean
  isContributeToImpressiveBugActivity: boolean
  usingEditorId: null
  sourceUrl: null

Fields available over blog_backend!("/posts/{}", id) and blog_backend!("/posts/list?{}", query):
  id: number
  postType: PostType
  accessPermission: AccessPermission
  title: string
  url: string
  entryName: null
  datePublished: string
  dateUpdated: string
  isMarkdown: boolean
  isDraft: boolean
  isPinned: boolean
  isPublished: boolean
*/

impl Post {
    /**
    Get raw json from remote

    Use this while it's hard to deserialize to struct
    **/
    pub async fn get_one_raw(&self, id: usize) -> Result<Value> {
        let client = reqwest::Client::new();

        let req = {
            let url = blog_backend!("/posts/{}", id);
            let req = client.get(url);
            setup_auth(req, &self.pat)
        };

        let resp = req.send().await?;

        let mut json = {
            let body = body_or_err(resp).await?;
            serde_json::from_str::<Value>(&body)
        }?;

        json["blogPost"].take().into_ok()
    }
}
