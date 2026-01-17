use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct PostCommand {
    #[clap(subcommand)]
    pub subcommands: PostAction,
}

#[derive(Debug, Subcommand)]
pub enum PostAction {
    Create,
    List,
    Update,
    Show,
}
