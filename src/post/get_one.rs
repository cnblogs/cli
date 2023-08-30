use crate::blog_backend;
use crate::infra::http::{body_or_err, setup_auth};
use crate::infra::json;
use crate::infra::result::IntoResult;
use crate::post::Post;
use anyhow::Result;
use chrono::DateTime;
use colored::Colorize;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct PostEntry {
    pub id: usize,
    pub title: String,
    pub url: String,

    #[serde(rename = "datePublished")]
    pub create_time: String,
    #[serde(rename = "dateUpdated")]
    pub modify_time: String,

    #[serde(rename = "isDraft")]
    pub is_draft: bool,
    #[serde(rename = "isPinned")]
    pub is_pinned: bool,
    #[serde(rename = "isPublished")]
    pub is_published: bool,

    // WRN:
    // Limited by the design of blog backend API
    // None implies that this filed is not fetched from server yet but DOSE NOT MEAN IT NOT EXIST
    #[serde(rename = "feedBackCount")]
    pub comment_count: Option<usize>,
    #[serde(rename = "postBody")]
    pub body: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl Post {
    pub async fn get_one(&self, id: usize) -> Result<PostEntry> {
        let url = blog_backend!("/posts/{}", id);

        let client = reqwest::Client::new();

        let req = {
            let req = client.get(url);
            setup_auth(req, &self.pat)
        };

        let resp = req.send().await?;
        let json = body_or_err(resp).await?;

        let body = {
            #[derive(Serialize, Deserialize, Debug)]
            struct Body {
                #[serde(rename = "blogPost")]
                pub entry: PostEntry,
            }
            json::deserialize::<Body>(&json)?
        };

        body.entry.into_ok()
    }
}

impl PostEntry {
    pub fn display_title_body(&self) {
        println!("{}\n", self.title.cyan().bold());
        if let Some(body) = &self.body {
            println!("{}", body);
        }
    }

    pub fn display_meta(&self) -> Result<()> {
        println!("{}", self.title.cyan().bold());
        {
            print!("Status");
            if self.is_published {
                print!(" {}", "Published".green());
            } else {
                print!(" {}", "Draft".yellow());
            }
            if self.is_pinned {
                print!(" {}", "Pinned".magenta());
            }
            println!()
        };
        if let Some(body) = &self.body {
            let words_count = words_count::count(body).words;
            println!("Words  {}", words_count);
        }
        if let Some(tags) = &self.tags {
            if let Some(tags_text) = tags
                .clone()
                .into_iter()
                .reduce(|acc, tag| format!("{}, {}", acc, tag))
            {
                println!("Tags   {}", tags_text);
            }
        }
        let create_time = DateTime::parse_from_rfc3339(&format!("{}Z", self.create_time))?;
        println!("Create {}", create_time.format("%Y/%m/%d %H:%M"));
        let modify_time = DateTime::parse_from_rfc3339(&format!("{}Z", self.create_time))?;
        println!("Modify {}", modify_time.format("%Y/%m/%d %H:%M"));
        println!("Link   https:{}", self.url);

        ().into_ok()
    }
}
