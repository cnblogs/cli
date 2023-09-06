use clap::Parser;

#[derive(Parser, Debug)]
pub struct Opt {
    #[arg(verbatim_doc_comment)]
    /// Show ing list, order by time in DESC
    #[arg(long)]
    #[arg(short = 'l')]
    pub list: bool,

    #[arg(verbatim_doc_comment)]
    /// Publish ing with specific content
    /// The visibility of ing is public
    #[arg(long)]
    #[arg(short = 'p')]
    #[arg(visible_alias = "pub")]
    #[arg(value_name = "CONTENT")]
    pub publish: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Comment ing with specific content
    /// You should also specify the id of ing via option --id
    #[arg(long)]
    #[arg(short = 'c')]
    #[arg(value_name = "CONTENT")]
    pub comment: Option<String>,
}
