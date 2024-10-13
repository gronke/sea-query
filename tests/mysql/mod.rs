use sea_query::{extension::mysql::*, tests_cfg::*, *};

mod foreign_key;
mod index;
mod query;
mod table;
mod if_else;

#[path = "../common.rs"]
mod common;
use common::*;
