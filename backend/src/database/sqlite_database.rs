use std::error::Error;

use aily_models::GameDanbooruPost;
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, SqliteConnection};
use diesel_async::{
    RunQueryDsl,
    pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool},
    sync_connection_wrapper::SyncConnectionWrapper,
};

use crate::{
    database::{Database, db_danbooru_post::DbDanbooruPost},
    errors::AilyError,
    schema::Arts,
};

pub struct SqliteDatabase {
    pool: Pool<SyncConnectionWrapper<SqliteConnection>>,
}

impl SqliteDatabase {
    pub fn new(connection_string: String) -> Result<SqliteDatabase, Box<dyn Error>> {
        let config = AsyncDieselConnectionManager::<SyncConnectionWrapper<SqliteConnection>>::new(
            connection_string,
        );
        let pool = Pool::builder(config).max_size(10).build()?;

        Ok(SqliteDatabase { pool })
    }
}

#[async_trait]
impl Database for SqliteDatabase {
    async fn get_random_post(&self, include_nsfw: bool) -> Result<GameDanbooruPost, AilyError> {
        let mut conn = self.pool.get().await.map_err(|_| AilyError::Unknown)?;

        let mut query = Arts::table.select(Arts::all_columns).into_boxed();

        if !include_nsfw {
            query = query.filter(Arts::is_nsfw.eq(false));
        }

        let posts = query
            .order(diesel::dsl::sql::<diesel::sql_types::Integer>("RANDOM()"))
            .limit(1)
            .load::<DbDanbooruPost>(&mut conn)
            .await
            .map_err(|e| {
                println!("{}", e);
                AilyError::Unknown
            })?;

        let post = posts.into_iter().next().ok_or(AilyError::NotFound)?;

        Ok(GameDanbooruPost::from(&post))
    }

    async fn get_post_by_id(&self, id: i32) -> Result<GameDanbooruPost, AilyError> {
        let mut conn = self.pool.get().await.map_err(|_| AilyError::Unknown)?;

        let posts = Arts::table
            .select(Arts::all_columns)
            .filter(Arts::id.eq(id))
            .load::<DbDanbooruPost>(&mut conn)
            .await
            .map_err(|_| AilyError::Unknown)?;

        let post = posts.into_iter().next().ok_or(AilyError::NotFound)?;

        Ok(GameDanbooruPost::from(&post))
    }
}
