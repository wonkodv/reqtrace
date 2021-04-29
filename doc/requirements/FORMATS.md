
# Config File


Json File

*   Artefacts
    *   id
        *   paths
        *   parser
*   tracing
    *   list of:
        *   pair of
            *   list of upwards Requirements
            *   List of downards Requirements


# Artefact Parsers

The set of formats that are parsed by this tool are well defined by this
specification.


## Tool wide Formats

These Rules hold for all Artefact formats supported by the tool except JSON,
where anything is possible.

### FMT_ID_v2: Requirement Identifier

Requirement identifier consist of letters, digits and underscore, specifically
they match the Regular Expression

    \p{XID_Start}\p{XID_Continue}+

This gives the user the greatest flexibility.

Covers:
*   REQ_FORMATS (Well defined Formats)

Comment:
Discussion in the [README](README.md#requirement-ids)

History:
*   v2: Align to Unicode Identifiers


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


## Markdown Requirements

This project's preferred format as used in `REQUIREMENTS.md` or this file.
Everything is ignored until a requirement starts. The everything is
a requirement until the next requirement or heading.

### FMT_MD: Markdown File Format


The artefact is a Markdown file with freely chosen layout.  A Requirement is in
a heading line with requirement ID and title, followed by description and other
attributes.

Covers:
*   REQ_FORMATS: Well defined Formats

Depends:
*   FMT_MD_START
*   FMT_MD_DESC
*   FMT_MD_DESC_HEADINGS
*   FMT_MD_ATTRIBUTES


### FMT_MD_START: Requirement Start

A Requirement starts with a `#` heading of any level that has the form `ID:
TITLE`.

### FMT_MD_DESC: Description

The paragraphs following the start of the requirement make up the description of
the requirement.

All paragraphs add to the description until:
*   The Start of another Requirement.
*   The start of an Attribute Paragraph
*   A Heading the same level or less. This ends the Requirement.



### FMT_MD_DESC_HEADINGS: Heading Level in Description is adjusted

Headings with a lower level than the starting one, that do not start a nested
requirement are added to the description. Their heading level is adjusted by
removing as many leading `#` as the requirement had

### FMT_MD_ATTRIBUTES: Attributes

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

### Example

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


### Markdown Artefact Parsing Options

The following options can be used to configure parsing of markdown artefacts.

#### FMT_MD_OPT_PREFIX: List of Prefixes

A List of strings can be passed, which is used to prevent the parser from
creating unintended requirements from headlines which accidentally have the
right form.

Lines in the markdown file which would start a new requirement, are treated as
normal headings, if the identifier of the would be requirement does not start
with one of the list of prefixes. If the list is empty, no prefix matching is
performed and all matching lines lead to a requirement.


## Mono Requirement File
A file which is one Requirement that has multiple `Depends` references. This can
be used as the Root of the coverage Tree, for example a packages `README.md`
like in this tool.

## FMT_MONO: Mono Requirement File
Artefact of type MonoRequirement emit exactly one Requirement with the following
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


## JSON Requirements

*   [Requirements]
    *   id
    *   location
    *   ...

TODO

### FMT_JSON: JSON Requirements Format
Covers:
*   REQ_FORMATS: Well defined Formats

## Rust Coverage Marks

Parse `cov_mark::hit!(DSG_001)`

TODO

Covers:
*   REQ_FORMATS: Well defined Formats


## Rust Unsafe Reasoning

Create Requirements from `unsafe` code, so each occurrence requires a justification in an
extra document. This can be used to prove that each use was analyzed for necessity and safe
implementation.

### FMT_RS_UNSAFE: Unique Requirement for each unsafe keyword

Each occurrence of `unsafe` in rust code leads to a unique requirement.
The Id of the requirement is given by a comment immediately following the keyword

    let x = unsafe /* UNSAFE_ID_EXAMPLE */ { 42 };

Requirement IDs must be unique (REQ_UNIQUE_ID) so each use of the `unsafe` keyword will
have to have a different ID.

Covers:
*   REQ_FORMATS: Well defined Formats

### FMT_RS_UNSAFE_NEEDS_ID: Unsafe Without Id Produce Error

Occurrences of the `unsafe` keyword without a comment that gives an ID produce a parsing error.

## Regex Parsing

Custom Parsers can be defined which are backed by regular Expressions. There are
two methods a simple and approachable one, and a complex and powerul one.

### FMT_REGEX_MULTIPASS

The Text File is parsed in phases:

1.  Load Text File
2.  Replace the Text using Regular expressions. This can be used to remove
    headers or footers 

Match Everything sort later
    1 big Regex with Names for attributes
    A Req Starts, everything that follows gets attached to that req until the
    next starts.


### FMT_REGEX_STATEMACHINE
Statemachine:
    List of Regexes
    List of states:
        State Name, Regex Name, Attribute to set and State that results from it



# Output Formats

## Json
each list sorted to minimize diff !

* Artefacts
  * Unique ID
  * Version Info
  * If File
    * Path
    * Hash over File
  * If Group
    * Child Artefacts (have no upwards or downwards artefact links)
  * Upwards Artefact IDs
  * Downwards Aretefact IDs
  * Requirements defined in that Artefact
    * ID
    * Title
    * Description
    * Location
    * Covers REQ_Id\*
    * Covered-by Req_Id

TODO

## Markdown

With link to artefact.md#req-id

## Latex

Generated by seperate Tool, using latex config, req state. Generates 1 tex file per artefact so recompiling tex is cheaper.
(attention using include or input, one makes aux files the other recompiles from
scratch)

Graph of Artefacts

SubSection per Artefact

SubSubsection per Requirement

ID, Title, \\ref{req-id}

Description

Covers:

* Artefact
  * Completeness [Link]ReqId: Title

Covered By:

* Artefact
  * [Link] ReqId Title


## ctags

Emit Tag with type R for the Requirement,
Type C for where a req is covered
Type D for where it is depended on.
