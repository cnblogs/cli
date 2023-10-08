pub mod fav;
pub mod ing;
pub mod news;
pub mod post;
pub mod user;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
#[non_exhaustive]
pub enum Cmd {
    /// User operations
    #[clap(visible_alias = "u")]
    User(user::Opt),
    /// Ing operations
    #[clap(visible_alias = "i")]
    Ing(ing::Opt),
    /// Post operations
    #[clap(visible_alias = "p")]
    Post(post::Opt),
    /// News operations
    #[clap(visible_alias = "n")]
    News(news::Opt),
    /// Favorite operations
    #[clap(visible_alias = "f")]
    Fav(fav::Opt),
}
