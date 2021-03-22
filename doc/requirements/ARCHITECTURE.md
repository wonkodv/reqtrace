# Architecture

The Tool consists of the following Components

*   Formatters:     Format Information
*   Parsers:        Extract Requirements from Files in various Ways
*   Cache:          Store Information for later retrieval
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
*  REQ_USER_FRIENDLY: Simple to use Interface
*  REQ_MACHINE_FRIENDLY: Easy to include in automated work flows

## Controller

The Controller Reads the Configuration file, and builds the tracing graph as
needed to answer the query. It then obtains the queried information, stores it
to the requested location in the requested format

### ARCH_CTRL_CONFIG: Single Config File

The Controller reads all information about the project structure from one single
file.

Covers:
*   REQ_CONFIG: Simple Configuration in One File


### ARCH_CTRL_ASM_LAZY: Assemble Graph as Needed

The Controller assembles those parts of the tracing graph, that are needed to
answer a query an not more.

Covers:
*   REQ_QUERIES: Configurable Information Granularity
*   REQ_FAST: Fast

### ARCH_CTRL_QUERY: Gather Information

The Controller gathers Information from the tracing graph or other components as
requested by a query.

Covers:
*   REQ_QUERIES: Configurable Information Granularity

### ARCH_CTRL_FORMAT: Output Format can be Chosen

The Controller passes the gathered information to the configured / requested
Formatter.

Covers:
*   REQ_CONFIGURABLE_OUTPUT: The Output Format is Configurable


## Tracing Graph

The Tracing Graph organizes Artefacts into a directed graph without loops.
Each Node in the Graph represents a single Artefact. An Edge from Artefact `A`
to Artefact `B` expresses, that one or more Requirements in `B` cover one or
more requirements in `A`.

Edges belong to a group. Each requirement in an Artefact must be covered at
least once for each group of edges that lead out of it.

### ARCH_TG_VALIDATE_LOOP: Validate that the Graph has no Loops

At leas One Requirement must be matched by an edge, otheriwse an error is
emitted.

### ARCH_TG_VALIDATE_EDGE: Validate Edge is used at least once

At leas One Requirement must be matched by an edge, otheriwse an error is
emitted.

### ARCH_TG_TRACE_EDGE_GROUP: Trace Edge Groups

Edges belong to a group. Each requirement in an Artefact must be covered at
least once for each group of edges that lead out of it.

### ARCH_TG_TRACE_COVERED: 
### ARCH_TG_TRACE_UNCOVERED: 
### ARCH_TG_TRACE_DERIVED: 

## Artefacts

TODO

## Cache
TODO
## Parser
TODO
## Formatter
TODO
