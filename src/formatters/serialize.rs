use super::super::common::*;
use std::{collections::HashSet, io, rc::Rc};

use crate::graph::Graph;
use crate::{errors::Error, trace::Tracing};
use Error::*;

use lazy_static::lazy_static;
use regex::Regex;
use serde::ser::SerializeStruct;
use serde::{ser, Serialize, Serializer};
use serde_json::json;

use crate::trace::TracedRequirement;

const VERSION: u32 = 1;

pub fn requirements<'r, R, S>(reqs: R, serializer: S) -> Result<S::Ok, S::Error>
where
    R: Iterator<Item = &'r Rc<Requirement>>,
    S: Serializer,
{
    let requirements: Vec<&'r Requirement> = reqs.map(|r| &**r).collect();
    let mut s = serializer.serialize_struct("requirements", 2)?;
    s.serialize_field("version", &VERSION)?;
    s.serialize_field("requirements", &requirements)?;
    s.end()
}
