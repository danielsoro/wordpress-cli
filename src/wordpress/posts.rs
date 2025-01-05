use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Content {
    pub rendered: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Title {
    pub rendered: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Post {
    pub id: u32,
    pub title: Title,
    pub content: Content,
}

#[derive(Parser, Debug)]
pub enum PostsSubcommand {
    /// List posts from WordPress
    List,
    /// Import posts to WordPress
    Import,
}

#[derive(Parser)]
pub struct Posts {
    #[clap(subcommand)]
    pub subcommand: PostsSubcommand,
}

impl Posts {
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>>  {
        match &self.subcommand {
            PostsSubcommand::List => self.list().await,
            PostsSubcommand::Import => self.import().await,
        }
    }

    async fn list(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!("Should move all HTTP call to a WordPress HTTP Client");
        let posts = reqwest::get("url")
            .await?.json::<Vec<Post>>().await?;

        if posts.is_empty() {
            println!("There are no posts.");
            return Ok(());
        }

        println!("{:#?}", posts);
        Ok(())
    }

    async fn import(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
