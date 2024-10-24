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


## CLI


Command Line Interface is the entry point to obtain tracing information. The
Information will be looked up or computed and then emitted.

### ARCH_CLI: Command Line Interface

All functionality of the tool is exposed in a simple command line interface.
The CLI is shipped as a statically linked binary.
Other Tools can interface with the CLI to provide caching, lazy parsing of complicated input formats
and generation of complicated output formats

Covers:
*  REQ_MACHINE_FRIENDLY: Easy to include in automated work flows
*  REQ_INSTALL: Easy to install
*  REQ_FAST

## ARCH_CONTROLLER: Controller

The controller orchestrates the other components into a pipeline

*   Read Config
*   Pass Files to Parsers to obtain Requirements
*   Put Requirements into Artefacts
*   Assemble Artefacts into Graph
*   Give Graph to Tracer, which computes Tracing Information
*   Give Artefacts, the Graph, Tracing and any errors to Formatter which puts them into files

## ARCH_ARTEFACT: Artefact

An Artefact is a list of requirements, parsed from one or more files by a parser

## ARCH_PARSER: Parser

A Parser processes an input file and emits Requirements

## ARCH_GRAPH: Graph

The Graph organizes Artefacts into a directed graph without loops.
Each Node in the Graph represents a single Artefact. An Edge from Artefact `A`
to Artefact `B` expresses, that one or more Requirements in `B` cover one or
more requirements in `A`.

Edges belong to a group. Each requirement in an Artefact must be covered at
least once for each group of edges that lead out of it.
Covers:
*   REQ_TRACE

## ARCH_TRACE: Tracer

The tracer walks the graph and calculates tracing information

Covers:
*   REQ_TRACE
*   UC_TMX

## ARCH_TRACING: Tracing Information

A Tracing holds a Graph of Artefacts and all the Information about which requirements cover which, along which edges of the graph.

### ARCH_FORMATTER: Format output in requested Format

The formatter takes Artefacts, the Graph, the Tracing or a list of Errors and turns them into machine or human readable form.

Covers:
*   REQ_TRACE


