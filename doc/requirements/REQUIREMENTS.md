# Requirements

Things this tool should do in no particular order

## REQ_TRACE: Determine which requirements cover which

Compute tracing for each Requirement, whether it is covered, uncovered, covers
another requirement or is derived.

## REQ_UNIQUE_ID_v2: Requirements have a unique Identifier

Each requirement must be identifiable by a short, unique string.
All unicode symbols typically used as identifiers must be possible,
though parsers may restrict this

History:
*   v2: Unicode

## REQ_MACHINE_FRIENDLY: Easy to include in automated work flows

For ease of integration into other tools, all functionality must be available via a CLI.

## REQ_INSTALL: Easy to install

The tool should be distributed as an executable without depending on
libraries, files, etc.

## REQ_EXTENSIBLE: Extensible Parsing

If internal parsers are not able to work on an Artefact, external tools can be
incorporated.

## REQ_IDENTIFIEABLE: Show versions of input artefacts in output

When reading the output, each input must be clearly identifiable.
For example by its:
*   git describe
*   hash
*   file modification time

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

## REQ_PARSER_ERROR: Useful Parser Errors

Parser Errors give the precise location and type of the problem, for example filename with
line number of the artefact.

## REQ_LATE_ERROR: Collect Errors but continue processing

When errors are encountered in parsing, tracing or outputting, processing
continues as long as possible and then all errors are reported.

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


##  REQ_CONFIG: Simple Configuration in One File

All Configuration is stored in a single file using a common Format that is
editable for humans and machine readable.

## REQ_FAST: Fast

Show results quickly, especially if only a small query is given.

## REQ_CACHE_FRIENDLY: Work well with build systems that cache

Report all files which are consumed, so that build systems like make or
ninja can know when an input has changed an Given Data and a requested format, the formatter formats the given data in the
given format.rerun the tool.

# Usecases

## UC_PARSE: Parse Artefacts

A Set of artefacts are parsed, reporting all requirements and errors.

Parameters:
*   Artefacts to Parse

## UC_TMX: Create Traceability Matrix

All requirements are matched up and down the Tracing Graph. The results are
stored in a file and bad tracing is reported.

Parameters:
*   Tracing Report Format
*   Tracing Report File

## UC_CHECK: Check for correct Tracing

Like `UC_TRACE` but the only output of interest is whether there were tracing errors or not,
for use in CI/CD Pipelines.
