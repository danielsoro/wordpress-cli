use crate::client::{Command, PostCommandList, WordPressOpts};
use clap::Parser;
use anyhow::Result;

#[derive(Parser, Debug)]
pub enum PostsSubcommand {
    /// List posts from WordPress
    List,
    /// Import posts to WordPress
    Import,
}

#[derive(Parser)]
pub struct PostCommand {
    #[clap(subcommand)]
    pub subcommand: PostsSubcommand,
}

impl PostCommand {
    pub async fn run(&self, word_press_client: WordPressOpts) -> Result<()> {
        match &self.subcommand {
            PostsSubcommand::List => {
                let posts = PostCommandList::new(word_press_client)
                .execute()
                .await?;
                println!("{:#?}", posts);
                Ok(())
            }
            PostsSubcommand::Import => Ok(()),
        }
    }
}
