Requirement Tracing
===================


Tool for easy to use simple Requirement Tracing.




Wording
=======

Artefact
--------

documents that contain Information about Requirements, for example:

* PDF
* Markdown File
* spreadsheet
* source code file
* Report of running Tests


Requirement
-----------

A uniquely identified requirement, which is required in one artefact and covered
in another. For example, Use Case, Checklist Items from some specification,
a Function which is "covered" by a unittest-testlog, ...

A requirement has
* unique ID
* short representative title
* optionally a description
* list of other requirements that are "covered" by it
* Optionally a Parent Requirement


Coverage
--------

Requirement "A" is "covered" by Requirement "B" if it appears in the list of
items covered by "B" and the artefact of A is directly above B in the graph.

Tracing Graph
-------------

The relationship of artefacts. A Node is an artefact, an edge from A to B means
"every Requirement in A has to be covered by a Requirement in B". A is said to
be directly above B. There are no loops in the graph.  If multiple Edges lead
into a Node, it means that it covers requirements from multiple Artefacts. If
multiple edges lead from a Node, it means that its requirements have to be
covered by each of the connected nodes.

Derived Requirement
-------------------

A Requirement is "derived" if it does not cover another Requirement. It is "made
up" by the artefact.
