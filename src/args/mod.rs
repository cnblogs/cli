pub mod cmd;
pub mod parser;

use crate::args::cmd::Cmd;
use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, Parser, ValueEnum)]
pub enum Style {
    Colorful,
    Normal,
    Json,
}

#[derive(Clone, Debug, Parser, ValueEnum)]
pub enum TimeStyle {
    Friendly,
    Normal,
}

#[derive(Parser, Debug)]
#[non_exhaustive]
pub struct GlobalOpt {
    #[arg(verbatim_doc_comment)]
    /// Execute with specific PAT
    ///   Example: cnb --with-pat 'FOOBARBAZ' post --list
    ///     Your PAT in ~/.cnbrc will be ignored in this execution if it exists
    ///     Please login if you don't want to input PAT everytime, try 'cnb user --help' for more details
    #[arg(long)]
    #[arg(value_name = "PAT")]
    pub with_pat: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Execute in debug mode, this will print some messages for the developer
    ///   Example: cnb --debug ing list
    ///     THIS OPTION IS UNSTABLE FOREVER and any output from it may change in the future
    ///     You should NEVER rely on the output while you turn this option on
    ///     *
    #[arg(long)]
    #[clap(visible_alias = "dbg")]
    pub debug: bool,

    #[arg(verbatim_doc_comment)]
    /// Configure the output style
    ///   Example: cnb --style json ing list
    ///     *
    #[arg(long)]
    #[arg(value_enum)]
    #[arg(hide_possible_values = true)]
    #[arg(default_value_t = Style::Colorful)]
    #[arg(value_name = "NAME")]
    pub style: Style,

    #[arg(verbatim_doc_comment)]
    /// Configure the time style
    ///   Example: cnb --style normal ing list
    ///     This option does not affect the output of '--style json'
    ///     *
    #[arg(long)]
    #[arg(value_enum)]
    #[arg(hide_possible_values = true)]
    #[arg(default_value_t = TimeStyle::Friendly)]
    #[arg(value_name = "NAME")]
    pub time_style: TimeStyle,

    #[arg(verbatim_doc_comment)]
    /// Fail if error occurred
    ///   Example: cnb --fail-on-error ing list
    ///     *
    #[arg(long)]
    #[clap(visible_alias = "foe")]
    #[arg(default_value_t = false)]
    pub fail_on_error: bool,

    #[arg(verbatim_doc_comment)]
    /// Suppress all output
    ///   Example: cnb --quiet ing list
    ///     *
    #[arg(long)]
    #[clap(visible_alias = "silent")]
    #[arg(default_value_t = false)]
    pub quiet: bool,
}

#[derive(Parser, Debug)]
#[command(author, about, long_about = None, version)]
#[non_exhaustive]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Option<Cmd>,
    #[clap(flatten)]
    pub global_opt: GlobalOpt,

    #[arg(verbatim_doc_comment)]
    /// Provide ID required by other options
    ///   Example: cnb --id 114514 post --show
    #[arg(long)]
    pub id: Option<usize>,

    #[arg(verbatim_doc_comment)]
    /// Reverse list output
    ///   Example: cnb --rev ing list
    #[arg(long)]
    pub rev: bool,

    #[arg(verbatim_doc_comment)]
    /// Skip items while request list
    ///   Example: cnb --skip 2 ing list
    ///     Use this option to save network I/O if some items of the list output are not needed
    ///     If this option is required but not specified, it will be set to 0
    #[arg(long)]
    #[arg(short = 's')]
    #[arg(value_name = "LENGTH")]
    pub skip: Option<usize>,

    #[arg(verbatim_doc_comment)]
    /// Take items while request list
    ///   Example: cnb --take 2 ing list
    ///     Use this option to save network I/O if only a subset of the list output are required
    ///     <LENGTH> should be in the range [0,100]
    ///     If <LENGTH> is greater than 100, it will be set to 100
    ///     If this option is required but not specified, it will be set to 8
    #[arg(long)]
    #[arg(short = 't')]
    #[arg(value_name = "LENGTH")]
    pub take: Option<usize>,
}
