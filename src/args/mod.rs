pub mod parser;
pub mod sub_cmd;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Option<SubCmds>,

    #[arg(verbatim_doc_comment)]
    /// Provide ID required by other options
    #[arg(long)]
    pub id: Option<usize>,

    #[arg(verbatim_doc_comment)]
    /// Execute with specific PAT
    /// Your PAT in ~/.cnbrc will be ignored in this execution if it exists
    #[arg(long)]
    #[arg(value_name = "PAT")]
    pub with_pat: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Reverse list output, such as --list-ing
    #[arg(long)]
    pub rev: bool,

    #[arg(verbatim_doc_comment)]
    /// Skip items while request list, such as --list-ing
    #[arg(long)]
    #[arg(value_name = "LENGTH")]
    #[arg(default_value = "0")]
    pub skip: usize,

    #[arg(verbatim_doc_comment)]
    /// Execute in debug mode, this will print some messages for developer
    /// THIS OPTION IS UNSTABLE FOREVER, any output from it maybe change in the future
    /// You should NEVER rely on the output while you turn this option on
    #[arg(long)]
    pub debug: bool,
}

#[derive(Subcommand, Debug)]
pub enum SubCmds {
    /// User operations
    User(sub_cmd::User),
    /// Ing operations
    Ing(sub_cmd::Ing),
    /// Post operations
    Post(sub_cmd::Post),
}
