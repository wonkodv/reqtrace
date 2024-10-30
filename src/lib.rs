//! Requirement Tracing Lib

#![cfg_attr(not(debug_assertions), warn(missing_docs))]
#![cfg_attr(debug_assertions, allow(dead_code))]

macro_rules! requirement_covered {
    ($id:ident) => {};
    ($id:ident,$title:literal) => {};
    ($id:literal,$title:literal) => {};
    ($id:ident:$title:literal) => {};
    ($id:literal:$title:literal) => {};
}

pub mod aggregator;
pub mod controller;
pub mod formatters;
pub mod models;
pub mod parsers;
pub mod trace;
pub mod util;
