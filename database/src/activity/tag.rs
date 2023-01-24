//! Database activities for tags
//!

use common::objects;
use diesel_async::AsyncPgConnection;

use crate::models;

use super::ActivityResult;

pub async fn list(
    conn: &mut AsyncPgConnection,
    pattern: &str,
) -> ActivityResult<Vec<objects::Tag>> {
    conn.build_transaction()
        .run(|txn| {
            Box::pin(async move {
                let tags = models::Tag::get_all(txn, pattern).await?;

                Ok(tags
                    .into_iter()
                    .map(|tag| objects::Tag {
                        uuid: tag.uuid,
                        name: tag.name,
                        colour: tag.colour,
                        black_text: tag.black_text,
                        description: tag.description,
                    })
                    .collect())
            })
        })
        .await
}
