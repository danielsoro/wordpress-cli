mod subcommands;
mod client;

use clap::Parser;
use crate::client::WordPressOpts;

/// WordPress CLI help you to manage your WordPress instance from your command line
#[derive(Parser)]
#[command(version, about)]
pub struct WordpressCli {
    #[clap(subcommand)]
    subcommand: subcommands::Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = WordpressCli::parse();
    let word_press_opts = WordPressOpts::new("https://test.latpf.org/wp-json/wp/v2".into());
    match args.subcommand {
        subcommands::Commands::Posts(post_command) => post_command.run(word_press_opts).await?,
    }
    Ok(())
}
