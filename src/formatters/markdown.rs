use super::super::common::*;
use std::{collections::HashSet, io, rc::Rc};

use crate::{
    errors::Error,
    trace::{Graph, Tracing},
};
use Error::*;

pub fn requirements<'r, W, R>(reqs: R, w: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r Rc<Requirement>>,
{
    for req in reqs {
        writeln!(
            w,
            "\n## {}: {}\n\nOrigin: `{}:{}`",
            req.id,
            req.title.as_ref().unwrap_or(&"".to_owned()),
            req.location.file.display(),
            req.location.line
        )?;

        if !req.covers.is_empty() {
            writeln!(w, "\nCovers:")?;
            for c in &req.covers {
                if let Some(title) = &c.title {
                    writeln!(w, "*   {}: {}", c.id, title)?;
                } else {
                    writeln!(w, "*   {}", c.id,)?;
                }
            }
        }
        if !req.depends.is_empty() {
            writeln!(w, "\nDepends:")?;
            for d in &req.depends {
                if let Some(title) = &d.title {
                    writeln!(w, "*   {}: {}", d.id, title)?;
                } else {
                    writeln!(w, "*   {}", d.id,)?;
                }
            }
        }

        for (k, v) in &req.attributes {
            let v = v.trim();
            if !v.is_empty() {
                writeln!(w, "\n{}:", k)?;
                writeln!(w, "{}", v)?;
            }
        }
    }
    Ok(())
}

pub fn err<'r, W>(error: &'r Error, w: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    match error {
        FormatError(loc, err) => {
            writeln!(w, "{}:{}: {}", loc.file.display(), loc.line, err)?;
        }
        DuplicateRequirement(r1, r2) => {
            writeln!(
                w,
                "{}:{}: Duplicate Requirement {} previously seen at {}:{}",
                r2.location.file.display(),
                r2.location.line,
                r1.id,
                r1.location.file.display(),
                r1.location.line,
            )?;
        }
        DuplicateAttribute(loc, attr) => {
            writeln!(
                w,
                "{}:{}: Duplicate Attribute {}",
                loc.file.display(),
                loc.line,
                attr,
            )?;
        }
        IoError(path, err) => {
            writeln!(w, "{}: IO Error: {}", path.display(), err,)?;
        }
        UnknownArtefactType(t) => {
            writeln!(w, "Unknown Artefact Type {}", t)?;
        }
        ConfigError(s) => {
            writeln!(w, "Config  Error: {:?}", s)?;
        }
        DuplicateArtefact(a) => {
            writeln!(w, "Duplicate Artefact: {}", a)?;
        }
        UnknownArtefact(a) => {
            writeln!(w, "Unknown Artefact: {}", a)?;
        }
        UnknownEdge(from, to) => {
            writeln!(w, "Unknown Edge {} -> {}", from, to)?;
        }
        CoveredWithWrongTitle(r1, r2, wrong_title) => {
            writeln!(
                w,
                "{}:{}: {} covered with wrong title \n\texpected: {}\n\tactual  : {}",
                r1.location.file.display(),
                r1.location.line,
                r1.id,
                r2.title.as_ref().unwrap_or(&"<no title>".to_owned()),
                wrong_title,
            )?;
        }
        DependWithWrongTitle(r1, r2, wrong_title) => {
            writeln!(
                w,
                "{}:{}: {} depend with wrong title:\n\texpected: {}\n\tactual  : {}",
                r1.location.file.display(),
                r1.location.line,
                r1.id,
                r2.title.as_ref().unwrap_or(&"<no title>".to_owned()),
                wrong_title,
            )?;
        }

        OnlyOnePathExpected | CombinedTracingsWithIntersectingEdges | EmptyGraph => {
            writeln!(w, "{:?}", error)?;
        }
    }
    Ok(())
}

pub fn errors<'r, W, R>(errors: R, w: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r Error>,
{
    writeln!(w, "# Errors")?;
    for e in errors {
        err(e, w)?;
    }

    Ok(())
}

pub fn tracing<'r, W>(tracing: &Tracing, g: &Graph, w: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    let mut set: HashSet<Vec<u8>> = HashSet::new();
    {
        // Parsing Errors
        let mut header_written = false;
        for e in g.get_parsing_errors() {
            let mut s = Vec::new();
            err(e, &mut s)?;
            if set.insert(s.clone()) {
                if !header_written {
                    header_written = true;
                    writeln!(w, "")?;
                    writeln!(w, "")?;
                    writeln!(w, "# Artefacts Errors")?;
                    writeln!(w, "")?;
                }
                w.write_all(&s)?;
            }
        }
    }
    {
        // Tracing Errors
        let mut header_written = false;
        for e in tracing.errors() {
            let mut s = Vec::new();
            err(e, &mut s)?;
            if set.insert(s.clone()) {
                if !header_written {
                    header_written = true;
                    writeln!(w, "")?;
                    writeln!(w, "")?;
                    writeln!(w, "# Tracing Errors")?;
                    writeln!(w, "")?;
                }
                w.write_all(&s)?;
            }
        }
    }

    {
        // Uncovered
        let mut uncovered: Vec<_> = tracing.uncovered().collect();
        uncovered.sort_unstable_by_key(|r| &r.id);

        if !uncovered.is_empty() {
            writeln!(w, "")?;
            writeln!(w, "")?;
            writeln!(w, "# Uncovered Requirements")?;
            writeln!(w, "")?;

            requirements(uncovered.into_iter(), w)?;
        }
    }

    {
        // Derived
        let mut derived: Vec<_> = tracing.derived().collect();
        derived.sort_unstable_by_key(|r| &r.id);

        if !derived.is_empty() {
            writeln!(w, "")?;
            writeln!(w, "")?;
            writeln!(w, "# Derived Requirements")?;
            writeln!(w, "")?;

            requirements(derived.into_iter(), w)?;
        }
    }

    {
        // Covered
        writeln!(w, "")?;
        writeln!(w, "")?;
        writeln!(w, "# Covered Requirements")?;
        writeln!(w, "")?;
        let mut covered: Vec<_> = tracing.coverages().collect();
        covered.sort_unstable_by_key(|(upper, _lower)| &upper.id);

        requirements(covered.into_iter().map(|(r1, _r2)| r1), w)?;
    }

    Ok(())
}
