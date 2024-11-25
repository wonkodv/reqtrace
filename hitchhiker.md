The Hitchhiker Guide to ReqTrace
==================================

This file gives a guided tour through the code, as suggested [here](https://wonkodv.github.io/hitchhikers-guide-to-code/)

Version
-------

Last updated for [v0.2.0](https://github.com/wonkodv/reqtrace/tree/v0.2.0)

Major Components
----------------

All Major Components take data in, and give data back.
Controller manages the flow of Data from One component
to the other. CLI manages the Controller.

*   [`controller::Controller::run()`](https://github.com/wonkodv/reqtrace/blob/v0.2.0/src/controller.rs#L282):
    This function executes each `job` that was requested by the command line or configured as default job
*   [Parsers](https://github.com/wonkodv/reqtrace/blob/v0.2.0/src/parsers/) turn one or more files into a list of
    requirements. They are called from Controller's
    [`parse_single_file`](https://github.com/wonkodv/reqtrace/blob/v0.2.0/src/controller.rs#L56)
    or `_multiple_files`)
*   [`models::Artefact`](https://github.com/wonkodv/reqtrace/blob/v0.2.0/src/models.rs#L260): Hold all the requirements from one (group of) File(s).
*   [`models::Graph`](https://github.com/wonkodv/reqtrace/blob/v0.2.0/src/models.rs#L364): Holds all the artefacts, and
    knows which covers which
*   [`trace::Tracer:new()`](https://github.com/wonkodv/reqtrace/blob/v0.2.0/src/trace.rs#L54): Walks a Graph and computes Trace Data. Calling `.data()` produces a [`models::TracedGraph`](https://github.com/wonkodv/reqtrace/blob/v0.2.0/src/models.rs#L410) with all tracing info.
*   [`aggregator::AggregatedGraph`](https://github.com/wonkodv/reqtrace/blob/v0.2.0/src/aggregator.rs#L44) walks a TracedGraph
    and creates lookup tables to index requirement info
    by Artefact id or requirement id. This is used by
    markdown formatter to link to related info.
*   [Formatters](https://github.com/wonkodv/reqtrace/blob/v0.2.0/src/formatters/) turn requirements, errors and tracings
    into (machine-) readable output. The markdown

Documentation
-------------

*   High level requirements and use cases are documented in
    [REQUIREMENTS.md](https://github.com/wonkodv/reqtrace/blob/v0.2.0/doc/requirements/REQUIREMENTS.md)
*   Components are described in
    [ARCHITECTURE.md](https://github.com/wonkodv/reqtrace/blob/v0.2.0/doc/requirements/ARCHITECTURE.md)
*   Decisions how to implement the architecture are documented in
    [DESIGN.md](https://github.com/wonkodv/reqtrace/blob/v0.2.0/doc/requirements/DESIGN.md)
    and
    [FORMATS.md](https://github.com/wonkodv/reqtrace/blob/v0.2.0/doc/requirements/FORMATS.md)
*   Code and TestsLogs reference the requirements documented in the above files
*   Which requirements cover which is described in the [Traceability Matrix (TMX)](https://github.com/wonkodv/reqtrace/blob/v0.2.0/reports/tmx.md). As
