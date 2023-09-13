use crate::api::ing::IngType;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Opt {
    #[command(subcommand)]
    pub cmd: Option<Cmd>,

    #[arg(verbatim_doc_comment)]
    /// Publish ing with specific content
    ///   Example: cnb ing --publish 'Hello world'
    ///     The visibility of ing is public
    ///     *
    #[arg(long)]
    #[arg(short = 'p')]
    #[arg(visible_alias = "pub")]
    #[arg(value_name = "CONTENT")]
    pub publish: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Comment ing with specific content
    ///   Example: cnb --id 114514 ing --comment 'Hello world'
    ///     You should also specify the id of the ing via --id
    #[arg(long)]
    #[arg(short = 'c')]
    #[arg(value_name = "CONTENT")]
    pub comment: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    #[clap(verbatim_doc_comment)]
    /// Show ing list, order by time in DESC
    /// Example: cnb ing list
    ///   *
    #[clap(visible_alias = "l")]
    List {
        #[arg(verbatim_doc_comment)]
        /// Ing type to show
        ///   Example: cnb ing list --type myself
        ///     *
        #[arg(long)]
        #[arg(value_name = "TYPE")]
        #[arg(default_value = "public")]
        r#type: Option<IngType>,
    },
}
