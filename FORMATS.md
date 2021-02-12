# Formats

Covers: REQ-006 (Well defined Formats)

## Primitives

These Rules hold for all Artefact formats:


### ID
The Id of a requirement, which matches the Regular Expression `[a-zA-Z][a-zA-Z0-9_-]`.




## Markdown Requirements

This project's format as used in `REQUIREMENTS.md` or this File.

The artefact is a Markdown file with freely chosen format. A requirement is
a Heading that matches the syntax stated below and the paragraphs that follow.

Options:
*   Requirement Prefix: A list of prefixes that are considered for finding
    Requirements. For example `"UC-","REQ-"`. An empty list disables prefix
    mathing.
*   Enforce Prefix: If true, Headings that start with a prefix but do not match
    the Syntax of a requirement produce an error.
*   Nested Requirement: If true, nested requirements are constructed. If False,
    Nested Requirements produce an Error.

A Requirement starts with a `#` heading of any level that has the form `ID:
TITLE`, if the id matches a prefix or the list of prefixes is empty.
The underlined headings are not supported to ease parsing. TODO: reconsider?

The Paragraphs that directly follow the heading give the description of the
requirement.

Paragraphs where each line matches `Covers: ID( \([^)]*\))?` (an ID whic hmay or
may not be followed by text in parenthesis, where text does not contain
a closing paren) are omitted from the description and define a coverage of the
requirement with ID. The text in the parenthesis must match the title of the
covered requirement.

All paragraphs are considered, until either:
*   A Heading with the same or lower level
*   A Heading that itself starts a Requirement. This becomes a nested
    requirement (unless this feature is deactivated)
