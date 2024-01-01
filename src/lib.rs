#![feature(try_blocks)]
#![feature(if_let_guard)]
#![feature(let_chains)]
#![feature(iterator_try_collect)]
#![feature(iterator_try_reduce)]
#![warn(clippy::all, clippy::nursery, clippy::cargo_common_metadata)]

pub mod api;
pub mod apis;
pub mod args;
pub mod display;
pub mod infra;
