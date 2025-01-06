use crate::client::{Content, Post, Title, WordPressClientOpts};
use crate::subcommands::WordPressClientCommand;
use anyhow::Result;
use clap::Parser;

const POST_PATH: &str = "posts";

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

        let response = reqwest::Client::new()
            .get(post_url)
            .basic_auth(
                self.word_press_client_opts
                    .username
                    .clone()
                    .unwrap_or("NOT_DEFINED".into()),
                self.word_press_client_opts.password.clone(),
            )
            .send()
            .await?;

        Ok(response.json::<Vec<Post>>().await?)
    }
}

pub struct WordPressPostCreat {
    word_press_client_opts: WordPressClientOpts,
    post: Post,
}

impl WordPressPostCreat {
    pub fn new(word_press_client_opts: WordPressClientOpts, post: Post) -> Self {
        Self {
            word_press_client_opts,
            post,
        }
    }
}

impl WordPressClientCommand<Post> for WordPressPostCreat {
    async fn execute(&self) -> Result<Post> {
        let post_url = format!("{}/{}", self.word_press_client_opts.base_url, POST_PATH);

        let response = reqwest::Client::new()
            .post(post_url)
            .basic_auth(
                self.word_press_client_opts
                    .username
                    .clone()
                    .unwrap_or("NOT_DEFINED".into()),
                self.word_press_client_opts.password.clone(),
            )
            .json(&self.post)
            .send()
            .await?;

        Ok(response.json::<Post>().await?)
    }
}

#[derive(Parser, Debug)]
pub enum PostsSubcommand {
    /// List posts from WordPress
    List,
    /// Import posts to WordPress
    Create {
        /// The post's title
        #[arg(value_name = "title", short = 't', long = "title")]
        title: String,
        /// The post's content
        #[arg(value_name = "content", short = 'c', long = "content")]
        content: String,
    },
}

#[derive(Parser)]
pub struct PostCommand {
    #[clap(subcommand)]
    pub subcommand: PostsSubcommand,
}

impl PostCommand {
    pub async fn run(&self, word_press_client_opts: WordPressClientOpts) -> Result<()> {
        match &self.subcommand {
            PostsSubcommand::List => {
                let posts = WordPressPostList::new(word_press_client_opts)
                    .execute()
                    .await?;
                println!("{:#?}", posts);
                Ok(())
            }
            PostsSubcommand::Create { title, content } => {
                let post = WordPressPostCreat::new(
                    word_press_client_opts,
                    Post {
                        id: None,
                        title: Title {
                            raw: Some(title.into()),
                            rendered: None,
                        },
                        content: Content {
                            raw: Some(content.into()),
                            rendered: None,
                        },
                        status: "publish".to_string(),
                    },
                )
                .execute()
                .await?;
                println!("{:#?}", post);
                Ok(())
            }
        }
    }
}
