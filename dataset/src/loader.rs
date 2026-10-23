use std::error::Error;

use aily_models::LoadDanbooruPost;
use reqwest::Client;

use crate::load_error::LoadError;

static DANBOORU_URL: &str = "https://danbooru.donmai.us/posts.json?";

pub struct Loader {
    danbooru_id: u64,
    danbooru_url: String,
    client: Client,
}

impl Loader {
    pub fn new(danbooru_id: u64) -> Loader {
        Loader {
            danbooru_id,
            danbooru_url: DANBOORU_URL.to_string(),
            client: Client::new(),
        }
    }

    pub async fn load_posts(
        &self,
        tags: String,
        count: u32,
        page: u32,
    ) -> Result<Vec<LoadDanbooruPost>, Box<dyn Error>> {
        let path = format!(
            "{}tags={}&limit={}&page={}",
            self.danbooru_url, tags, count, page
        );

        let user_agent_value = format!("Aily/1.0 (user #{})", self.danbooru_id);
        let response = self
            .client
            .get(path)
            .header("Accept", "application/json")
            .header("User-Agent", user_agent_value.as_str())
            .send()
            .await?;

        if response.status().is_success() {
            let posts = response.json::<Vec<LoadDanbooruPost>>().await?;
            Ok(posts)
        } else {
            println!("{}", response.text().await.unwrap());
            Err(Box::new(LoadError))
        }
    }

    pub async fn load_tags_by_ids(
        &self,
        ids: &[u64],
    ) -> Result<Vec<(u64, Vec<String>)>, Box<dyn Error>> {
        let mut all_tags = Vec::new();
        let user_agent_value = format!("Aily/1.0 (user #{})", self.danbooru_id);

        for chunk in ids.chunks(200) {
            let ids_str = chunk
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");

            let path = format!(
                "{}tags=id:{}&limit={}",
                self.danbooru_url,
                ids_str,
                chunk.len()
            );

            let response = self
                .client
                .get(&path)
                .header("Accept", "application/json")
                .header("User-Agent", user_agent_value.as_str())
                .send()
                .await?;

            if response.status().is_success() {
                let posts = response.json::<Vec<LoadDanbooruPost>>().await?;
                for post in posts {
                    if !post.tag_string_general().is_empty() {
                        let tags = post
                            .tag_string_general()
                            .split_whitespace()
                            .map(String::from)
                            .collect::<Vec<_>>();
                        all_tags.push((*post.id(), tags));
                    }
                }
            } else {
                let err_text = response.text().await.unwrap_or_default();
                println!("Error while loading batch: {}", err_text);
                return Err(Box::new(LoadError));
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }

        Ok(all_tags)
    }
}
