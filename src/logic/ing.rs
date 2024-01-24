//! 闪存相关逻辑
//!

use anyhow::{Ok, Result};

use crate::{
    api::{
        self,
        ing::{get_comment_list::IngCommentEntry, get_list::IngEntry},
    },
    apis::ing::{comment, delete, post, query as iq, query_by_id},
    args::cmd::ing::{CreateIng, QueryIng},
    infra::iter::IntoIteratorExt,
};

/// 根据queryset查询
/// TODO: 提到我和我评论的解析存在问题。
pub async fn get_ings_and_comments(
    t: &str,
    q: &QueryIng,
) -> Result<Vec<(api::ing::get_list::IngEntry, Vec<IngCommentEntry>)>> {
    if let Some(ids) = &q.id {
        let a = ids
            .iter()
            .map(|id| async move { query_by_id(t, id).await })
            .join_all()
            .await
            .into_iter()
            .filter(|x| {
                if x.is_err() {
                    eprintln!("{}", x.as_ref().err().unwrap())
                }
                x.is_ok()
            })
            .map(|x| x.unwrap())
            .collect::<Vec<IngEntry>>();

        get_ing_comments(t, a).await
    } else {
        let a = iq(t, &q.into())
            .await?
            .into_iter()
            .collect::<Vec<api::ing::get_list::IngEntry>>();
        get_ing_comments(t, a).await
    }
}

// TODO: 分类细化
/// 初步提取公共部分
pub async fn get_ing_comments(
    t: &str,
    i: Vec<IngEntry>,
) -> Result<Vec<(IngEntry, Vec<IngCommentEntry>)>> {
    let a = i
        .into_iter()
        .map(|ing| async {
            let result = comment::get(t, ing.id.to_string().as_str()).await;
            result.map(|comment_vec| (ing, comment_vec))
        })
        .join_all()
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;
    Ok(a)
}

/// 通过ID删除
pub async fn delete_by_ing_id(t: &str, ids: Vec<u64>) {
    ids.into_iter()
        .map(|id| async move { delete(t, id).await })
        .join_all()
        .await
        .iter()
        .for_each(|x| {
            if x.is_err() {
                eprintln!("{:?}", x.as_ref().err().unwrap().to_string())
            }
        });
}

/// 创建闪存
pub async fn create_ing_with_arg(t: &str, c: CreateIng) {
    let cc = c.into();
    let e = post(t, &cc).await;

    if e.is_err() {
        eprintln!("{:?}", e.as_ref().err().unwrap().to_string());
    } else {
        println!("🙈 ! {:?}", cc.content);
    }
}
