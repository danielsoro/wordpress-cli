use clap::Parser;

#[derive(Parser, Debug)]
pub enum PostsSubcommand {
    /// List posts from wordpress
    List,
    /// Import posts to wordpress
    Import,
}

#[derive(Parser)]
pub struct Posts {
    #[clap(subcommand)]
    pub subcommand: PostsSubcommand,
}

impl Posts {
    pub fn run(&self) {
        match &self.subcommand {
            PostsSubcommand::List => self.list(),
            PostsSubcommand::Import => self.import(),
        }
    }

    fn list(&self) {
        println!("List posts");
    }

    fn import(&self) {
        println!("Import posts");
    }
}

#[derive(Parser)]
pub enum PostCommands {
    /// Manage posts from wordpress
    Posts(Posts),
}
