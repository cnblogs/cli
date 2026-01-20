use clap::Parser;
use cnblogs_lib::context::Context;
use cnblogs_lib::{commands::Cli, logic};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut ctx = Context::new()?;
    ctx.json = cli.json;
    logic::run(cli, &mut ctx).await?;
    Ok(())
}
