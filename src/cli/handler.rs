use crate::cli::Cli;
use crate::handler::Handler;

pub struct CliHandler;

impl Handler for CliHandler{
    async fn run(_cli:&Cli) -> anyhow::Result<()> {
        println!("以cli模式运行了");
        todo!("暂时未实现")
    }
}