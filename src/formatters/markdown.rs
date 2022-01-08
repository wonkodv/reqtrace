use super::super::common::*;
use std::{collections::HashSet, io, rc::Rc};

use crate::graph::Graph;
use crate::{errors::Error, trace::Tracing};
use Error::*;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::trace::TracedRequirement;

lazy_static! {
    static ref REPLACE: Regex = Regex::new(r"[^A-Za-z0-9_-]").unwrap(); // TODO: is this defined somewhere?
}

fn requirement_link(req: &Rc<Requirement>) -> String {
    if let Some(title) = &req.title {
        let replaced = format!("{}-{}", req.id, title);
        let replaced = replaced.to_lowercase();
        let replaced = REPLACE.replace_all(&replaced, "-");

        format!("[{}](#{} \"{}\")", req.id, replaced, title)
    } else {
        let replaced = &req.id;
        let replaced = replaced.to_lowercase();
        let replaced = REPLACE.replace_all(&replaced, "-");

        format!("[{}](#{})", req.id, replaced)
    }
}

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

pub fn traced_requirements<'r, W, R>(reqs: R, graph: &Graph<'_>, w: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r TracedRequirement<'r>>,
{
    for req in reqs {
        writeln!(
            w,
            "\n## {}: {}\n\nOrigin: `{}:{}`",
            req.requirement.id,
            req.requirement.title.as_ref().unwrap_or(&"".to_owned()),
            req.requirement.location.file.display(),
            req.requirement.location.line
        )?;

        if !req.upper.is_empty() {
            writeln!(w, "\nCovers:")?;
            for (fork, coverage) in &req.upper {
                if coverage.is_empty() {
                    writeln!(
                        w,
                        "*   Does not cover: {}",
                        fork.from(graph).artefact(graph).id
                    )?;
                } else {
                    writeln!(w, "*   {}", fork.from(graph).artefact(graph).id)?;
                    for cov in coverage {
                        writeln!(w, "    *   {}", requirement_link(cov.upper_requirement))?;
                    }
                }
            }
        }

        if !req.lower.is_empty() {
            writeln!(w, "\nCovered By:")?;
            for (fork, coverage) in &req.lower {
                if coverage.is_empty() {
                    write!(w, "*   Not Covered by: ",)?;
                    let mut comma = false;
                    for t in fork.tines(graph) {
                        if comma {
                            write!(w, ", ")?;
                        }
                        comma = true;
                        write!(w, "{}", t.to(graph).artefact(graph).id)?;
                    }
                    writeln!(w)?;
                } else {
                    write!(w, "*   ")?;
                    let mut comma = false;
                    for t in fork.tines(graph) {
                        if comma {
                            write!(w, ", ")?;
                        }
                        comma = true;
                        write!(w, "{}", t.to(graph).artefact(graph).id)?;
                    }
                    writeln!(w)?;

                    for cov in coverage {
                        writeln!(w, "    *   {}", requirement_link(cov.lower_requirement))?;
                    }
                }
            }
        }

        for (k, v) in &req.requirement.attributes {
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
        UnknownFork(from, to) => {
            writeln!(w, "Unknown Edge {} -> {}", from, to)?;
        }
        CoveredWithWrongTitle {
            upper,
            lower,
            wrong_title,
        } => {
            writeln!(
                w,
                "{}:{}: {} covered with wrong title \n\texpected: {}\n\tactual  : {}",
                upper.location.file.display(),
                upper.location.line,
                upper.id,
                lower.title.as_ref().unwrap_or(&"<no title>".to_owned()),
                wrong_title,
            )?;
        }
        DependWithWrongTitle {
            upper,
            lower,
            wrong_title,
        } => {
            writeln!(
                w,
                "{}:{}: {} depend with wrong title:\n\texpected: {}\n\tactual  : {}",
                upper.location.file.display(),
                upper.location.line,
                upper.id,
                lower.title.as_ref().unwrap_or(&"<no title>".to_owned()),
                wrong_title,
            )?;
        }

        ArtefactTypeOnlyAllowsOnePath(_, _) | EmptyGraph => {
            writeln!(w, "{:?}", error)?;
        }
        UnknownJob(j) => {
            writeln!(w, "unknown job {:?}", j)?;
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
    writeln!(w, "```")?;
    for e in errors {
        err(e, w)?;
    }
    writeln!(w, "```")?;

    Ok(())
}

pub fn tracing<W>(tracing: &Tracing<'_>, graph: &Graph<'_>, w: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    {
        let mut errors = graph.get_parsing_errors().peekable();
        if errors.peek().is_some() {
            writeln!(w)?;
            writeln!(w)?;
            writeln!(w, "# Artefact Errors")?;
            writeln!(w)?;
            writeln!(w, "```")?;

            for e in errors {
                err(e, w)?;
            }
            writeln!(w, "```")?;
        }
    }

    {
        let errors = tracing.errors();
        if !errors.is_empty() {
            writeln!(w)?;
            writeln!(w)?;
            writeln!(w, "# Tracing Errors")?;
            writeln!(w)?;
            writeln!(w, "```")?;

            let mut set: HashSet<Vec<u8>> = HashSet::new();
            for e in errors {
                let mut s = Vec::new();
                err(e, &mut s)?;
                if set.insert(s.clone()) {
                    w.write_all(&s)?;
                }
            }
            writeln!(w, "```")?;
        }
    }

    {
        // Uncovered
        let mut uncovered: Vec<_> = tracing.uncovered().collect();
        uncovered.sort_unstable_by_key(|r| &r.requirement.id);

        if !uncovered.is_empty() {
            writeln!(w)?;
            writeln!(w)?;
            writeln!(w, "# Uncovered Requirements")?;
            writeln!(w)?;

            for r in uncovered {
                writeln!(w, "*   {}", requirement_link(r.requirement))?;
            }
        }
    }

    {
        // Derived
        let mut derived: Vec<_> = tracing.derived().collect();
        derived.sort_unstable_by_key(|r| &r.requirement.id);

        if !derived.is_empty() {
            writeln!(w)?;
            writeln!(w)?;
            writeln!(w, "# Derived Requirements")?;
            writeln!(w)?;

            for r in derived {
                writeln!(w, "*   {}", requirement_link(r.requirement))?;
            }
        }
    }

    {
        // Covered
        writeln!(w)?;
        writeln!(w)?;
        writeln!(w, "# Requirements")?;
        writeln!(w)?;
        let mut covered: Vec<_> = tracing.requirements().iter().collect();
        covered.sort_unstable_by_key(|req| (req.artefact(graph).id, &req.requirement.id));
        traced_requirements(covered.into_iter(), graph, w)?;
    }

    Ok(())
}
