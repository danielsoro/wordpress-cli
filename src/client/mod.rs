use anyhow::Result;
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

#[derive(Clone)]
pub struct WordPressClientOpts {
    base_url: String,
    username: Option<String>,
    password: Option<String>,
}

impl WordPressClientOpts {
    pub fn builder() -> WordPressClientOptsBuilder {
        WordPressClientOptsBuilder::default()
    }
}

#[derive(Default)]
pub struct WordPressClientOptsBuilder {
    base_url: String,
    username: Option<String>,
    password: Option<String>,
}

impl WordPressClientOptsBuilder {
    pub fn base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    pub fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    pub fn password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }

    pub fn build(self) -> WordPressClientOpts {
        WordPressClientOpts {
            base_url: self.base_url,
            username: self.username,
            password: self.password,
        }
    }
}

pub trait WordPressClientCommand<T> {
    async fn execute(&self) -> Result<T>;
}

#[derive(Clone)]
pub struct WordPressPostList {
    word_press_client_opts: WordPressClientOpts,
}

impl WordPressPostList {
    pub fn new(word_press_client: WordPressClientOpts) -> Self {
        Self {
            word_press_client_opts: word_press_client,
        }
    }
}

impl WordPressClientCommand<Vec<Post>> for WordPressPostList {
    async fn execute(&self) -> Result<Vec<Post>> {
        let post_url = format!("{}/{}", self.word_press_client_opts.base_url, POST_PATH);

        let posts = reqwest::Client::new()
            .get(post_url)
            .basic_auth(
                self.word_press_client_opts
                    .username.clone()
                    .unwrap_or("NOT_DEFINED".into()),
                self.word_press_client_opts
                    .password.clone()
            )
            .send()
            .await?
            .json::<Vec<Post>>()
            .await?;
        Ok(posts)
    }
}
