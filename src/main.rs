mod wordpress;
use clap::Parser;

/// Wordpress CLI help you to manage your wordpress instace from your command line
#[derive(Parser)]
#[command(version, about)]
pub struct WordpressCli {
    #[clap(subcommand)]
    subcommand: wordpress::Commands,
}

fn main() {
    let args = WordpressCli::parse();
    match args.subcommand {
        wordpress::Commands::Posts(posts) => posts.run(),
    }
}
