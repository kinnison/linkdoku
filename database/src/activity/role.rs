//! Role based activity such as update/creation of roles

use diesel_async::AsyncPgConnection;

use crate::models;

use super::{ActivityError, ActivityResult};

pub async fn update(
    conn: &mut AsyncPgConnection,
    actor: &str,
    role: &models::Role,
) -> ActivityResult<()> {
    // This activity requires permissions

    conn.build_transaction()
        .run(|txn| {
            Box::pin(async move {
                if !role.can_modify(txn, actor).await? {
                    Err(ActivityError::PermissionDenied)
                } else {
                    // Okay we're permitted to make the change, so let's go
                    role.save(txn).await.map_err(|e| e.into())
                }
            })
        })
        .await
}
