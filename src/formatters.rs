use serde::{Deserialize, Serialize};
use std::{io, path::PathBuf, rc::Rc};

use crate::{
    common::{Format, Requirement},
};

use crate::errors::Error;
use Error::*;

pub mod gnuerr;
pub mod tags;

pub fn requirements<'r, W, R>(requiremens: R, format: &Format, writer: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r Rc<Requirement>>,
{
    match format {
        Format::Tags => tags::requirements(requiremens, writer),
        _ => todo!(),
    }
}

pub fn errors<'r, W, R>(errors: R, format: &Format, writer: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r Error>,
{
    match format {
        Format::Tags => gnuerr::errors(errors, writer),
        _ => todo!(),
    }
}
