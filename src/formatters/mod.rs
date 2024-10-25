use std::{io, rc::Rc};

use crate::models::Error;
use crate::models::Format;
use crate::models::Graph;
use crate::models::Requirement;
use crate::models::TracedGraph;
use serde::Serialize;
use serde::Serializer as _;
use serde_json::Serializer as _;

mod gnuerr;
// TODO: mod markdown;
// TODO: mod serialize;
mod tags;

pub fn graph<W>(graph: &Graph, format: &Format, writer: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    match format {
        // Format::Markdown => markdown::graph(graph, writer),
        // Format::Json => serialize::graph(graph, format, writer),
        _ => todo!(),
    }
}

pub fn requirements<W>(graph: &Graph, format: &Format, writer: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    match format {
        Format::Tags => tags::requirements(graph, writer),
        //       Format::Markdown => markdown::requirements(reqs, writer),
        //       Format::Json     => serde_json::to_writer_pretty(writer, &serialize::Requirements::new(reqs))
        //           .map_err(io::Error::other),
        _ => todo!(),
    }
}

pub fn errors<'e, E, W>(errors: E, format: &Format, writer: &mut W) -> io::Result<()>
where
    W: io::Write,
    E: Iterator<Item = &'e Error>,
{
    match format {
        //     Format::GnuError => gnuerr::errors(errors, writer),
        //     Format::Json => serde_json::to_writer_pretty(writer, &serialize::Errors::new(errors))
        //         .map_err(io::Error::other),
        _ => todo!(),
    }
}

pub fn tracing<W>(traced_graph: &TracedGraph, format: &Format, writer: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    match format {
        //    Format::Markdown => markdown::tracing(tracing, graph, writer),
        Format::GnuError => gnuerr::tracing(traced_graph, writer),
        //    Format::Json => {
        //        serde_json::to_writer_pretty(writer, &serialize::Trace::new(tracing, graph))
        //            .map_err(io::Error::other)
        //    }
        _ => todo!(),
    }
}
