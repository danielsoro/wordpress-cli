mod wordpress;

use clap::Parser;

/// WordPress CLI help you to manage your WordPress instance from your command line
#[derive(Parser)]
#[command(version, about)]
pub struct WordpressCli {
    #[clap(subcommand)]
    subcommand: wordpress::Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = WordpressCli::parse();
    match args.subcommand {
        wordpress::Commands::Posts(posts) => posts.run().await?,
    }
    
    Ok(())
}
