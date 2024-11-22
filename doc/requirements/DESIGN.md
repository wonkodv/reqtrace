# Design

## Command Line Interface

All functionality is provided by a central controller, which can be
interfaced from the command line.


### DSG_CLI: Command Line Interface

The tool should be invoked via a simple CLI and set the exit code to indicate if a job was
successful.

Covers:
*   REQ_MACHINE_FRIENDLY: Easy to include in automated work flows
*   ARCH_CLI: Command Line Interface

### DSG_CLI_RETURN_CODE: Set return Code to indicate success

Set the process' return code to:
*   `2` if there were fatal errors (invalid configuration or similar)
*   `1` if there were errors (file not found, parser errors, uncovered requirement, derived
    requirement, ...)
*   `0` otherwise

Covers:
*   REQ_MACHINE_FRIENDLY: Easy to include in automated work flows
*   UC_CHECK: Check for correct Tracing
*   ARCH_CLI: Command Line Interface

## Controller

The Controller Ties all the other Components together and offers all functionality to the
CLI.
It does so using Jobs.
A Job defines, which output information should be emitted, and in what format.
The controller then does the necessary Steps to provide this information.

### DSG_JOBS: Jobs encode requested behavior

Jobs configure what work should be done, and in what format the results should be
presented. a Job has the following Fields:
*   Query: What to do
*   Format: How results are Presented
*   File: Where results should be stored (`-` acts as stdout)
*   SetReturncode: Wether errors encountered by this query affect the CLI Process's return
    code.

Covers:
*   ARCH_CONTROLLER
*   REQ_CONFIG: Simple Configuration in One File


### DSG_CTRL_CONFIG: Single Config File

The Controller the following information from a single config file:

*   Artefacts
*   Relation between Artefacts
*   Jobs

Covers:
*   REQ_CONFIG: Simple Configuration in One File
*   ARCH_CONTROLLER

### DSG_CTRL_PARSE: Parse all Artefacts

If the Job.Query is Parse or Trace:
For all Artefacts, parse the content of all files of the artefact to the correct parser.
Collect all requirement, any encountered errors and any inspected files in the Artefact.

Covers:
*   ARCH_CONTROLLER
*   UC_PARSE: Parse Artefacts

### DSG_CTRL_DETECT_DUPLICATE_REQS: Detect duplicates

After Parsing, when assembling Requirements, detect duplicate requirements in the same Artefact

Covers:
*   REQ_UNIQUE_ID_v2
*   ARCH_ARTEFACT


### DSG_CTRL_GRAPH: Construct Graph

If the Job.Query is Parse or Trace:
After Parsing, Assemble the artefacts in a Graph, which contains all artefacts,
the relations between artefacts, and graph configuration errors.

Covers:
*   ARCH_CONTROLLER

### DSG_CTRL_TRACE: Trace Requirements

If the Job.Query is Trace, pass the Graph to the Tracer to get a Traced Graph.


Covers:
*   ARCH_CONTROLLER
*   UC_TMX: Create Traceability Matrix
*   UC_CHECK: Check for correct Tracing

### DSG_CTRL_FORMAT: Format Job Output

Pass the output of running Job.Query to the Formatter specified by Job.Format and Write to
Job.File.

Covers:
*   ARCH_CONTROLLER

#### DSG_CTRL_RETURN_CODE: Return Code Indicates if Job found Errors

If Job.SetReturnCode, and running job.query produced any errors, indicate them to the CLI,
so it can set an error return code.

Covers:
*   ARCH_CONTROLLER

## Parser

### DSG_PARSER: Parse Data

The Parser is called with data from one file, and the format of that data.
It returns Lists with:
*   All found Requirements
*   All encountered Errors

Covers:
*   ARCH_PARSER

## Tracer

### DSG_TRACE: Walk the Graph and trace requirements

The Tracer is passed a Graph. The Tracer inspects all relations of the graph,
recording tracing information as it is encountered.
After all Relations are processed, a final validation pass uncovers any unresolved
problems and records Errors for them.

The Tracer Performs the following Steps:
1.  Collect Requirements from all Artefacts
2.  Trace all Relations
3.  Validate

Covers:
*   ARCH_TRACE

#### DSG_TRACE_COLLECT: Collect Requirements from Artefact

All requirements from all artefacts are added to  index.
Each requirement is also added to a list of derived requirements of its artefact
All Covers and Depends references of each requirement are added to a list of illegal
references, from which they will be remove if they traced validly.

Covers:
*   ARCH_TRACE

#### DSG_TRACE_DETECT_DUPLICATE: Detect duplicate Requirements in different Artefacts

While collecting requirements, if there are two requirements the same identifier, log an error.

See DSG_ART_DETECT_DUPLICATE for detecting duplicates in the same artefact

Covers:
* REQ_UNIQUE_ID_v2


#### DSG_TRACE_RELATION: Trace Relation

For each Relation, inspect the requirements of the upper artefact and perform the
following steps:

1.  find lower requirements that match upper's `depends`
2.  for all lower artefacts, find lower requirements that cover upper
3.  for all coverages found:
    1.  remove them from the list of invalid references
    2.  remove lower from the list of derived requirements
    3.  Record the coverage information with the relation
    4.  if covered with title, add an error if the title is not matched
    correctly
4.  if no coverage was found, add the requirement to the list of uncovered
    requirements of the relation

Covers:
*   ARCH_TRACE


#### DSG_TRACE_UPWARDS: Trace upwards using `covers` attribute

Requirement U covers Requirement D if U.id appears in D.Covers.

Covers:
*   REQ_TRACE
*   REQ_MATCH_ID
*   REQ_UP
*   ARCH_TRACE

#### DSG_TRACE_DOWNWARDS: Trace downwards using `depends` attribute

Requirement U covers Requirement D if D.id appears in U.Depends.

Covers:
*   REQ_TRACE
*   REQ_MATCH_ID
*   REQ_DOWN
*   ARCH_TRACE

#### DSG_TRACE_DERIVED: Record requirements that do not cover anything

When Tracing, All requirements are first added to their artefacts' deerived list.
Whenever a requirement R covers an upper requirement U, R is removed from it's artefact's
derived list.
All remaing requirements are derived.

Covers:
*   REQ_TRACE
*   ARCH_TRACE

#### DSG_TRACE_UNCOVERED: Record requirements that are not completely covered

When tracing a Requirement R along a Relation, if neither downward nor upward coverage is
found, that requirement is added to the list of uncovered requirements along that
relation.

A requirement can be covered along on edge while being uncovered along another.

For example, in this project, a DSG Requirement has to be covered Both by a unittest
`design => [unittests]` and by either code or a format specification `design => [format,
code]`.

Covers:
*   REQ_TRACE
*   ARCH_TRACE

#### DSG_TRACE_CHECK_TITLE: When tracing upwards or downwards match title

When tracing Upwards or Downwards, emit an error if the title of the coverage does
not match the title of the covered requirement

Example:

    ### REQ_U: Title of Upper

    An Upper Requirement

    ### REQ_D: Title of Lower

    A Lower Requirement that covers REQ_U with an exactly mathcing title.

    Covers:
    *   REQ_U: Title of Upper

Covers:
*   REQ_TRACE
*   REQ_VAL_TITLE: Check matching title
*   ARCH_TRACE

#### DSG_TRACE_REFERENCE_EXIST: Coverage Links must exist

For each Requirement that is encountered, store all "covers" and "depends" references
in a list of invalid references.
When the Requirement is successfully covered against a requirement matching
that reference, it is removed from the list of invalid references.

This approach is necessary to walk the graph only once.

Covers:
*   REQ_TRACE
*   REQ_DOWN
*   REQ_UP
*   ARCH_TRACE
*   REQ_VAL_COVERAGE

### DSG_TRACE_VALIDATE_EDGE: Validate Edge is used at least once

After tracing, if an edge can be found, along which no requirement is
covered, an error is emitted. This is likely a misconfiguration.

TODO: rewrite to be single pass

Covers:
*   ARCH_TRACE


## Formatter


### DSG_FORMATTER: Formatter

The formatter is called with
*   either a Graph, or a Traced Graph
*   a Format
and formats the given information according to format.

If the output format is not easily created from the Traced Graph, it passes the TYraced
Graph to the Aggregator, and the formats an aggregated Graph.

Covers:
*   ARCH_FORMATTER

### DSG_AGGREGATOR: Cross Referenced Trace Graph

The aggregator creates lookup tables, where all tracing information is indexed by either
requirement id or artefact id, so formatter does not have to crawl through relations when
trying to write coverage information for a requirement.



## Requirements

Requirement Objects are dumb containers containing the data of one Requirement

### DSG_REQ_FIELDS: Requirement Fields

Attributes of a requirement that this tool requires:
*   ID: a short string that uniquely identifies this requirement

Optional Attributes that are handled:
*   Title:  Text that briefly summarizes this requirement (on line)
*   Description: Text that gives detailed description
*   Coverage: List of requirement IDs that are covered by this one
*   Dependencies: List of requirement IDs which cover this one
*   Tags:   List of Strings that can be used to categorize requirements

Attributes inferred during requirement Parsing:
*   Location:   Artefact that defines this requirement and the location inside
    the artefact where it is defined

Arbitrary Additional Attributes are possible, for example

*   History: Text about how this requirement changed
*   Comment: Text with even more details, further reading, etc. that has a lower
    priority than Description which may be excluded from reports

Covers:
*   REQ_UP
*   REQ_DOWN
*   ARCH_REQUIREMENT


## Artefact                                                      `

An Artefact represents a (group of) files.
They load cached requirements or parse
them as needed.

### DSG_ART_FILES: Artefact loads one or more Files

An Artefact represents one or more files of the same type.

Covers:
*   ARCH_ARTEFACT

### DSG_ART_PARSE_COLLECT_ERRORS: Collect errors but keep parsing
While parsing artefacts, all encountered errors are stored and parsing continues.

Covers:
*   REQ_LATE_ERROR
*   REQ_PARSER_ERROR
*   ARCH_ARTEFACT


### DSG_ART_IGNORE_DERIVED: Ignore Derived Requirements

Artefacts can be configured to ignore derived requirements

Covers:
*   ARCH_ARTEFACT

## DSG_GRAPH: Artefact Graph

The Graph holds all Artefacts, and a list of relations describing which artefact covers
which

Covers:
*   ARCH_GRAPH

### DSG_GRAPH_RELATION: Artefact Relationships

A Relation has fields:
*   Upper Requirement ID
*   List of Lower Requirement IDs

Covers:
*   ARCH_GRAPH

## DSG_TRACED_GRAPH: Tracing Information of Grpah

The Traced Graph has Fields:
*   Artefacts, indexed by artefact id
*   Lists of Derived Requirements, indexed by artefact id
*   List of Traced Relations
    *   Relations
    *   list of Coverages
        *   Upper Requirement
        *   Lower Requirement
        *   Location of the Reference (the Covers Line)
    *   List of derived Requirements
    *   List of Tracing Errors

Covers:
*   ARCH_TRACED_GRAPH
