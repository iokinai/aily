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
}
