use crate::blog_backend;
use crate::infra::http::{body_or_err, setup_auth};
use crate::infra::json;
use crate::infra::result::IntoResult;
use crate::post::Post;
use anyhow::Result;
use chrono::DateTime;
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PostEntry {
    #[serde(rename = "postBody")]
    body: String,
    title: String,
    #[serde(rename = "datePublished")]
    create_time: String,
    #[serde(rename = "dateUpdated")]
    modify_time: String,
    tags: Vec<String>,
    #[serde(rename = "isDraft")]
    is_draft: bool,
    #[serde(rename = "isPinned")]
    is_pinned: bool,
    #[serde(rename = "isPublished")]
    is_published: bool,
    url: String,
}

/*
export type BlogPost = {
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

    // fields also in PostListRespItem
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

    // fields only in PostLispRespItem
    aggCount: number
    feedBackCount: number
    isInSiteCandidate: boolean
    isInSiteHome: boolean
    postConfig: number
    viewCount: number
    webCount: number
}
*/

impl Post {
    pub async fn get_post(&self, id: usize) -> Result<PostEntry> {
        let url = blog_backend!("/posts/{}", id);

        let client = reqwest::Client::new();

        let req = {
            let req = client.get(url);
            setup_auth(req, &self.pat)
        };

        let resp = req.send().await?;
        let json = body_or_err(resp).await?;

        #[derive(Serialize, Deserialize, Debug, Default)]
        struct Body {
            #[serde(rename = "blogPost")]
            pub entry: PostEntry,
        }

        let body = json::deserialize::<Body>(&json)?;

        body.entry.into_ok()
    }
}

impl PostEntry {
    pub fn display_title_body(&self) {
        println!("{}\n", self.title.cyan().bold());
        println!("{}", self.body);
    }

    pub fn display_meta(&self) -> Result<()> {
        if self.is_pinned {
            print!("{} ", " Pinned ".on_blue());
        }
        println!("{}", self.title.cyan().bold());
        if self.is_draft {
            println!("{}", "Draft".yellow());
        }
        if self.is_published {
            println!("{}", "Published".green());
        }
        println!("Words: {}", words_count::count(&self.body).words);
        if let Some(tags) = self
            .tags
            .clone()
            .into_iter()
            .reduce(|acc, tag| format!("{}, {}", acc, tag))
        {
            println!("Tags: {}", tags);
        }
        let create_time = DateTime::parse_from_rfc3339(&format!("{}Z", self.create_time))?;
        println!("Create: {}", create_time.format("%Y/%m/%d %H:%M"));
        let modify_time = DateTime::parse_from_rfc3339(&format!("{}Z", self.create_time))?;
        println!("Modify: {}", modify_time.format("%Y/%m/%d %H:%M"));
        println!("Link: https:{}", self.url);
        ().into_ok()
    }
}
