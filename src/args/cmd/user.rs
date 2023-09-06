use clap::Parser;

#[derive(Parser, Debug)]
pub struct Opt {
    #[arg(verbatim_doc_comment)]
    /// Login with your personal access token (PAT)
    /// Example: cnb user --login FOOBARBAZ
    /// PAT will be saved in ~/.cnbrc
    /// You can create PAT in https://account.cnblogs.com/tokens
    #[arg(long)]
    #[arg(value_name = "PAT")]
    pub login: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Logout and remove ~/.cnbrc
    /// Example: cnb user --logout
    #[arg(long)]
    pub logout: bool,

    #[arg(verbatim_doc_comment)]
    /// Show user info
    /// Example: cnb user --info
    #[arg(long)]
    #[arg(short = 'i')]
    pub info: bool,
}
