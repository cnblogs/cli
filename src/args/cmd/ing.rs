use clap::Parser;

#[derive(Parser, Debug)]
pub struct Opt {
    #[arg(verbatim_doc_comment)]
    /// Show ing list, order by time in DESC
    /// Example: cnb ing --list
    #[arg(long)]
    #[arg(short = 'l')]
    pub list: bool,

    #[arg(verbatim_doc_comment)]
    /// Publish ing with specific content
    /// Example: cnb ing --publish 'Hello world'
    /// The visibility of ing is public
    #[arg(long)]
    #[arg(short = 'p')]
    #[arg(visible_alias = "pub")]
    #[arg(value_name = "CONTENT")]
    pub publish: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Comment ing with specific content
    /// Example: cnb --id 114514 ing --comment 'Hello world'
    /// You should also specify the id of the ing via --id
    #[arg(long)]
    #[arg(short = 'c')]
    #[arg(value_name = "CONTENT")]
    pub comment: Option<String>,
}
