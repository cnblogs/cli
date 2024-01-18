//! 闪存相关逻辑
//!

use crate::{
    apis::ing::{query_by_id, query as iq},
    args::cmd::ing::QueryIng,
    infra::iter::IntoIteratorExt,
};

pub async fn get_ings_and_comments(t: &str, q: &QueryIng) {
    println!("{:?}", q);
    if let Some(ids) = &q.id {
        let a = ids
            .iter()
            .map(|id| async move { query_by_id(t, id).await })
            .join_all()
            .await
            ;

        a.iter().for_each(|i| {
            match i {
                Ok(e) => {
                    println!("{:?}", e);
                }
                Err(e) => {
                    eprintln!("{:?}", e);
                }
            }
        })
        
    }
    
    let a: Result<Vec<crate::apis::ing::IngEntry>, anyhow::Error> = iq(t, &q.into()).await;
    match a {
        Ok(e) => {
            println!("{:?}", e);
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }

}
