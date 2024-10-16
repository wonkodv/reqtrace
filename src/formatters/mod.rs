use std::{io, rc::Rc};

use crate::{
    common::{Format, Requirement},
    graph::Graph,
    trace::Tracing,
};

use crate::errors::Error;
use serde::Serialize;
use serde::Serializer as _;
use serde_json::Serializer as _;

mod gnuerr;
mod markdown;
mod serialize;
mod tags;

pub fn graph<W>(graph: &Graph, format: &Format, writer: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    match format {
        Format::Markdown => {
            todo!();
            //markdown::graph(graph, writer)
        }
        Format::Json => {
            todo!();
            // serialize::graph(graph, format, writer)
        }
        _ => todo!(),
    }
}

pub fn requirements<'r, R, W>(reqs: R, format: &Format, writer: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r Rc<Requirement>>,
{
    match format {
        Format::Tags => tags::requirements(reqs, writer),
        Format::Markdown => markdown::requirements(reqs, writer),
        Format::Json => serde_json::to_writer_pretty(writer, &serialize::Requirements::new(reqs))
            .map_err(io::Error::other),
        _ => todo!(),
    }
}

pub fn errors<'e, E, W>(errors: E, format: &Format, writer: &mut W) -> io::Result<()>
where
    W: io::Write,
    E: Iterator<Item = &'e Error>,
{
    match format {
        Format::GnuError => gnuerr::errors(errors, writer),
        Format::Json => serde_json::to_writer_pretty(writer, &serialize::Errors::new(errors))
            .map_err(io::Error::other),
        _ => todo!(),
    }
}

pub fn tracing<W>(
    tracing: &Tracing<'_>,
    graph: &Graph,
    format: &Format,
    writer: &mut W,
) -> io::Result<()>
where
    W: io::Write,
{
    match format {
        Format::Markdown => markdown::tracing(tracing, graph, writer),
        Format::GnuError => gnuerr::tracing(tracing, graph, writer),
        Format::Json => {
            serde_json::to_writer_pretty(writer, &serialize::Trace::new(tracing, graph))
                .map_err(io::Error::other)
        }
        _ => todo!(),
    }
}
