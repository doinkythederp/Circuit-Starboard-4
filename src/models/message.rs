use anyhow::Result;
use sqlx::{query_as, PgPool};

pub struct Message {
    pub message_id: i64,
    pub guild_id: i64,
    pub channel_id: i64,
    pub author_id: i64,

    pub is_nsfw: bool,

    pub forced_to: Vec<i32>,
    pub trashed: bool,
    pub trash_reason: Option<String>,
    pub frozen: bool,
}

impl Message {
    pub async fn create(
        pool: &PgPool,
        message_id: i64,
        guild_id: i64,
        channel_id: i64,
        author_id: i64,
        is_nsfw: bool,
    ) -> Result<Self> {
        query_as!(
            Self,
            r#"INSERT INTO messages (message_id, guild_id, channel_id,
                author_id, is_nsfw)
            VALUES ($1, $2, $3, $4, $5) RETURNING *"#,
            message_id,
            guild_id,
            channel_id,
            author_id,
            is_nsfw,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e.into())
    }
}