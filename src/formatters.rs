use std::{io, path::PathBuf};
use serde::{Deserialize, Serialize};

use crate::common::{Format, ParserError, Requirement};

pub mod tags;
pub mod gnuerr;


pub fn requirements<'r, W, R>(requiremens: R, format: &Format, writer: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r Requirement>,
{
    match format {
        Format::Tags => { tags::requirements(requiremens, writer) },
        _ => todo!()
    }
}

pub fn errors<'r, W, R>(errors: R, format: &Format, writer: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r ParserError>
{
    match format {
        Format::Tags => { gnuerr::errors(errors, writer) },
        _ => todo!()
    }
}
