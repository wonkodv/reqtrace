use super::super::common::*;
use crate::graph::Graph;
use crate::{
    errors,
    trace::{TracedRequirement, Tracing},
};
use serde::ser::{SerializeMap, Serializer};
use serde_json::json;
use std::{collections::HashSet, io, rc::Rc};

const VERSION: u32 = 1;

pub fn requirements<'r, S, R>(reqs: R, format: &Format, s: &mut S) -> Result<(), S::Error>
where
    R: Iterator<Item = &'r Rc<Requirement>>,
    S: Serializer,
{
    todo!();
}
