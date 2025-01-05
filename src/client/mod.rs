use serde::{Deserialize, Serialize};

const POST_PATH: &str = "posts";

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub rendered: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Title {
    pub rendered: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: u32,
    pub title: Title,
    pub content: Content,
}

pub struct WordPressOpts {
    pub base_url: String,
}

impl WordPressOpts {
    pub fn new(base_url: String) -> Self {
        WordPressOpts {
            base_url,
        }
    }
}

pub trait Command<T> {
    async fn execute(&self) -> Result<T, anyhow::Error>;
}

pub struct PostCommandList {
    word_press_opts: WordPressOpts,
}

impl PostCommandList {
    pub fn new(word_press_client: WordPressOpts) -> Self {
        Self { word_press_opts: word_press_client }
    }
}

impl Command<Vec<Post>> for PostCommandList {
    async fn execute(&self) -> Result<Vec<Post>, anyhow::Error> {
        let posts = reqwest::get(format!("{}/{}", self.word_press_opts.base_url.clone(), POST_PATH))
            .await?.json::<Vec<Post>>().await?;
        Ok(posts)
    }
}