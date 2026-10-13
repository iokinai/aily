use std::{collections::HashMap, error::Error};

use aily_models::{AilyPost, Session};
use async_trait::async_trait;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper, SqliteConnection};
use diesel_async::{
    RunQueryDsl,
    pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool},
    sync_connection_wrapper::SyncConnectionWrapper,
};
use log::error;

use crate::{
    database::{Database, db_aily_post::DbAilyPost, db_artwork::DbArtwork},
    errors::{AilyError, NotFound},
    game::GameSession,
    schema::{aily_posts, artworks},
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
    async fn get_random_post(&self, include_nsfw: bool) -> Result<AilyPost, AilyError> {
        let mut conn = self.pool.get().await.map_err(|_| AilyError::Unknown)?;

        let mut query = aily_posts::table
            .inner_join(artworks::table)
            .select((DbAilyPost::as_select(), DbArtwork::as_select()))
            .into_boxed();

        if !include_nsfw {
            query = query.filter(artworks::is_nsfw.eq(false));
        }

        let posts = query
            .order(diesel::dsl::sql::<diesel::sql_types::Integer>("RANDOM()"))
            .limit(1)
            .load::<(DbAilyPost, DbArtwork)>(&mut conn)
            .await
            .inspect_err(|e| error!("Error while getting random post: {}", e))
            .map_err(|_| AilyError::Unknown)?;

        let (post, artwork) = posts
            .into_iter()
            .next()
            .ok_or(AilyError::NotFound(NotFound::Artwork))?;

        Ok(AilyPost::new(
            post.id,
            post.artwork_id,
            artwork.danbooru_id,
            post.is_ai_generated,
            artwork.is_nsfw,
            artwork.path,
        ))
    }

    async fn get_post_by_id(&self, id: i32) -> Result<AilyPost, AilyError> {
        let mut conn = self.pool.get().await.map_err(|_| AilyError::Unknown)?;

        let posts = aily_posts::table
            .inner_join(artworks::table)
            .select((DbAilyPost::as_select(), DbArtwork::as_select()))
            .filter(aily_posts::id.eq(id))
            .load::<(DbAilyPost, DbArtwork)>(&mut conn)
            .await
            .map_err(|_| AilyError::Unknown)?;

        let (post, artwork) = posts
            .into_iter()
            .next()
            .ok_or(AilyError::NotFound(NotFound::Artwork))?;

        Ok(AilyPost::new(
            post.id,
            post.artwork_id,
            artwork.danbooru_id,
            post.is_ai_generated,
            artwork.is_nsfw,
            artwork.path,
        ))
    }

    async fn get_session(&self, sess: &GameSession) -> Result<Session, AilyError> {
        let mut ids = Vec::new();
        ids.reserve(sess.posts().len() + 1);

        ids.push(*sess.current());
        ids.extend(sess.posts().iter().map(|p| p.0));

        let mut conn = self
            .pool
            .get()
            .await
            .inspect_err(|e| error!("Could not get connection from pool: {}", e))
            .map_err(|_| AilyError::Unknown)?;

        let raw_posts = aily_posts::table
            .inner_join(artworks::table)
            .select((DbAilyPost::as_select(), DbArtwork::as_select()))
            .filter(aily_posts::id.eq_any(ids))
            .load::<(DbAilyPost, DbArtwork)>(&mut conn)
            .await
            .map_err(|_| AilyError::Unknown)?;

        let posts_map: HashMap<i32, AilyPost> = raw_posts
            .into_iter()
            .map(|(post, artwork)| {
                (
                    post.id,
                    AilyPost::new(
                        post.id,
                        post.artwork_id,
                        artwork.danbooru_id,
                        post.is_ai_generated,
                        artwork.is_nsfw,
                        artwork.path,
                    ),
                )
            })
            .collect();

        let current_id = *sess.current();
        let current = posts_map.get(&current_id).cloned().ok_or_else(|| {
            error!("Current post ID {} not found in database", current_id);
            AilyError::Unknown
        })?;

        let mut prev = Vec::with_capacity(sess.posts().len());
        for &(post_id, flag) in sess.posts().iter() {
            if let Some(game_post) = posts_map.get(&post_id) {
                prev.push((game_post.clone(), flag));
            }
        }

        Ok(Session::new(prev, current))
    }
}
