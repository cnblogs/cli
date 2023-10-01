use crate::api::post::Post;
use crate::infra::http::{body_or_err, RequestBuilderExt};
use crate::infra::iter::IntoIteratorExt;
use crate::infra::json;
use crate::infra::result::WrapResult;
use crate::openapi;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::ops::Range;

// HACK:
// Convert skip and take to page index range while page size is 10
fn get_page_index_range(skip: usize, take: usize) -> Range<usize> {
    let page_size = 10;
    let start_index = skip / page_size + 1;
    let end_index = if take == 0 {
        0
    } else {
        ((take + skip) as f64 / page_size as f64).ceil() as usize
    };
    start_index..end_index + 1
}

// HACK:
// Convert skip and take to a range to slice the vec while page size is 10
const fn get_slice_range(skip: usize, take: usize) -> Range<usize> {
    let skip_left = skip - (skip / 10) * 10;
    skip_left..skip_left + take
}

#[test]
fn test_get_page_index_range_and_get_slice_range() {
    fn f(skip: usize, take: usize) -> (Range<usize>, Range<usize>) {
        (
            get_page_index_range(skip, take),
            get_slice_range(skip, take),
        )
    }

    assert_eq!(f(0, 00), (1..1, 0..00));
    assert_eq!(f(0, 01), (1..2, 0..01));
    assert_eq!(f(0, 09), (1..2, 0..09));
    assert_eq!(f(0, 10), (1..2, 0..10));
    assert_eq!(f(0, 11), (1..3, 0..11));
    assert_eq!(f(0, 19), (1..3, 0..19));
    assert_eq!(f(0, 20), (1..3, 0..20));
    assert_eq!(f(0, 21), (1..4, 0..21));

    assert_eq!(f(1, 00), (1..1, 1..01));
    assert_eq!(f(1, 01), (1..2, 1..02));
    assert_eq!(f(1, 09), (1..2, 1..10));
    assert_eq!(f(1, 10), (1..3, 1..11));
    assert_eq!(f(1, 11), (1..3, 1..12));
    assert_eq!(f(1, 19), (1..3, 1..20));
    assert_eq!(f(1, 20), (1..4, 1..21));
    assert_eq!(f(1, 21), (1..4, 1..22));
    assert_eq!(f(1, 29), (1..4, 1..30));

    assert_eq!(f(9, 00), (1..1, 9..09));
    assert_eq!(f(9, 01), (1..2, 9..10));
    assert_eq!(f(9, 09), (1..3, 9..18));
    assert_eq!(f(9, 10), (1..3, 9..19));
    assert_eq!(f(9, 11), (1..3, 9..20));
    assert_eq!(f(9, 19), (1..4, 9..28));
    assert_eq!(f(9, 20), (1..4, 9..29));
    assert_eq!(f(9, 21), (1..4, 9..30));
    assert_eq!(f(9, 29), (1..5, 9..38));

    assert_eq!(f(10, 00), (2..1, 0..00));
    assert_eq!(f(10, 01), (2..3, 0..01));
    assert_eq!(f(10, 09), (2..3, 0..09));
    assert_eq!(f(10, 10), (2..3, 0..10));
    assert_eq!(f(10, 11), (2..4, 0..11));
    assert_eq!(f(10, 19), (2..4, 0..19));
    assert_eq!(f(10, 20), (2..4, 0..20));
    assert_eq!(f(10, 21), (2..5, 0..21));
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResultEntry {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Content")]
    pub summary: String,

    #[serde(rename = "UserName")]
    pub user_name: String,

    #[serde(rename = "VoteTimes")]
    pub vote_count: usize,
    #[serde(rename = "ViewTimes")]
    pub view_count: usize,
    #[serde(rename = "CommentTimes")]
    pub comment_count: usize,

    #[serde(rename = "PublishTime")]
    pub create_time: String,
    #[serde(rename = "Uri")]
    pub url: String,
}

impl Post {
    pub async fn search_site(
        &self,
        skip: usize,
        take: usize,
        keyword: &str,
    ) -> Result<Vec<SearchResultEntry>> {
        let client = &reqwest::Client::new();

        let slice_range = get_slice_range(skip, take);

        let entry_vec = {
            let entry_vec = get_page_index_range(skip, take)
                .map(|i| async move {
                    let req = {
                        let url = openapi!("/zzkdocuments/blog");
                        let query = [
                            ("pageIndex", i.to_string()),
                            ("keyWords", keyword.to_string()),
                        ];
                        client.get(url).query(&query).pat_auth(&self.pat)
                    };
                    let resp = req.send().await?;

                    let body = body_or_err(resp).await?;
                    json::deserialize::<Vec<SearchResultEntry>>(&body)
                })
                .join_all()
                .await
                .into_iter()
                .collect::<Result<Vec<Vec<SearchResultEntry>>>>()?
                .concat();

            entry_vec
                .into_iter()
                .enumerate()
                .filter(|(i, _)| slice_range.contains(i))
                .map(|(_, entry)| entry)
                .collect::<Vec<SearchResultEntry>>()
        };

        entry_vec.wrap_ok()
    }
}
