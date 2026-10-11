use std::{error::Error, time::Duration};

use aily_models::LoadDanbooruPost;
use clap::Parser;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{loader::Loader, options::Options};

mod load_error;
mod loader;
mod options;

static TABLE: &str = r#"CREATE TABLE Arts(
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    danbooru_id INTEGER,
    is_nsfw BOOLEAN NOT NULL,
    is_ai_generated BOOLEAN NOT NULL,
    is_ai_assisted BOOLEAN NOT NULL,
    path TEXT NOT NULL
);"#;

fn is_nsfw(post: &LoadDanbooruPost) -> bool {
    post.rating() != "g"
}

fn is_ai_generated(post: &LoadDanbooruPost) -> bool {
    post.tag_string().contains("ai-generated")
}

fn is_ai_assisted(post: &LoadDanbooruPost) -> bool {
    post.tag_string().contains("ai-assisted")
}

fn serialize_posts_into_sql(posts: &Vec<LoadDanbooruPost>) -> Vec<String> {
    posts.iter().filter_map(|post| {
        if let None = post.file_url() {
            return None;
        }

        Some(format!(
            "INSERT INTO Arts (danbooru_id, is_nsfw, is_ai_generated, is_ai_assisted, path) VALUES ({}, {}, {}, {}, '{}');",
            post.id(),
            is_nsfw(post),
            is_ai_generated(post),
            is_ai_assisted(post),
            post.file_url().clone().unwrap(),
        ))
    }).collect()
}

async fn load_posts_with_tags(
    tags: String,
    count: usize,
    loader: &Loader,
) -> Result<Vec<LoadDanbooruPost>, Box<dyn Error>> {
    let mut result = Vec::new();
    result.reserve(count);
    let mut page = 1;

    while result.len() < count as usize {
        let batch = loader.load_posts(tags.clone(), 200, page).await?;

        if batch.is_empty() {
            break;
        }

        result.extend(batch);
        page += 1;

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Ok(result)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::parse();

    let loader = Loader::new(options.id);

    let mut result: Vec<LoadDanbooruPost> = Vec::new();
    result.reserve(options.count as usize);

    result.extend(
        load_posts_with_tags(
            "-video -ai-generated".to_string(),
            (options.count / 2) as usize,
            &loader,
        )
        .await?,
    );

    result.extend(
        load_posts_with_tags(
            "-video ai-generated".to_string(),
            (options.count / 2) as usize,
            &loader,
        )
        .await?,
    );

    let sql_string = serialize_posts_into_sql(&result).join("\n");

    let mut file = File::options()
        .create(true)
        .write(true)
        .open(options.output)
        .await?;

    file.write(TABLE.as_bytes()).await?;
    file.write("\n".as_bytes()).await?;
    file.write(sql_string.as_bytes()).await?;

    Ok(())
}
