use clap::Parser;

#[derive(Parser, Debug)]
pub struct Opt {
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
    #[arg(default_missing_value = "8")]
    #[arg(value_name = "LENGTH")]
    pub list: bool,

    #[arg(verbatim_doc_comment)]
    /// Delete post
    /// You should also specify the id of post via option --id
    #[arg(long)]
    #[arg(visible_alias = "del")]
    pub delete: bool,

    #[arg(verbatim_doc_comment)]
    /// Search post by keyword and output the post id list that matches
    /// Example: cnb post --search FOO
    #[arg(long)]
    #[arg(value_name = "KEYWORD")]
    pub search: Option<String>,
}
