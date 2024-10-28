# Architecture

The Tool consists of the following Components

*   **CLI** thin wrapper around Controller, making its functions available on commandline
*   **Controller** Orchestrates the other components
*   **Parser** Takes external files and turns them into Artefacts, exist in various formats
*   **Tracer** Walks the graph of artefacts, collecting information about which requirements cover which
*   **Formatter** Takes requirements, errors and tracing


Data Models
*   Config      User Configuration
*   Requirement
*   Artefact    Description of input files and list of parsed requirements
*   Graph       Holds a graph of artefacts
*   Tracing     Information about which requirement covers which


## Active Components

### ARCH_CLI: Command Line Interface

All functionality of the tool is exposed in a simple command line interface.
The CLI is shipped as a statically linked binary.
Other Tools can interface with the CLI to provide caching, lazy parsing of complicated input formats
and generation of complicated output formats

Covers:
*  REQ_MACHINE_FRIENDLY: Easy to include in automated work flows
*  REQ_INSTALL: Easy to install
*  REQ_FAST

### ARCH_CONTROLLER: Controller

The controller orchestrates the other components into a pipeline

*   Read Config
*   Pass Files to Parsers to obtain Requirements
*   Put Requirements into Artefacts
*   Assemble Artefacts into Graph
*   Give Graph to Tracer, which computes Tracing Information
*   Give Artefacts, the Graph, Tracing and any errors to Formatter which puts them into files

### ARCH_PARSER: Parser

A Parser processes an input file and emits Requirements.
There are Parsers for several File formats, dependant on the configuration for an
input file.

Covers:
*   REQ_EXTENSIBLE: Extensible Parsing
*   UC_PARSE: Parse Artefacts

### ARCH_TRACE: Tracer

The tracer walks the graph and calculates tracing information

Covers:
*   REQ_TRACE
*   UC_TMX

### ARCH_FORMATTER: Format output in requested Format

The formatter takes Artefacts, the Graph, the Tracing or a list of Errors and turns them into machine or human readable form.

Covers:
*   UC_TMX
*   REQ_HUMAN_READABLE
*   REQ_MACHINE_READABLE

## Data Models

*   Requirement
*   Artefact    Description of input files and list of parsed requirements
*   Graph       Holds a graph of artefacts
*   Tracing     Information about which requirement covers which
*
### ARCH_REQUIREMENT: Requirement

A Requirement is the basic unit of information that this tool operates on.
A Requirement object stores one typical software requirement (for example
"DSG_CLI_RETURN_CODE")

A requirement also stores information about purely covering information
(for example: `main_rc`  which covers DSG_CLI_RETURN_CODE)

### ARCH_ARTEFACT: Artefact

An Artefact is a list of requirements, parsed from one or more files by a parser


### ARCH_GRAPH: Graph

The Graph organizes Artefacts into a directed graph:

Each Node in the Graph represents:
*   either a single Artefact,
*   or a Relation.

An Edge must:
*   point from an Artefact to a Relation or,
*   from a Relation to an artefact.

Artefacts can have any number of edges in or out.

Relations can have one incoming edge, and one or more out going edges.

For any subgraph, with
*   an Edge from Artefact U to Relation R
*   one or more Edges from Relation R to Artefacts D1, D2, ...
All requirements of U must be covered by at least one requirement one of the
artefacts D1, D2,  ....


Covers:
*   REQ_TRACE

## ARCH_TRACED_GRAPH: Tracing Information of Grpah

The Traced Graph holds all the information of the Graph, plus:
*   For each Relatiuon:
    *   Which Requirements are Covered by which
    *   Which Requirements are not covered along this relation
*   For each Artefact
    *   Which requirements are derived (do not cover another Requiremnt)
