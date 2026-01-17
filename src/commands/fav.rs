use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum FaverateAction {
    List,
}
