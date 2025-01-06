mod client;
mod subcommands;

use crate::client::WordPressClientOpts;
use anyhow::Result;
use clap::Parser;

/// WordPress CLI help you to manage your WordPress instance from your command line
#[derive(Parser)]
#[command(version, about)]
pub struct WordpressCli {
    #[clap(subcommand)]
    subcommand: subcommands::Commands,
}

#[tokio::main]
async fn main() -> Result<()> {
    let word_press_opts = WordPressClientOpts::builder()
        .build();

    let args = WordpressCli::parse();
    match args.subcommand {
        subcommands::Commands::Posts(post_command) => post_command.run(word_press_opts).await?,
    }
    Ok(())
}
