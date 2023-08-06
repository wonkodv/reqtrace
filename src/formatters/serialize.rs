use super::super::common::*;
use std::rc::Rc;

use crate::graph::Graph;
use crate::trace::Tracing;

use serde::ser::SerializeStruct;
use serde::Serializer;

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
pub fn tracing<S>(tracing: &Tracing<'_>, graph: &Graph, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    todo!()
}
