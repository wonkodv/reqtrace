# Architecture

The Tool consists of the following Components

*   Formatters:     Format Information
*   Parsers:        Extract Requirements from Files in various Ways
*   Artefacts:      Manage Input Files, depends on Parser / Cache
*   Tracing Graph:  Manage Artefacts compute Tracing
*   Controller:     Read Config, Build Graph, query Information, pass it to
                    formatter
*   CLI:            Frontend to the Controller




## CLI


Command Line Interface is the entry point to obtain tracing information. The
Information will be looked up or computed and then emitted.

### ARCH_CLI: Command Line Interface

All functionality of the tool is exposed in a simple command line interface.

Covers:
*  REQ_MACHINE_FRIENDLY: Easy to include in automated work flows

## ARCH_CONTROLLER: Controller

The Controller Reads the Configuration file, and builds the tracing graph as
needed to answer the query. It then obtains the queried information, stores it
to the requested location in the requested format


## ARCH_ARTEFACT: Artefact

An artefact represents Requirements from a file, or several related files.
Artefacts parse files with Parsers and store requirements

## ARCH_PARSER: Parser

A Parser processes an input file and emits Requirements

Covers:
*   REQ_TRACE

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

### ARCH_FORMATTER: Format output in requested Format

The formatter stores all available kinds of information in different
selectable formats

Covers:
*   REQ_TRACE
