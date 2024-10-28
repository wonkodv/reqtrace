#![allow(clippy::too_many_lines)]

use std::collections::HashSet;
use std::io;
use std::rc::Rc;

use lazy_static::lazy_static;
use regex::Regex;

use crate::aggregator::AggregatedGraph;
use crate::aggregator::RequirementTrace;
use crate::models::Location;
use crate::models::LocationInFile;
use crate::models::Requirement;
use crate::models::TracedGraph;
use crate::models::*;

lazy_static! {
    static ref REPLACE_WITH_DASH: Regex = Regex::new(r"[ ]").unwrap(); // TODO: is this defined somewhere?
    static ref REMOVE: Regex = Regex::new(r"[^A-Za-z0-9_-]").unwrap(); // TODO: is this defined somewhere?
}

/// requirement Id as markdown link
fn requirement_link(req: &Requirement) -> String {
    if let Some(title) = &req.title {
        let replaced = format!("{}-{}", req.id, title);
        let replaced = replaced.to_lowercase();
        let replaced = REPLACE_WITH_DASH.replace_all(&replaced, "-");
        let replaced = REMOVE.replace_all(&replaced, "");

        format!("[{}](#{} \"{}\")", req.id, replaced, title)
    } else {
        let replaced = &req.id;
        let replaced = replaced.to_string().to_lowercase();
        let replaced = REPLACE_WITH_DASH.replace_all(&replaced, "-");
        let replaced = REMOVE.replace_all(&replaced, "");

        format!("[{}](#{})", req.id, replaced)
    }
}

/// requirement Id as markdown link  plus req title
fn requirement_link_with_title(req: &Requirement) -> String {
    if let Some(title) = &req.title {
        let replaced = format!("{}-{}", req.id, title);
        let replaced = replaced.to_lowercase();
        let replaced = REPLACE_WITH_DASH.replace_all(&replaced, "-");
        let replaced = REMOVE.replace_all(&replaced, "");

        format!("[{}](#{}): {}", req.id, replaced, title)
    } else {
        let replaced = &req.id;
        let replaced = replaced.to_string().to_lowercase();
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

pub fn requirement(req: &Requirement, w: &mut impl io::Write) -> io::Result<()> {
    writeln!(
        w,
        "\n## {}: {}\n\nOrigin: {}",
        req.id,
        req.title.as_ref().unwrap_or(&String::new()),
        location_link(&req.location)
    )?;

    if let Some(description) = req.attributes.get("Description") {
        writeln!(w, "\n\n{description}")?;
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
                writeln!(w, "\n{k}:")?;
                writeln!(w, "{v}")?;
            }
        }
    }

    Ok(())
}

pub fn requirements(graph: &Graph, w: &mut impl io::Write) -> io::Result<()> {
    let mut reqs: Vec<&Rc<Requirement>> = graph
        .artefacts
        .values()
        .flat_map(|art| art.requirements.values())
        .collect();

    reqs.sort_by(|r, o| r.id.cmp(&o.id));

    for req in reqs {
        requirement(&req, w)?;
    }
    Ok(())
}

pub fn traced_requirement(req: &RequirementTrace<'_>, w: &mut impl io::Write) -> io::Result<()> {
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
        writeln!(w, "\n\n{description}")?;
    }

    for (k, v) in &req.requirement.attributes {
        if k != "Description" {
            let v = v.trim();
            if !v.is_empty() {
                writeln!(w, "\n{k}:")?;
                writeln!(w, "{v}")?;
            }
        }
    }

    {
        writeln!(w, "\nUpwards Tracing:")?;
        let mut derived = true;
        for (relation, coverages) in &req.covers {
            writeln!(w, "*   {}", relation)?;
            for (upper_requirement, reference_location) in coverages {
                writeln!(w, "    *   {}", requirement_link(upper_requirement),)?;
                writeln!(
                    w,
                    "        Reference: {}",
                    location_link(reference_location),
                )?;

                derived = false;
            }
        }
        if derived {
            writeln!(w, "    *   Derived")?;
        }
    }

    {
        writeln!(w, "\nDownwards Tracing:")?;
        for (relation, coverages) in &req.depends {
            writeln!(w, "*   {}", relation)?;
            if coverages.is_empty() {
                writeln!(w, "    *    UNCOVERED")?;
            }
            for (upper_requirement, reference_location) in coverages {
                writeln!(w, "    *   {}", requirement_link(upper_requirement),)?;
                writeln!(
                    w,
                    "        Reference: {}",
                    location_link(reference_location),
                )?;
            }
        }
    }

    Ok(())
}

pub fn err(error: &Error, w: &mut impl io::Write) -> io::Result<()> {
    match error {
        Error::Format(loc, err) => {
            writeln!(
                w,
                "*   Format Error: {}\n    in {}",
                err,
                location_link(loc),
            )
        }
        Error::DuplicateRequirement(r1, r2) => {
            writeln!(
                w,
                concat!(
                    "*   Duplicate Requirement: {}\n",
                    "    first seen in {}\n",
                    "    then again in {}"
                ),
                r1.id,
                location_link(&r2.location),
                location_link(&r1.location),
            )
        }
        Error::DuplicateAttribute(loc, attr, req) => {
            writeln!(
                w,
                "*   Duplicate Attribute: {} when parsing {}\n    {}",
                attr,
                req,
                location_link(loc),
            )
        }
        Error::Io(path, err) => {
            writeln!(w, "*   IO Error: {}\n   {}", err, path.display())
        }
        Error::ArtefactConfig(err) => {
            writeln!(w, "*   Artefact Config Error: {}", err,)
        }
        Error::DuplicateArtefact(a) => {
            writeln!(w, "*   Duplicate Artefact: {a}")
        }
        Error::UnknownArtefact(a) => {
            writeln!(w, "*    Unknown Artefact: {a}")
        }
        Error::CoveredWithWrongTitle {
            upper,
            lower,
            wrong_title,
            location,
        } => {
            writeln!(
                w,
                concat!(
                    "*   Requirement covered with wrong title:\n",
                    "    Upper Requirement {}",
                    "    {}\n",
                    "    Lower Requirement {}\n",
                    "    {}\n",
                    "    Title of Upper Requirement: {}\n",
                    "    Title used to cover it:     {}\n",
                    "    at:                         {}",
                ),
                upper.id,
                location_link(&upper.location),
                lower.id,
                location_link(&lower.location),
                upper.title.as_ref().unwrap_or(&"<no title>".to_owned()),
                wrong_title,
                location,
            )
        }
        Error::DependWithWrongTitle {
            upper,
            lower,
            wrong_title,
            location,
        } => {
            writeln!(
                w,
                concat!(
                    "*   Requirement depended on with wrong title:\n",
                    "    Upper Requirement {}\n",
                    "    {}\n",
                    "    Lower Requirement {}\n",
                    "    {}\n",
                    "    Title of Lower Requirement: {}\n",
                    "    Title used to cover it:     {}\n",
                    "    Referenced at:              {}\n",
                ),
                upper.id,
                location_link(&upper.location),
                lower.id,
                location_link(&lower.location),
                upper.title.as_ref().unwrap_or(&"<no title>".to_owned()),
                wrong_title,
                location,
            )
        }

        Error::DependOnUnknownRequirement(req, depend, location) => {
            writeln!(
                w,
                "*   {} depends on unknown Requirement {}\n    {}",
                requirement_link(req),
                depend,
                location_link(location),
            )
        }
        Error::CoversUnknownRequirement(req, cover, location) => {
            writeln!(
                w,
                "*   {} covers unknown Requirement {}\n    {}",
                requirement_link(req),
                cover,
                location_link(location),
            )
        }
        Error::EmptyGraph => {
            writeln!(w, "Tracing Graph is empty")
        }
        Error::UnusedRelation(rel) => {
            writeln!(
                w,
                "*   No requirement was traced along the relation {}. (Configuration error?)",
                rel
            )
        }
    }
}

pub fn errors<'r, W, R>(errors: R, w: &mut impl io::Write) -> io::Result<()>
where
    R: Iterator<Item = &'r Error>,
{
    writeln!(w, "# Errors")?;
    for e in errors {
        err(e, w)?;
    }

    Ok(())
}

pub fn tracing(aggregated_graph: &AggregatedGraph<'_>, w: &mut impl io::Write) -> io::Result<()> {
    let traced_graph = aggregated_graph.traced_graph;
    {
        let mut headline = false;

        for a in traced_graph.artefacts.values() {
            if !a.errors.is_empty() {
                if !headline {
                    headline = true;
                    writeln!(w)?;
                    writeln!(w)?;
                    writeln!(w, "# Input Errors")?;
                    writeln!(w)?;
                }
                writeln!(w)?;
                writeln!(w, "## {}", a.id)?;
                writeln!(w)?;
            }
            for e in &a.errors {
                err(e, w)?;
            }
        }
    }

    {
        let errors = &traced_graph.errors;
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
        let mut headline = false;

        for rel in &traced_graph.traced_relations {
            if !rel.uncovered.is_empty() {
                if !headline {
                    headline = true;
                    writeln!(w)?;
                    writeln!(w)?;
                    writeln!(w, "# Uncovered Requirements")?;
                    writeln!(w)?;
                }
                writeln!(w)?;
                writeln!(w, "## {}", rel)?;
                writeln!(w)?;

                for req_id in &rel.uncovered {
                    writeln!(
                        w,
                        "*   {}",
                        requirement_link(aggregated_graph.requirements[&req_id].requirement)
                    )?;
                }
            }
        }
    }

    {
        // Derived

        let mut headline = false;
        for (art, derived) in &traced_graph.derived {
            if !derived.is_empty() {
                if !headline {
                    headline = true;
                    writeln!(w)?;
                    writeln!(w)?;
                    writeln!(w, "# Derived Requirements")?;
                    writeln!(w)?;
                }
                writeln!(w)?;
                writeln!(w, "## {}", art)?;
                writeln!(w)?;

                for r in derived {
                    writeln!(
                        w,
                        "*   {}",
                        requirement_link(aggregated_graph.requirements[r].requirement)
                    )?;
                }
            }
        }
    }

    {
        // All Requirements
        writeln!(w)?;
        writeln!(w)?;
        writeln!(w, "# Requirements")?;
        writeln!(w)?;

        for artefact in traced_graph.artefacts.values() {
            writeln!(w, "## {}", artefact.id)?;

            for req_id in artefact.requirements.keys() {
                traced_requirement(&aggregated_graph.requirements[req_id], w)?
            }
        }
    }

    Ok(())
}
