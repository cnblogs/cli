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
    pub ing_list: Option<usize>,

    #[arg(verbatim_doc_comment)]
    /// Publish ing with specific content
    #[arg(long)]
    #[arg(value_name = "CONTENT")]
    pub pub_ing: Option<String>,

    /*    /// Show post details
        /// You should also specify the ID of post via option --id
        #[arg(long)]
        #[arg(verbatim_doc_comment)]
        pub show_post: Option<String>,


        /// Comment ing with specific content
        #[arg(long)]
        #[arg(num_args = 2)]
        #[arg(value_names = ["ING_ID", "COMMENT"])]
        #[arg(verbatim_doc_comment)]
        pub comment_ing: Option<Vec<String>>,

    */
}
