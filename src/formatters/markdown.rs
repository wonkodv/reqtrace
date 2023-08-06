use super::super::common::*;
use std::{collections::HashSet, io, rc::Rc};

use crate::graph::Graph;
use crate::{errors::Error, trace::Tracing};
use Error::*;

use lazy_static::lazy_static;
use regex::Regex;

use crate::trace::TracedRequirement;

lazy_static! {
    static ref REPLACE_WITH_DASH: Regex = Regex::new(r"[ ]").unwrap(); // TODO: is this defined somewhere?
    static ref REMOVE: Regex = Regex::new(r"[^A-Za-z0-9_-]").unwrap(); // TODO: is this defined somewhere?
}

/// requirement Id as markdown link
fn requirement_link(req: &Rc<Requirement>) -> String {
    if let Some(title) = &req.title {
        let replaced = format!("{}-{}", req.id, title);
        let replaced = replaced.to_lowercase();
        let replaced = REPLACE_WITH_DASH.replace_all(&replaced, "-");
        let replaced = REMOVE.replace_all(&replaced, "");

        format!("[{}](#{} \"{}\")", req.id, replaced, title)
    } else {
        let replaced = &req.id;
        let replaced = replaced.to_lowercase();
        let replaced = REPLACE_WITH_DASH.replace_all(&replaced, "-");
        let replaced = REMOVE.replace_all(&replaced, "");

        format!("[{}](#{})", req.id, replaced)
    }
}

/// requirement Id as markdown link
/// TODO: currently works only on GITHUB and only if generated file lands in root/*/
fn location_link(loc: &Location) -> String {
    let file = &loc.file.display();
    match &loc.location_in_file {
        Some(LocationInFile::Line(line)) => {
            format!("[{file}:{line}](../{file}?plain=1#L{line})")
        }
        Some(LocationInFile::LineAndColumn(line, column)) => {
            format!("[{file}:{line}:{column}](../{file}?plain=1#L{line})")
        }
        Some(LocationInFile::String(string)) => {
            format!("[{file}](../{file}):{string}")
        }
        None => {
            format!("[{file}](../{file})")
        }
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
            "\n## {}: {}\n\nOrigin: {}",
            req.id,
            req.title.as_ref().unwrap_or(&"".to_owned()),
            location_link(&req.location)
        )?;

        if let Some(description) = req.attributes.get("Description") {
            writeln!(w, "\n\n{}", description)?;
        }

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
            if k != "Description" {
                let v = v.trim();
                if !v.is_empty() {
                    writeln!(w, "\n{}:", k)?;
                    writeln!(w, "{}", v)?;
                }
            }
        }
    }
    Ok(())
}

pub fn traced_requirements<'r, W, R>(reqs: R, graph: &Graph, w: &mut W) -> io::Result<()>
where
    W: io::Write,
    R: Iterator<Item = &'r TracedRequirement<'r>>,
{
    for req in reqs {
        writeln!(
            w,
            "\n## {}{}\n\nOrigin: {}",
            req.requirement.id,
            req.requirement
                .title
                .as_ref()
                .map(|t| format!(": {t}"))
                .unwrap_or_default(),
            location_link(&req.requirement.location)
        )?;

        if let Some(description) = req.requirement.attributes.get("Description") {
            writeln!(w, "\n\n{}", description)?;
        }

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
            if k != "Description" {
                let v = v.trim();
                if !v.is_empty() {
                    writeln!(w, "\n{}:", k)?;
                    writeln!(w, "{}", v)?;
                }
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
            writeln!(
                w,
                "*   Format Error: {}\n    in {}",
                err,
                location_link(loc),
            )
        }
        DuplicateRequirement(r1, r2) => {
            writeln!(
                w,
                concat!(
                    "*   Duplicate Requirement: {}\n",
                    "first seen in {}\n",
                    "then again in {}"
                ),
                r1.id,
                location_link(&r2.location),
                location_link(&r1.location),
            )
        }
        DuplicateAttribute(loc, attr, req) => {
            writeln!(
                w,
                "*   Duplicate Attribute: {} when parsing {}\n    {}",
                attr, req, loc
            )
        }
        IoError(path, err) => {
            writeln!(w, "*   IO Error: {}\n   {}", err, path.display())
        }
        UnknownArtefactType(t) => {
            writeln!(w, "*   Unknown Artefact Type: {}", t)
        }
        ConfigError(s) => {
            writeln!(w, "*   Config  Error: {:?}", s)
        }
        DuplicateArtefact(a) => {
            writeln!(w, "*   Duplicate Artefact: {}", a)
        }
        UnknownArtefact(a) => {
            writeln!(w, "*    Unknown Artefact: {}", a)
        }
        UnknownFork(from, to) => {
            writeln!(w, "*    Unknown Edge {} -> {}", from, to)
        }
        CoveredWithWrongTitle {
            upper,
            lower,
            wrong_title,
            location,
        } => {
            writeln!(
                w,
                concat!(
                    "*   Requirement covers with wrong title:\n",
                    "*   Upper Requirement {}\n",
                    "*\n",
                    "    {}\n",
                    "*   Lower Requirement {}\n",
                    "    {}\n",
                    "*   Title of Upper Requirement: {}\n",
                    "*   Title used to cover it:     {}\n",
                ),
                upper.id,
                location_link(&upper.location),
                lower.id,
                location_link(&lower.location),
                upper.title.as_ref().unwrap_or(&"<no title>".to_owned()),
                wrong_title,
            )?;
            if let Some(location) = location {
                writeln!(
                    w,
                    "*   Referenced at:              {}",
                    location_link(&location)
                )?;
            };
            Ok(())
        }
        DependWithWrongTitle {
            upper,
            lower,
            wrong_title,
            location,
        } => {
            writeln!(
                w,
                concat!(
                    "*   Requirement depended on with wrong title:\n",
                    "*   Upper Requirement {}\n",
                    "    {}\n",
                    "*   Lower Requirement {}\n",
                    "    {}\n",
                    "*   Title of Lower Requirement: {}\n",
                    "*   Title used to cover it:     {}\n",
                ),
                upper.id,
                location_link(&upper.location),
                lower.id,
                location_link(&lower.location),
                upper.title.as_ref().unwrap_or(&"<no title>".to_owned()),
                wrong_title,
            )?;
            if let Some(location) = location {
                writeln!(
                    w,
                    "*   Referenced at:              {}",
                    location_link(&location)
                )?;
            };
            Ok(())
        }

        ArtefactTypeOnlyAllowsOnePath(_, _) | EmptyGraph => {
            writeln!(w, "{:?}", error)
        }
        UnknownJob(j) => {
            writeln!(w, "unknown job {:?}", j)
        }
        DependOnUnknownRequirement(req, depend, location) => {
            writeln!(
                w,
                "*   {} depends on unknown Requirement {}\n    {}",
                req.id,
                depend,
                location_link(&location.as_ref().unwrap_or(&req.location)),
            )
        }
        CoversUnknownRequirement(req, cover, location) => {
            writeln!(
                w,
                "*   {} covers unknown Requirement {}\n    {}",
                req.id,
                cover,
                location_link(&location.as_ref().unwrap_or(&req.location)),
            )
        }
        GenericError(err) => {
            writeln!(w, "*    {} ", err)
        }
    }
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

pub fn tracing<W>(tracing: &Tracing<'_>, graph: &Graph, w: &mut W) -> io::Result<()>
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

            for e in errors {
                err(e, w)?;
            }
        }
    }

    {
        let errors = tracing.errors();
        if !errors.is_empty() {
            writeln!(w)?;
            writeln!(w)?;
            writeln!(w, "# Tracing Errors")?;
            writeln!(w)?;

            let mut set: HashSet<Vec<u8>> = HashSet::new();
            for e in errors {
                // echo each error string only once
                let mut s = Vec::new();
                err(e, &mut s)?;
                if set.insert(s.clone()) {
                    w.write_all(&s)?;
                }
            }
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
        covered.sort_unstable_by_key(|req| (&req.artefact(graph).id, &req.requirement.id));
        traced_requirements(covered.into_iter(), graph, w)?;
    }

    Ok(())
}
