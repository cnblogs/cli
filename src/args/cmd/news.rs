use clap::Parser;

#[derive(Parser, Debug)]
pub struct Opt {
    #[arg(verbatim_doc_comment)]
    /// Show news list, order by time in DESC
    ///   Example: cnb news --list
    #[arg(long)]
    #[arg(short = 'l')]
    pub list: bool,
}
