use anyhow::Result;
use qt_tool::cli::Cli;
use clap::Parser;
#[tokio::main]
async fn main() -> Result<()>{
    Cli::parse().run().await
}
