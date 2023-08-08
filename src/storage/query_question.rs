use crate::{
    config,
    entities::{prelude::*, *},
    leetcode::IdSlug,
};
use miette::{Error, IntoDiagnostic, Result};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tracing::debug;

/// Find the problem, if there are more than one, return multiple
///
/// * `idslug`: id or title
pub async fn get_question_index(idslug: IdSlug) -> Result<Vec<index::Model>, Error> {
    let db = config::conn_db().await?;
    match idslug {
        IdSlug::Id(id) => {
            let models = Index::find_by_id(id)
                .all(&db)
                .await
                .into_diagnostic()?;
            debug!("get value {:#?}", models);
            Ok(models)
        }
        IdSlug::Slug(slug) => {
            let models = Index::find()
                .filter(index::Column::QuestionTitleSlug.contains(slug))
                .all(&db)
                .await
                .into_diagnostic()?;
            debug!("res {:#?}", models);

            Ok(models)
        }
    }
}

/// Find the problem, if there are more than one, return one
///
/// * `idslug`: id or title
pub async fn get_question_index_exact(idslug: IdSlug) -> Result<index::Model, Error> {
    let db = config::conn_db().await?;
    match idslug {
        IdSlug::Id(id) => {
            let models = Index::find_by_id(id)
                .one(&db)
                .await
                .into_diagnostic()?
                .unwrap_or_default();
            debug!("get value {:#?}", models);
            Ok(models)
        }
        IdSlug::Slug(slug) => {
            let models = Index::find()
                .filter(index::Column::QuestionTitleSlug.eq(slug))
                .one(&db)
                .await
                .into_diagnostic()?
                .unwrap_or_default();
            debug!("res {:#?}", models);

            Ok(models)
        }
    }
}

pub async fn query_all_index() -> Result<Vec<index::Model>, Error> {
    let db = config::conn_db().await?;

    let models = Index::find()
        .all(&db)
        .await
        .into_diagnostic()?;

    Ok(models)
}
