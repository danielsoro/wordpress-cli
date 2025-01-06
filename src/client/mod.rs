use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub raw: Option<String>,
    pub rendered: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Title {
    pub raw: Option<String>,
    pub rendered: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Option<u32>,
    pub title: Title,
    pub content: Content,
    pub status: String,
}

#[derive(Clone, Debug)]
pub struct WordPressClientOpts {
    pub base_url: String,
    pub username: Option<String>,
    pub password: Option<String>,
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
