use crate::{
    api::ing::{IngSendFrom, IngType},
    apis::{
        self,
        ing::{IngContent, QeurySet},
    },
};
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[non_exhaustive]
pub struct Opt {
    #[command(subcommand)]
    pub cmd: Option<Cmd>,

    #[arg(verbatim_doc_comment)]
    /// Publish ing with specific content
    ///   Example: cnb ing --publish 'Hello world'
    ///     The visibility of ing is public
    ///     *
    #[arg(long)]
    #[arg(short = 'p')]
    #[arg(visible_alias = "pub")]
    #[arg(value_name = "CONTENT")]
    pub publish: Option<String>,

    #[arg(verbatim_doc_comment)]
    /// Comment ing with specific content
    ///   Example: cnb --id 114514 ing --comment 'Hello world'
    ///     You should also specify the id of the ing via --id
    #[arg(long)]
    #[arg(short = 'c')]
    #[arg(value_name = "CONTENT")]
    pub comment: Option<String>,
}

#[derive(Debug, Subcommand)]
#[non_exhaustive]
pub enum Cmd {
    #[clap(verbatim_doc_comment)]
    /// Show ing list, order by time in DESC
    /// Example: cnb ing list
    ///   *
    #[clap(visible_alias = "l")]
    List {
        #[arg(verbatim_doc_comment)]
        /// Ing type to show
        ///   Example: cnb ing list --type myself
        ///     *
        #[arg(long)]
        #[arg(value_name = "TYPE")]
        #[arg(default_value = "public")]
        r#type: Option<IngType>,

        #[arg(verbatim_doc_comment)]
        /// Align ing content to user name automatically
        ///   Example: cnb ing list --align
        #[arg(long)]
        #[arg(value_name = "BOOL")]
        #[arg(default_value_t = true)]
        align: bool,
    },

    /// 根据条件查询闪存。
    ///
    Query(QueryIng),

    /// 创建闪存
    Create(CreateIng),

    /// 根据ID删除闪存
    Delete { id: Vec<u64> },
}

#[derive(Debug, Args)]
pub struct CreateIng {
    /// 闪存内容
    pub content: String,

    /// 是否私有，默认是全站
    #[arg(short, long, default_value_t = false)]
    pub private: bool,

    /// 是否发布为幸运
    #[arg(short, long, default_value_t = false)]
    pub lucky: bool,

    /// 是否发布在某个标签下，默认不发布标签。
    #[arg(short, long, default_value = "")]
    pub tag: String,
}

impl From<CreateIng> for IngContent {
    fn from(value: CreateIng) -> Self {
        let mut cont = String::new();
        if !value.tag.is_empty() {
            cont.push_str(format!("[{}]", value.tag).as_str())
        }
        cont.push_str(value.content.as_str());
        Self {
            content: cont,
            is_private: value.private,
            lucky: value.lucky,
            client_type: IngSendFrom::Cli,
        }
    }
}

/// 查询参数
/// 当使用type为
#[derive(Debug, Args, Clone)]
pub struct QueryIng {
    /// 查询类型
    #[arg(
        short,
        long,
        value_name = "TYPE",
        default_value_t = QueryType::F,
        default_missing_value = "f",
        value_enum
    )]
    pub r#type: QueryType,
    /// 分页查询，起始索引是1
    #[arg(short('n'), long, default_value_t = 1)]
    pub page_index: u64,
    /// 分页查询数量， 默认是10
    #[arg(short('s'), long, default_value_t = 10)]
    pub page_size: u64,
    /// 按照标签查询
    #[arg(short('g'), long)]
    pub tag: Option<String>,
    /// 根据ID查询
    #[arg(short, long)]
    pub id: Option<Vec<u64>>,
}

impl From<&QueryIng> for QeurySet {
    fn from(value: &QueryIng) -> Self {
        Self {
            r#type: value.r#type.clone().into(),
            page_index: value.page_index,
            page_size: value.page_size,
            tag: value.tag.clone().unwrap_or_default(),
        }
    }
}

/// 过滤的类型
#[derive(Debug, Clone, ValueEnum, Parser)]
pub enum QueryType {
    /// 关注
    F = 1,
    /// 我的
    My = 4,
    /// 全站
    P = 5,
    /// 新回应
    Rc = 6,
    /// 我回应
    Mc = 7,
    /// 按照Tag过滤，使用T时，如果没有Query::tag或者站点不存在，则不会有结果。
    T = 10,
    /// 回复我
    C = 13,
    /// 提到我
    M = 14,
}

impl From<QueryType> for apis::ing::QueryIngType {
    fn from(value: QueryType) -> Self {
        match value {
            QueryType::F => Self::Following,
            QueryType::My => Self::My,
            QueryType::P => Self::All,
            QueryType::Rc => Self::RecentComment,
            QueryType::Mc => Self::MyComment,
            QueryType::T => Self::Tag,
            QueryType::C => Self::Comment,
            QueryType::M => Self::Mention,
        }
    }
}
