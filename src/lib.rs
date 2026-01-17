#![feature(try_blocks)]
#![feature(if_let_guard)]
#![feature(iterator_try_collect)]
#![feature(iterator_try_reduce)]
#![warn(clippy::nursery, clippy::cargo_common_metadata)]

pub mod api;
pub mod api_bak;
pub mod apis;
pub mod args;
pub mod commands;
pub mod context;
pub mod display;
pub mod infra;
pub mod logic;
pub mod models;
pub mod tools;
