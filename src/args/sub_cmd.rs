use clap::Parser;

#[derive(Parser, Debug)]
pub struct User {
    #[arg(verbatim_doc_comment)]
    /// Login with your personal access token (PAT)
    /// PAT will be saved in ~/.cnbrc
    /// You can create PAT in https://account.cnblogs.com/tokens
    #[arg(long)]
    #[arg(value_name = "PAT")]
    pub login: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Logout and remove ~/.cnbrc
    #[arg(long)]
    pub logout: bool,

    #[arg(verbatim_doc_comment)]
    /// Show user info
    #[arg(long)]
    #[arg(short = 'i')]
    pub info: bool,
}

#[derive(Parser, Debug)]
pub struct Ing {
    #[arg(verbatim_doc_comment)]
    /// Show ing list, order by time in DESC
    /// <LENGTH> should in range [0,100]
    /// If <LENGTH> greater than 100, it will be set to 100
    #[arg(long)]
    #[arg(short = 'l')]
    #[arg(value_name = "LENGTH")]
    #[arg(num_args = 0..=1)]
    #[arg(default_missing_value = "8")]
    pub list: Option<usize>,

    #[arg(verbatim_doc_comment)]
    /// Publish ing with specific content
    /// The visibility of ing is public
    /// This option has the alias --pub
    #[arg(long)]
    #[arg(short = 'p')]
    #[arg(alias = "pub")]
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

#[derive(Parser, Debug)]
pub struct Post {
    #[arg(verbatim_doc_comment)]
    /// Show title and content of a specific post
    /// You should also specify the id of post via option --id
    #[arg(long)]
    #[arg(short = 's')]
    pub show: bool,

    #[arg(verbatim_doc_comment)]
    /// Show metadata of a specific post
    /// You should also specify the id of post via option --id
    #[arg(long)]
    #[arg(short = 'm')]
    pub show_meta: bool,

    #[arg(verbatim_doc_comment)]
    /// Show post list, order by time in DESC
    /// <LENGTH> should in range [0,100]
    /// If <LENGTH> greater than 100, it will be set to 100
    #[arg(long)]
    #[arg(short = 'l')]
    #[arg(value_name = "LENGTH")]
    #[arg(num_args = 0..=1)]
    #[arg(default_missing_value = "8")]
    pub list: Option<usize>,
}
