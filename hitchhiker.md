The Hitchhiker Guide to ReqTrace
==================================

This file gives a guided tour through the code, as suggested [here](https://wonkodv.github.io/hitchhikers-guide-to-code/)

Version
-------

Last updated for [v0.1.1](https://github.com/wonkodv/reqtrace/tree/v0.1.1)

Major Components
----------------

*   [`controller::Controller::run()`](https://github.com/wonkodv/reqtrace/blob/v0.1.1/src/controller.rs#L136):
    This function executes each `job` that was requested by the command line or configured as default job
*   [Parsers](https://github.com/wonkodv/reqtrace/blob/v0.1.1/src/parsers/) turn one or more files into a list of
    requirements
*   [`common::artefact`](https://github.com/wonkodv/reqtrace/blob/v0.1.1/src/common.rs#L217): Holds one element in the tracing
    graph, like a document, a group of source files. Parses those files on demand using a parser of the right format. After
    parsing, holds all the requirements of the artefact
*   [`graph::Graph`](https://github.com/wonkodv/reqtrace/blob/v0.1.1/src/graph.rs#L102): Holds all the artefacts, and
    knows which covers which
*   [`trace::Tracing`](https://github.com/wonkodv/reqtrace/blob/v0.1.1/src/trace.rs#L51): Walks a Graph, collecting all the
    successful and failed traces from one artefact to another
*   [Formatters](https://github.com/wonkodv/reqtrace/blob/v0.1.1/src/formatters/) turn requirements, errors and tracings
    into (machine-) readable output
