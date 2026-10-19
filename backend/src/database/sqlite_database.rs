use std::error::Error;

use aily_models::{AilyPost, Artwork, Tag, TagsPost};
use async_trait::async_trait;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, SelectableHelper, SqliteConnection,
};
use diesel_async::{
    RunQueryDsl,
    pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool},
    sync_connection_wrapper::SyncConnectionWrapper,
};
use log::error;

use crate::{
    database::{Database, db_aily_post::DbAilyPost, db_artwork::DbArtwork, db_tag::DbTag},
    errors::{AilyError, NotFound},
    schema::{aily_posts, artwork_tag, artworks, tags},
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
    async fn get_random_artwork(&self, include_nsfw: bool) -> Result<Artwork, AilyError> {
        let mut conn = self.pool.get().await.map_err(|_| AilyError::Unknown)?;

        let mut query = artworks::table.select(DbArtwork::as_select()).into_boxed();

        if !include_nsfw {
            query = query.filter(artworks::is_nsfw.eq(false));
        }

        let posts = query
            .order(diesel::dsl::sql::<diesel::sql_types::Integer>("RANDOM()"))
            .limit(1)
            .load::<DbArtwork>(&mut conn)
            .await
            .inspect_err(|e| error!("Error while getting random post: {}", e))
            .map_err(|_| AilyError::Unknown)?;

        let artwork = posts
            .into_iter()
            .next()
            .ok_or(AilyError::NotFound(NotFound::Artwork))?;

        Ok(Artwork::from(&artwork))
    }

    async fn get_aily_tags_artwork(&self, include_nsfw: bool) -> Result<Artwork, AilyError> {
        let mut conn = self.pool.get().await.map_err(|_| AilyError::Unknown)?;

        let mut query = artwork_tag::table
            .inner_join(artworks::table)
            .select(DbArtwork::as_select())
            .into_boxed();

        if !include_nsfw {
            query = query.filter(artworks::is_nsfw.eq(false));
        }

        let result = query
            .order(diesel::dsl::sql::<diesel::sql_types::Integer>("RANDOM()"))
            .limit(1)
            .load::<DbArtwork>(&mut conn)
            .await
            .map_err(|_| AilyError::Unknown)?;

        let artwork = result
            .into_iter()
            .next()
            .ok_or(AilyError::NotFound(NotFound::Artwork))?;

        Ok(Artwork::from(&artwork))
    }

    async fn get_artworks_by_ids(&self, ids: &[i32]) -> Result<Vec<Artwork>, AilyError> {
        let mut conn = self
            .pool
            .get()
            .await
            .inspect_err(|e| error!("Could not get connection from pool: {}", e))
            .map_err(|_| AilyError::Unknown)?;

        let artworks = artworks::table
            .select(DbArtwork::as_select())
            .filter(artworks::id.eq_any(ids))
            .load::<DbArtwork>(&mut conn)
            .await
            .map_err(|_| AilyError::Unknown)?;

        Ok(artworks.iter().map(|a| Artwork::from(a)).collect())
    }

    async fn get_posts_by_artwork_ids(&self, ids: &[i32]) -> Result<Vec<AilyPost>, AilyError> {
        let mut conn = self.pool.get().await.map_err(|_| AilyError::Unknown)?;

        let posts = aily_posts::table
            .inner_join(artworks::table)
            .select((DbAilyPost::as_select(), DbArtwork::as_select()))
            .filter(aily_posts::artwork_id.eq_any(ids))
            .load::<(DbAilyPost, DbArtwork)>(&mut conn)
            .await
            .map_err(|_| AilyError::Unknown)?;

        Ok(posts
            .iter()
            .map(|(post, artwork)| {
                AilyPost::new(
                    post.id,
                    post.artwork_id,
                    artwork.danbooru_id,
                    post.is_ai_generated,
                    artwork.is_nsfw,
                    artwork.path.clone(),
                )
            })
            .collect())
    }

    async fn get_post_by_artwork_id(&self, id: i32) -> Result<AilyPost, AilyError> {
        let mut conn = self.pool.get().await.map_err(|_| AilyError::Unknown)?;

        let posts = aily_posts::table
            .inner_join(artworks::table)
            .select((DbAilyPost::as_select(), DbArtwork::as_select()))
            .filter(aily_posts::artwork_id.eq(id))
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

    async fn make_tags_guess(&self, tag: &String, artwork_id: i32) -> Result<bool, AilyError> {
        let mut conn = self.pool.get().await.map_err(|_| AilyError::Unknown)?;

        let ids = tags::table
            .select(tags::id)
            .filter(tags::tag_name.eq(tag))
            .load::<i32>(&mut conn)
            .await
            .map_err(|_| AilyError::Unknown)?;

        let id = ids
            .into_iter()
            .next()
            .ok_or(AilyError::NotFound(NotFound::Tag))?;

        let result = artwork_tag::table
            .select(artwork_tag::id)
            .filter(
                artwork_tag::artwork_id
                    .eq(artwork_id)
                    .and(artwork_tag::tag_id.eq(id)),
            )
            .load::<i32>(&mut conn)
            .await
            .map_err(|_| AilyError::Unknown)?;

        if result.is_empty() {
            return Ok(false);
        }

        Ok(true)
    }

    async fn get_tags_list(&self) -> Result<Vec<Tag>, AilyError> {
        let mut conn = self.pool.get().await.map_err(|_| AilyError::Unknown)?;

        let tags = tags::table
            .select(DbTag::as_select())
            .load::<DbTag>(&mut conn)
            .await
            .map_err(|_| AilyError::Unknown)?;

        Ok(tags.iter().map(|t| Tag::from(t)).collect())
    }

    async fn get_tags_post_by_artwork_id(&self, id: i32) -> Result<TagsPost, AilyError> {
        let mut conn = self.pool.get().await.map_err(|_| AilyError::Unknown)?;

        let res = artwork_tag::table
            .inner_join(artworks::table)
            .inner_join(tags::table)
            .select((DbArtwork::as_select(), DbTag::as_select()))
            .filter(artworks::id.eq(id))
            .load::<(DbArtwork, DbTag)>(&mut conn)
            .await
            .map_err(|_| AilyError::Unknown)?;

        if res.is_empty() {
            return Err(AilyError::NotFound(NotFound::Artwork));
        }

        let tags = res.iter().map(|(_, tag)| Tag::from(tag)).collect();
        let db_artwork = res[0].0.clone();
        let artwork = Artwork::from(&db_artwork);

        Ok(TagsPost::from((artwork, tags)))
    }
}
