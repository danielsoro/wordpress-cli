use clap::Parser;
use posts::PostCommand;

pub mod posts;

#[derive(Parser)]
pub enum Commands {
    /// Manage post's WordPress
    Posts(PostCommand),
}


