use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Opt {
    #[arg(verbatim_doc_comment)]
    /// Show title and content of a specific post
    /// Example: cnb --id 114514 post --show
    /// You should also specify the id of the post via --id
    #[arg(long)]
    #[arg(short = 's')]
    pub show: bool,

    #[arg(verbatim_doc_comment)]
    /// Show metadata of a specific post
    /// Example: cnb --id 114514 post --show-meta
    /// You should also specify the id of the post via --id
    #[arg(long)]
    #[arg(short = 'm')]
    pub show_meta: bool,

    #[arg(verbatim_doc_comment)]
    /// Show post list, order by time in DESC
    /// Example: cnb post --list
    /// <LENGTH> should in range [0,100]
    /// If <LENGTH> greater than 100, it will be set to 100
    #[arg(long)]
    #[arg(short = 'l')]
    pub list: bool,

    #[arg(verbatim_doc_comment)]
    /// Delete post
    /// Example: cnb --id 114514 post --delete
    /// You should also specify the id of the post via --id
    #[arg(long)]
    #[arg(visible_alias = "del")]
    pub delete: bool,

    #[arg(verbatim_doc_comment)]
    /// Search post by keyword and output the post id list that matches
    /// Example: cnb post --search 'Hello world'
    #[arg(long)]
    #[arg(short = 'f')]
    #[arg(visible_alias = "find")]
    #[arg(value_name = "KEYWORD")]
    pub search: Option<String>,

    #[command(subcommand)]
    pub cmd: Option<Cmd>,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// Create post
    /// Example: cnb post create --title 'Title' --body 'Body'
    #[clap(visible_alias = "c")]
    Create {
        #[arg(verbatim_doc_comment)]
        /// Set post title
        /// Example: cnb post create --title 'Title' --body 'Body'
        #[arg(long)]
        #[arg(value_name = "TITLE")]
        title: String,

        #[arg(verbatim_doc_comment)]
        /// Set post body
        /// Example: cnb post create --title 'Title' --body 'Body'
        #[arg(long)]
        #[arg(value_name = "BODY")]
        body: String,

        #[arg(verbatim_doc_comment)]
        /// Set post status to publish
        /// Example: cnb post create --title 'Title' --body 'Body' --publish
        #[arg(long)]
        #[arg(visible_alias = "pub")]
        publish: bool,
    },
    /// Update post
    /// Example: cnb --id 114514 post update --title 'Title'
    /// You should also specify the id of the post via --id
    #[clap(visible_alias = "u")]
    Update {
        #[arg(verbatim_doc_comment)]
        /// Set post title
        /// Example: cnb --id 114514 post update --title 'Title'
        #[arg(long)]
        #[arg(value_name = "TITLE")]
        title: Option<String>,

        #[arg(verbatim_doc_comment)]
        /// Set post body
        /// Example: cnb --id 114514 post update --body 'Body'
        #[arg(long)]
        #[arg(value_name = "BODY")]
        body: Option<String>,

        #[arg(verbatim_doc_comment)]
        /// Set post publish state
        /// Example: cnb --id 114514 post update --publish true
        #[arg(long)]
        #[arg(visible_alias = "pub")]
        publish: Option<bool>,
    },
}
