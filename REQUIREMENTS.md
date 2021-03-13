
# Requirements

## REQ_VCS: Allow Version Control


All Config, state, intermediate results are in Plain Text based formats that are usable in
Version Control and produce usable diff.

## REQ_INSTALL: Easy to install

no package management, libraries, dependencies


## REQ_EXTENSIBLE: Extensible Parsing

If internal parsers are not able to work on an Artefact, external tools can be
incorporated.


## REQ_MACHINE_READABLE: Machine Readable Output

The Result is presented in a format that can easily be read by other tools

## REQ_HUMAN_READABLE: Human Readable Output

The Result is presented in a format that can easily be read, navigated, shared
and searched by humans.

## REQ_FORMATS: Well defined Formats

To work with external programs as parsers or to process the output, the formats used must be well
defined.

## REQ_GROUPING: Artefact Grouping

A set of Artefacts can act as one Node in the Tracing Graph

## REQ_UP: Upward Coverage

A Requirement covers a higher one by including the id of the higer one in its
Coverage attribute.

## REQ_DOWN: Downward Coverage


A Requirement is covered by a lower one by including the id of the lower one in
its Dependencies attribute.

## REQ_DELEGATION: Coverage Delegation

A Requirement delegates to another requirement in the same artefact or to
a lower requirement by including the id of the lower one in
its Dependencies attribute.

It does not need to be and can not be covered itself through upward coverage of a lower
requirement.

## REQ_ERROR: Useful Parser Errors

Parser Errors give the precise location and type of the problem, for example filename with
line number of the artefact.

## REQ_FAST: Fast

Show results quickly, especially if only a small query is given

## REQ_NO_OVERCACHING: No over-caching

If the user has to flush the cache, this is a bug in the Program.


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

# Usecases

## UC_VALIDATE: Validate Configuration

The configuration is loaded, the Tracing Graph validated. This Step should be fast.

## UC_STATUS: Query Parsing State

All artefacts are checked for changes since last parsing

## UC_PARSE: Parse Artefacts

A Set of artefacts are parsed, reporting any errors found

## UC_COMPUTE: Compute Tracing

All requirements are matched up and down the Tracing Graph. The results are
stored in a file and bad tracing is reported.

## UC_ANALYZE_SINGLE: Analyze a Requirement

For a Requirement, look up what it covers, and where it is covered itself and
where coverage is missing.

## UC_ANALYZE_IMPACT: Analyze Dependencies of Requirement

For a Requirement, look up all requirements that it depends upon transitively.

## UC_EXPORT_PARSING: Export Parsed Requirements

Information about the requirements of a set of artefacts is exported in
a selected Format

## UC_EXPORT_TRACING: Export Tracing

Information about the coverage of a set of requirements or artefacts is exported
in a selected format
