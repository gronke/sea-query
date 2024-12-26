use sea_query::{tests_cfg::*, *};

mod exception;
mod foreign_key;
mod index;
mod query;
mod table;
mod unsupported;

#[path = "../common.rs"]
mod common;
use common::*;
