use clap::Parser;
use posts::Posts;

pub mod posts;

#[derive(Parser)]
pub enum Commands {
    /// Manage posts's Wordpress
    Posts(Posts),
}
