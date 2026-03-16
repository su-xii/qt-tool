use anyhow::Result;
use qt_tool::cli::Cli;
use clap::Parser;
#[tokio::main]
async fn main() -> Result<()>{
    if let Err(e) = Cli::parse().run().await{
        eprintln!("{}",e)
    }
    Ok(())
}
