use std::{io, rc::Rc};

use crate::{
    common::{Format, Requirement},
    graph::Graph,
    trace::Tracing,
};

use crate::errors::Error;

mod gnuerr;
mod markdown;
mod serialize;
mod tags;

pub fn graph<W>(graph: &Graph, format: &Format, writer: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    match format {
        //   Format::Markdown => markdown::graph(graph, writer),
        // TODO      Format::Json => serialize::graph(graph, format, writer),
        _ => todo!(),
    }
}

pub fn requirements<'r, W, R>(requirements: R, format: &Format, writer: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r Rc<Requirement>>,
{
    match format {
        Format::Tags => tags::requirements(requirements, writer),
        Format::Markdown => markdown::requirements(requirements, writer),
        Format::Json => {
            let mut ser = serde_json::Serializer::pretty(writer);
            match serialize::requirements(requirements, &mut ser) {
                Ok(o) => Ok(()),
                Err(err) => todo!(),
            }
        }
        _ => todo!(),
    }
}

pub fn errors<'r, W, R>(errors: R, format: &Format, writer: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r Error>,
{
    match format {
        Format::GnuError => gnuerr::errors(errors, writer),

        // TODO      Format::Json => serialize::errors(errors, format, writer),
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
        // TODO      Format::Json => serialize::tracing(tracing, graph, format, writer),
        _ => todo!(),
    }
}
