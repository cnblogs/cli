use clap::Parser;

#[derive(Parser, Debug)]
#[non_exhaustive]
pub struct Opt {
    #[arg(verbatim_doc_comment)]
    /// Show favorite list, order by time in DESC
    ///   Example: cnb fav --list
    #[arg(long)]
    #[arg(short = 'l')]
    pub list: bool,
}
