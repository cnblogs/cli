pub mod parser;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(verbatim_doc_comment)]
    /// Login with your personal access token (PAT)
    /// PAT will be saved in ~/.cnbrc
    /// You can create PAT in https://account.cnblogs.com/tokens
    #[arg(long)]
    #[arg(value_name = "PAT")]
    pub login: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Logout and remove ~/.cnbrc
    #[arg(long = "logout")]
    pub logout: bool,

    #[arg(verbatim_doc_comment)]
    /// Show user info
    #[arg(long)]
    #[arg(short = 'u')]
    pub user_info: bool,

    #[arg(verbatim_doc_comment)]
    /// Show ing list, order by time in DESC
    /// <LENGTH> should in range [0,100]
    /// If <LENGTH> greater than 100, it will be set to 100
    #[arg(long)]
    #[arg(short = 'i')]
    #[arg(value_name = "LENGTH")]
    #[arg(num_args = 0..=1)]
    #[arg(default_missing_value = "8")]
    pub list_ing: Option<usize>,

    #[arg(verbatim_doc_comment)]
    /// Publish ing with specific content
    /// The visibility of ing is public
    #[arg(long)]
    #[arg(value_name = "CONTENT")]
    pub pub_ing: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Comment ing with specific content
    /// You should also specify the id of ing via option --id
    #[arg(long)]
    #[arg(value_name = "CONTENT")]
    pub comment_ing: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Provide ID required by other options
    #[arg(long)]
    pub id: Option<usize>,

    #[arg(verbatim_doc_comment)]
    /// Run with specific PAT
    /// Your PAT in ~/.cnbrc will be ignored in this execution if it exists
    #[arg(long)]
    #[arg(value_name = "PAT")]
    pub with_pat: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Show title and content of a specific post
    /// You should also specify the id of post via option --id
    #[arg(long)]
    pub show_post: bool,

    #[arg(verbatim_doc_comment)]
    /// Show metadata of a specific post
    /// You should also specify the id of post via option --id
    #[arg(long)]
    pub show_post_meta: bool,
}
