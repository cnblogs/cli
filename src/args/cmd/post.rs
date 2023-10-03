use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Opt {
    #[clap(verbatim_doc_comment)]
    /// Show title and content of a specific post
    ///   Example: cnb --id 114514 post --show
    ///     You should also specify the id of the post via --id
    #[arg(long)]
    #[arg(short = 's')]
    pub show: bool,

    #[arg(verbatim_doc_comment)]
    /// Show metadata of a specific post
    ///   Example: cnb --id 114514 post --show-meta
    ///     You should also specify the id of the post via --id
    ///     *
    #[arg(long)]
    #[arg(visible_alias = "sm")]
    pub show_meta: bool,

    #[arg(verbatim_doc_comment)]
    /// Show comment list of post, order by time in DESC
    ///   Example: cnb --id 114514 post --show-comment
    ///     You should also specify the id of the post via --id
    ///     *
    #[arg(long)]
    #[arg(visible_alias = "sc")]
    pub show_comment: bool,

    #[arg(verbatim_doc_comment)]
    /// Show post list, order by time in DESC
    ///   Example: cnb post --list
    ///     <LENGTH> should in range [0,100]
    ///     If <LENGTH> greater than 100, it will be set to 100
    #[arg(long)]
    #[arg(short = 'l')]
    pub list: bool,

    #[arg(verbatim_doc_comment)]
    /// Delete post
    ///   Example: cnb --id 114514 post --delete
    ///     You should also specify the id of the post via --id
    ///     *
    #[arg(long)]
    #[arg(visible_alias = "del")]
    pub delete: bool,

    #[command(subcommand)]
    pub cmd: Option<Cmd>,
}

#[derive(Parser, Debug)]
pub struct CreateCmd {
    #[arg(verbatim_doc_comment)]
    /// Set post title
    ///   Example: cnb post create --title 'Title' --body 'Body'
    #[arg(long)]
    #[arg(value_name = "TITLE")]
    pub title: String,

    #[arg(verbatim_doc_comment)]
    /// Set post body
    ///   Example: cnb post create --title 'Title' --body 'Body'
    #[arg(long)]
    #[arg(value_name = "BODY")]
    pub body: String,

    #[arg(verbatim_doc_comment)]
    /// Set post status to publish
    ///   Example: cnb post create --title 'Title' --body 'Body' --publish
    ///     *
    #[arg(long)]
    #[arg(visible_alias = "pub")]
    pub publish: bool,
}

#[derive(Parser, Debug)]
pub struct UpdateCmd {
    #[arg(verbatim_doc_comment)]
    /// Set post title
    ///   Example: cnb --id 114514 post update --title 'Title'
    #[arg(long)]
    #[arg(value_name = "TITLE")]
    pub title: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Set post body
    ///   Example: cnb --id 114514 post update --body 'Body'
    #[arg(long)]
    #[arg(value_name = "BODY")]
    pub body: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Set post publish state
    ///   Example: cnb --id 114514 post update --publish true
    ///     *
    #[arg(long)]
    #[arg(value_name = "BOOL")]
    #[arg(visible_alias = "pub")]
    pub publish: Option<bool>,
}

#[derive(Parser, Debug)]
pub struct SearchCmd {
    #[arg(verbatim_doc_comment)]
    /// Search self post
    ///   Example: cnb post search --self 'Keyword'
    #[arg(long)]
    #[arg(long = "self")]
    #[arg(value_name = "KEYWORD")]
    pub self_keyword: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Search site post
    ///   Example: cnb post search --site 'Keyword'
    #[arg(long)]
    #[arg(long = "site")]
    #[arg(value_name = "KEYWORD")]
    pub site_keyword: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    #[clap(verbatim_doc_comment)]
    /// Create post
    /// Example: cnb post create --title 'Title' --body 'Body'
    ///   *
    #[clap(visible_alias = "c")]
    Create(CreateCmd),

    #[clap(verbatim_doc_comment)]
    /// Update post
    /// Example: cnb --id 114514 post update --title 'Title'
    ///   You should also specify the id of the post via --id
    ///   *
    #[clap(visible_alias = "u")]
    Update(UpdateCmd),

    #[clap(verbatim_doc_comment)]
    /// Search post
    /// Example: cnb post search --self 'Keyword'
    ///   *
    #[clap(visible_alias = "s")]
    Search(SearchCmd),
}
