use super::super::common::*;
use std::{collections::HashSet, io, rc::Rc};

use crate::graph::Graph;
use crate::{errors::Error, trace::Tracing};
use Error::*;

use lazy_static::lazy_static;
use regex::Regex;
use serde::ser;
use serde_json::json;

use crate::trace::TracedRequirement;

const VERSION: u32 = 1;

fn get_serizlizer<W>(format: &Format, w: &mut W) -> Box<dyn ser::Serializer>
where
    W: io::Write,
{
    match format {
        Format::Json => serde_json::Serializer::pretty(w),
        _ => unreachable!("no serializer for {format}"),
    }
}

// fn ser_requirement(r: &Requirement, ser: &dyn SerializeMap) -> io::Result<()> {}

pub fn requirements<'r, W, R>(reqs: R, format: &Format, w: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r Rc<Requirement>>,
{
    let s = get_serizlizer(format, w);
    let mut map = s.serialize_struct()?;
    {
        map.serialize_entry("version", VERSION);
        map.serialize_key("requirements")?;
        let mut reqs_map = map.serialize_map()?;
        {
            for req in reqs {
                rm.serialize_key(r.id)?;
                let mut req_map = rm.serialize_struct();
                req_map.serialize_str(req.id);
                req_map.serialize_str(req.title);
            }
        }
    }
}
