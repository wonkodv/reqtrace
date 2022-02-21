
# Requirements

Things this tool should do in no particular order

## REQ_TRACE: Determine whcih requirements cover which

Compute tracing for each Requirement, wether it is covered, uncovered, covers
another requirement or is derived.

## REQ_UNIQUE_ID_v2: Requirements have a unique Identifier

Each requirement must be identifiable by a short, unique string.
All unicode symbols must be possible, though parsers may restrict this

History:
*   v2: Unicode

## REQ_USER_FRIENDLY: Simple to use Interface

The User Interface should be slim and straight forward.

## REQ_MACHINE_FRIENDLY: Easy to include in automated work flows

The tool should be easy to include in automated work flows, IDEs, CI Pipelines
etc.

## REQ_VCS: Allow Version Control


All Config, state, intermediate results are in Plain Text based formats that are usable in
Version Control and produce usable diff.

## REQ_INSTALL: Easy to install

no package management, libraries, dependencies


## REQ_EXTENSIBLE: Extensible Parsing

If internal parsers are not able to work on an Artefact, external tools can be
incorporated.

## REQ_IDENTIFIEABLE: Show versions of input artefacts in output

When reading the output, each input must be clearly identifiable.
For example by its:
*   git describe
*   hash
*   file modification time

## REQ_CONFIGURABLE_OUTPUT: The Output Format is Configurable

The Format in which Information is returned is configurable

## REQ_MACHINE_READABLE: Machine Readable Output

Information can be returned in a format that can easily be read by other tools

## REQ_HUMAN_READABLE: Human Readable Output

Information can be returned in a format that can easily be read by humans

## REQ_FORMATS: Well defined Formats

To work with external programs as parsers or to process the output, the formats used must be well
defined.

## REQ_UNICODE_SAFE: Sane Handling of unicode

Some Characters can be represented by multiple different sequences of Unicode
Code Points. Also Unicode Encodings like UTF-8 can encode the same Codepoint
as different bytes.

This must be handled.

## REQ_ARTEFACT_PARSE_ID: Parse Artefact Identifier

An identifier can be parsed from the line, for example a version string, or
an expanded RCS Keyword (like `$Id: /path/to/artefact.md$42$`).

## REQ_ARTEFACT_QUERY_ID: Query Tool for Artefact Identifier

An identifier can be gotten from an external tool like `git describe --tags`.

## REQ_UP: Upward Coverage

A Requirement covers a higher one by including the id of the higer one in its
Coverage attribute.

## REQ_DOWN: Downward Coverage


A Requirement is covered by a lower one by including the id of the lower one in
its Dependencies attribute.

## REQ_DELEGATION: Coverage Delegation

A Requirement delegates to another requirement in the same artefact by including
the id of the lower one in its Dependencies attribute.

It does not need to be covered itself through upward coverage of a lower
requirement.

## REQ_ERROR: Useful Parser Errors

Parser Errors give the precise location and type of the problem, for example filename with
line number of the artefact.


## REQ_MATCH_ID: Match by ID

A Requirements covers another by its ID.

## REQ_VAL_TITLE: Check matching title

A Coverage link that is established by requirement ID can be verified by
comparing the requirement's title.

Comment:
This is only really necessary where the requirement ids are not informative.
For example a Requirement with the id `DSG_123` and the title `Delete Everything`
could be covered by a line of code like:

    create_temporary_file();    COVERS(DSG_123)

Which gives little information to the reader. It is not obvious if the
requirement has nothing to do with the object that covers it.

By also providing the title of the requirement, things can get even worse, if
the title is wrong as the reader now believes to know which requirement is
meant.

    create_temporary_file();    COVERS(DSG_123, "Create Temporary File")

This is is prevented, by checking the tile. The above case would produce
a verification Error. The code would have to be changed like the following for
the tool to accept it without warning:

    create_temporary_file();    COVERS(DSG_123, "Delete Everything")

At this point you have defeated the tool, but now a review can easily discover
the wrong coverage.


## REQ_VAL_COVERAGE: Validate Coverage

An error is reported for a Coverage claim for which no Requirement exists in the
relevant artefacts.

## REQ_VAL_GRAPH: Validate Graph

An error is reported for an invalid tracing graph. A Tracing Graph is invalid,
if:
*   there is a loop
*   a Node has no edges leading in or out


##  REQ_CONFIG: Simple Configuration in One File

All Configuration is stored in a single file using a common Format that is
editable for humans and machine readable.


## REQ_QUERIES: Configurable Information Granularity

Instead of returning every detail about the Tracing, specific parts of
information can be queried, so that the output is as slim as possible and only
relevant information is computed.

## REQ_FAST: Fast

Show results quickly, especially if only a small query is given.

## REQ_NO_OVERCACHING: No over-caching

If the user has to flush the cache, this is a bug in the Program.


## REQ_CLI: Offer a simple Command Line Interface

For ease of integration into other tools, all functionality must be available via a CLI.


# Usecases

## UC_VALIDATE: Validate Configuration

The configuration is loaded, the Tracing Graph validated. This Step should be fast.

## UC_CACHE_STATUS: Query Parsing State

All artefacts are checked for changes since last parsing

## UC_PARSE: Parse Artefacts

A Set of artefacts are parsed, reporting all requirements and errors.


Parameters:
*   Artefacts to Parse

## UC_TRACE: Compute Tracing

All requirements are matched up and down the Tracing Graph. The results are
stored in a file and bad tracing is reported.

Parameters:
*   Tracing Report

## UC_CHECK: Check for correct Tracing

Like `UC_TRACE` but the only output of interest is whether there were tracing errors or not

## UC_ANALYZE_SINGLE: Analyze a Requirement

For a Requirement, look up what it covers, and where it is covered itself and
where coverage is missing.

Parameters:
*   Requirement Id

## UC_ANALYZE_IMPACT: Analyze Dependencies of Requirement

For a Requirement, look up all requirements that it depends upon transitively.

Parameters:
*   Requirement Id
