mod wordpress;
use clap::Parser;
use wordpress::posts::PostCommands;

#[derive(Parser)]
pub struct WordpressCli {
    #[clap(subcommand)]
    subcommand: PostCommands,
}

fn main() {
    let args = WordpressCli::parse();
    match args.subcommand {
        PostCommands::Posts(posts) => posts.run(),
    }
}
