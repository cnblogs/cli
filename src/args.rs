use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Login with your personal access token (PAT)
    /// PAT will be saved in ~/.cnbrc
    /// You can create PAT in https://account.cnblogs.com/tokens
    #[arg(long = "login")]
    #[arg(value_name = "PAT")]
    #[arg(verbatim_doc_comment)]
    pub login: Option<String>,

    /// Logout and remove ~/.cnbrc
    #[arg(long = "logout")]
    #[arg(verbatim_doc_comment)]
    pub logout: bool,

    /// Show user info
    #[arg(long = "user-info")]
    #[arg(short = 'u')]
    #[arg(verbatim_doc_comment)]
    pub user_info: bool,

    /*    /// Show post details
        /// You should also specify the ID of post via option --id
        #[arg(long)]
        #[arg(verbatim_doc_comment)]
        pub show_post: Option<String>,

        /// Show ing list, order by time in DESC
        /// <LENGTH> should in range [0,20]
        #[arg(long)]
        #[arg(short = 'i')]
        #[arg(verbatim_doc_comment)]
        pub list_ing: Option<usize>,

        /// Publish ing with specific content
        /// The privilege of ing is public
        #[arg(long)]
        #[arg(verbatim_doc_comment)]
        pub pub_ing: Option<Vec<String>>,

        /// Comment ing with specific content
        #[arg(long)]
        #[arg(num_args = 2)]
        #[arg(value_names = ["ING_ID", "COMMENT"])]
        #[arg(verbatim_doc_comment)]
        pub comment_ing: Option<Vec<String>>,

    */
}
