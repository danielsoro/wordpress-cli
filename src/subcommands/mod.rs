use clap::Parser;
use posts::PostCommand;

pub mod posts;

trait WordPressClientCommand<T> {
    async fn execute(&self) -> anyhow::Result<T>;
}

#[derive(Parser)]
pub enum Commands {
    /// Manage post's WordPress
    Posts(PostCommand),
}
