use std::io;
use std::rc::Rc;

use serde::Serialize;
use serde::Serializer as _;
use serde_json::Serializer as _;

use crate::aggregator::AggregatedGraph;
use crate::models::Error;
use crate::models::Format;
use crate::models::Graph;
use crate::models::Requirement;
use crate::models::TracedGraph;

mod gnuerr;
mod markdown;
mod serialize;
mod tags;

// pub fn graph(graph: &Graph, format: &Format, writer: &mut impl io::Write) -> io::Result<()> {
//    requirement_covered!(DSG_FORMATTER);
//     match format {
//     //    Format::Markdown => markdown::graph(graph, writer),
//         // Format::Json => serialize::graph(graph, format, writer),
//         _ => todo!(),
//     }
// }

pub fn requirements(graph: &Graph, format: &Format, writer: &mut impl io::Write) -> io::Result<()> {
    requirement_covered!(DSG_FORMATTER);
    match format {
        Format::Tags => tags::requirements(graph, writer),
        Format::Markdown => markdown::requirements(graph, writer),
        //       Format::Json     => serde_json::to_writer_pretty(writer, &serialize::Requirements::new(reqs))
        //           .map_err(io::Error::other),
        _ => todo!(),
    }
}

// pub fn errors<'e, E, W>(graph: &Graph,  format: &Format, writer: &mut W) -> io::Result<()>
// where
//     W: io::Write,
// {
//      requirement_covered!(DSG_FORMATTER);
//     match format {
//         Format::Markdown => markdown::errors(graph, writer),
//         Format::GnuError => gnuerr::errors(errors, writer),
//         //     Format::Json => serde_json::to_writer_pretty(writer, &serialize::Errors::new(errors))
//         //         .map_err(io::Error::other),
//         _ => todo!(),
//     }
// }

pub fn tracing(
    traced_graph: &TracedGraph,
    format: &Format,
    writer: &mut impl io::Write,
) -> io::Result<()> {
    requirement_covered!(DSG_FORMATTER);
    match format {
        Format::GnuError => gnuerr::tracing(traced_graph, writer),
        Format::Markdown => {
            let aggregated_graph = AggregatedGraph::new(traced_graph);
            markdown::tracing(&aggregated_graph, writer)
        }
        Format::Json => serde_json::to_writer_pretty(writer, &serialize::Trace::new(traced_graph))
            .map_err(io::Error::other),
        _ => todo!(),
    }
}
