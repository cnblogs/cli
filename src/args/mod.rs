pub mod parser;
pub mod sub_cmd;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Subcommand)]
pub enum SubCmds {
    /// User operations
    #[clap(visible_alias = "u")]
    User(sub_cmd::User),
    /// Ing operations
    #[clap(visible_alias = "i")]
    Ing(sub_cmd::Ing),
    /// Post operations
    #[clap(visible_alias = "p")]
    Post(sub_cmd::Post),
}

#[derive(Clone, Debug, Parser, ValueEnum)]
pub enum Style {
    Colorful,
    Normal,
    Json,
}

#[derive(Debug, Parser)]
#[command(author, about, long_about = None, version)]
pub struct Args {
    #[command(subcommand)]
    command: Option<SubCmds>,

    #[arg(verbatim_doc_comment)]
    /// Provide ID required by other options
    /// Example: cnb --id 114514 post --show
    #[arg(long)]
    pub id: Option<usize>,

    #[arg(verbatim_doc_comment)]
    /// Execute with specific PAT
    /// Example: cnb --with-pat FOOBARBAZ post --list
    /// Your PAT in ~/.cnbrc will be ignored in this execution if it exists
    /// Please login if you don't want to input PAT everytime, try 'cnb user --help' for more details
    #[arg(long)]
    #[arg(value_name = "PAT")]
    pub with_pat: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Reverse list output
    /// Example: cnb --rev ing --list
    #[arg(long)]
    pub rev: bool,

    #[arg(verbatim_doc_comment)]
    /// Skip items while request list
    /// Example: cnb --skip 2 ing --list
    /// Use this option to save network I/O if some items of the list output are not needed
    /// If this option is required but not specified, it will be set to 0
    #[arg(long)]
    #[arg(short = 's')]
    #[arg(value_name = "LENGTH")]
    pub skip: Option<usize>,

    #[arg(verbatim_doc_comment)]
    /// Take items while request list
    /// Example: cnb --take 2 ing --list
    /// Use this option to save network I/O if only a subset of the list output are required
    /// <LENGTH> should be in the range [0,100]
    /// If <LENGTH> is greater than 100, it will be set to 100
    /// If this option is required but not specified, it will be set to 8
    #[arg(long)]
    #[arg(short = 't')]
    #[arg(value_name = "LENGTH")]
    pub take: Option<usize>,

    #[arg(verbatim_doc_comment)]
    /// Execute in debug mode, this will print some messages for the developer
    /// Example: cnb --debug ing --list
    /// THIS OPTION IS UNSTABLE FOREVER and any output from it may change in the future
    /// You should NEVER rely on the output while you turn this option on
    #[arg(long)]
    #[clap(visible_alias = "dbg")]
    pub debug: bool,

    #[arg(verbatim_doc_comment)]
    /// Configure the output style
    #[arg(long)]
    #[arg(value_enum)]
    #[arg(hide_possible_values = true)]
    #[arg(default_value = "colorful")]
    #[arg(value_name = "NAME")]
    pub style: Style,
}
