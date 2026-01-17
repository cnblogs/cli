use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum NewsAction {
    List,
}
