use std::{error::Error, time::Duration};

use aily_models::LoadDanbooruPost;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{loader::Loader, options::AilyModeOptions};

fn is_nsfw(post: &LoadDanbooruPost) -> bool {
    post.rating() != "g"
}

fn is_ai_generated(post: &LoadDanbooruPost) -> bool {
    post.tag_string().contains("ai-generated")
}

fn serialize_posts_into_sql(posts: &Vec<LoadDanbooruPost>) -> Vec<String> {
    let mut id_counter = 1;
    posts.iter().filter_map(|post| {
        if let None = post.file_url() {
            return None;
        }

        Some(
            {
                let res = format!(
                    "INSERT INTO artworks (id, danbooru_id, path, is_nsfw) VALUES ({}, {}, '{}', {});\nINSERT INTO aily_posts(artwork_id, is_ai_generated) VALUES ({}, {});",
                    id_counter,
                    post.id(),
                    post.file_url().clone().unwrap(),
                    is_nsfw(post),
                    id_counter,
                    is_ai_generated(post),
                );
                id_counter += 1;
                res
            }
        )

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

pub async fn run(
    options: &AilyModeOptions,
    loader: &Loader,
    output: &String,
) -> Result<(), Box<dyn Error>> {
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
        .write(false)
        .append(true)
        .open(output.clone())
        .await?;

    file.write(sql_string.as_bytes()).await?;

    Ok(())
}
