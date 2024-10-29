# File Formats

The data format for machine readable input and output
needs to be well supported, text based and produce a reasonably small diff if
little changed.

## General Considerations

Candidates:
*   Machine Readable
    *   JSON
    *   YAML
    *   TOML
    *   CBOR
*   Human readable
    *   Markdown
    *   Tex
    *   HTML
    *   Typst
    *   (JSON, YAML)
    *   TOML

Text based formats will work better with SVCS,
debugging and plumbing in other tools with limited libraries e.g. Bash scripts.
For the machine readable format, JSON is chosen. The possible benefits of other formats
(easier to write for humans, smaller size, more data types) are not needed to represent requirements or tracing information.
For human readability, Markdown is the most readable format before additional rendering.
Other formats might be more useful for further processing with tools, but that can be achieved by
generating them from the Json export.

For configuration files, JSON is not comfortable enough to maintain, YAML is too complex, so TOML is chosen.


The following Hold unless otherwise specified:

### FMT_ID_v2: Requirement Identifier

Requirement identifier consist of letters, digits and underscore, specifically
they match the Regular Expression

    \p{XID_Start}\p{XID_Continue}+

This gives the user the greatest flexibility.

Covers:
*   REQ_FORMATS: Well defined Formats

Comment:
Discussion in the [README](README.md#requirement-ids)

History:
*   v2: use to Unicode Identifiers


### FMT_FILE_ENCODINGS: Handle File Encodings

When not otherwise specified, Text Files are read as UTF-8 and encoding errors are
replaced.

Covers:
*  REQ_UNICODE_SAFE: Sane Handling of unicode


### FMT_UNICODE_NORMALIZE: Normalize Unicode during read

All input strings are unicode normalizes as
[NFC](https://www.unicode.org/reports/tr15/#Normalization_Forms_Table).
This means that
*   All output derived from input will be NFC normalized
*   Identifier Matching can be done on the byte level

Covers:
*  REQ_UNICODE_SAFE: Sane Handling of unicode


Comment:
See [Rust RFC 2457](https://rust-lang.github.io/rfcs/2457-non-ascii-idents.html) on the topic.

This means two requirement ids are equal if their NFC forms are equal.



## Config


### FMT_CONFIG_TOML: Use a Single TOML File as Configuration

The configuration should be placed in one file `requirements.toml`.
The format is TOML.

Covers:
*   REQ_CONFIG: Simple Configuration in One File


### FMT_CONFIG_STRUCTURE

The config file has the following fields:

Key `version` indicates the version of the Config file.

Supported Versions:
*   `0`: **unstable** Only what serde can deserialize into the current `crate::models::Config`

## Export

### FMT_EXPORT_JSON: JSON for Exporting Results

The Json Formatter emits Json objects for:
*   Lists of Errors
*   Lists of Requirements
*   Graph (Artefacts + Relations)
*   Tracing Info

The Output Object has a version field which indicates the format of the remainder of the object.
Supported Versions:

Version:
*   `0`: **unstable**. What serde can serialize from current `crate::models::*`

Covers: REQ_CACHE_FRIENDLY, REQ_MACHINE_READABLE

### FMT_EXPORT_MARKDOWN: Export to Markdown

Errors, Requirements, Status, Tracing Info are exported as a useful
standalone Markdown File. The format may change with future versions.

Covers:
*   REQ_FORMATS: Well defined Formats
*   REQ_HUMAN_READABLE

### FMT_EXPORT_CTAGS: Export Requirements as CTags

Export Requirements as `tags` file for easy navigation with tools like vim or emacs.

For each requirement, emit one line:
*   with requirement ID
*   File
*   Line Number
*   Type `r`

If Tracing is exported,
add the location a requirement was covered at with `c` or depended on with `d`.

All lines of the file are sorted

Covers:
*   REQ_FORMATS: Well defined Formats

## Import

### FMT_IMPORT_JSON: JSON for Importing Requirements

The Json Parser loads lists of requirements from Json files, matching the following specification:

Input Json Files should contain a single object with:
*   `"version"`: the version of the file format
*   `"requirements"`: a List of Requirement Objects.

Supported Versions:
*   `0`: **unstable** Only what serde can deserialize into the current `crate::models::Requirement`

Covers:
*   REQ_CACHE_FRIENDLY
*   REQ_EXTENSIBLE
*   REQ_FORMATS: Well defined Formats

### Markdown Requirements

This project's preferred format as used in `REQUIREMENTS.md` or this file.
Everything is ignored until a requirement starts. Then, everything is
a requirement until the next requirement or heading.

#### FMT_IMPORT_MARKDOWN_REQUIREMENT: Markdown File Format


The artefact is a Markdown file with freely chosen layout.  A Requirement is in
a heading line with requirement ID, a colon, a space and a title, followed by description and other
attributes.

Covers:
*   REQ_FORMATS: Well defined Formats


#### FMT_MD_START: Requirement Start

A Requirement starts with a `#` heading of any level that has the form `ID:
TITLE`.

Covers:
*   REQ_FORMATS: Well defined Formats
#### FMT_MD_DESC: Description

The paragraphs following the start of the requirement make up the description of
the requirement.

All paragraphs add to the description until:
*   The Start of another Requirement.
*   The start of an Attribute Paragraph
*   A Heading the same level or less. This ends the Requirement.


Covers:
*   REQ_FORMATS: Well defined Formats

#### FMT_MD_DESC_HEADINGS: Heading Level in Description is adjusted

Headings with a lower level than the starting one, that do not start a nested
requirement are added to the description. Their heading level is adjusted by
removing as many leading `#` as the requirement had

Covers:
*   REQ_FORMATS: Well defined Formats

#### FMT_MD_ATTRIBUTES: Attributes

Attributes are parsed from paragraphs that start with a single word followed by
a colon. Once the first Attribute Paragraph is encountered, the Parser will add
to the Attribute Value until:

*   The Start of another Requirement.
*   The start of another Attribute Paragraph
*   A Heading the same level or less. This ends the Requirement.

There are 3 types of Attribute Paragraphs:
*   Text: can span multiple Paragraphs
*   Short List: Comma separated list of IDs
*   Long list:  one paragraph that only consists of bullet points

The Attribute `Tags` can be a short or long list of arbitrary Tags which have
the same format as requirement IDs.

The Attributes `Depends` and `Covers` can be short lists of Requirement IDs or
long lists, where each item starts with a requirement id, optionally followed by
a colon and the title of the referenced requirement.

Comment:

    `Covers: Some, IDs,`

or

    `Covers:
    *   Some
    *   IDs

Covers:
*   REQ_FORMATS: Well defined Formats
#### Example

A Markdown File with the following content:

    # Headline
    ## REQ_ID: Requirement Title
    Description paragraph
    ### Heading in the description
    another Description Paragraph

    Covers: REQ_COV

    ## Headline outside the description

would lead to one Requirement with id `REQ_ID` and title `Requirement Title`
It would cover `REQ_COV` by ID.
The Description would be:

    Description paragraph
    # Heading in the description
    another Description Paragraph

#### FMT_MD_OPT_PREFIX: List of Prefixes

A List of strings can be passed, which is used to prevent the parser from
creating unintended requirements from headlines which accidentally have the
right form.

Lines in the markdown file which would start a new requirement, are treated as
normal headings, if the identifier of the would be requirement does not start
with one of the list of prefixes. If the list is empty, no prefix matching is
performed and all matching lines lead to a requirement.

Covers:
*   REQ_FORMATS: Well defined Formats

### The Mono Requirement File

A file which is one Requirement, that has multiple `Depends` references. This can
be used as the Root of the coverage Tree, for example a package's `README.md`
like in this tool.

#### FMT_IMPORT_MONO_REQ: Single Requirement Per File
the MonoReq parser emits exactly one Requirement with the following
attributes:
*   Id: The stem of the file path (i.e. `README.md`)
*   Title:  The first line containing Word-Characters with all non-word
    characters trimmed of both ends of the line. (Allowing Markdown heading,
    C style comments, ...)
*   Depends: Every Requirement-Id that immediately follows a fat arrow (`=>`).

Comment:
See this projects README for examples.

Covers:
*   REQ_FORMATS: Well defined Formats


### Rust Files

The `requirement_covered` rust macro is used to cover a Requirement from Code.

#### FMT_IMPORT_RUS_COV_MARK: Rust Coverage Marks

Parse `requirement_covered!(REQ_ID)` and `requirement_covered!(REQ_ID,"TITLE")` as Coverage Links.
The requirement ID is derived form the surrounding items Path

Covers:
*   REQ_FORMATS: Well defined Formats
