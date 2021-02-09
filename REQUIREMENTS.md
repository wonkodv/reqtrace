
Requirements
============

REQ-001: Allow Version Control
-------


All Config, state, intermediate results are in Plain Text based formats that are usable in
Version Control

REQ-002: Easy to install
----------------

no package management, libraries, dependencies


REQ-004: Extensible Parsing
----------------

If internal parsers are not able to work on an Artefact, external tools can be incorporated.


REQ-005: Machine Readable Output
----------------

The Result is presented in a format that can easily be read by other tools

REQ-006: Well defined Formats
----------------

To work with external programs as parsers or to process the output, the formats used must be well
defined.

REQ-007: Artefact Grouping
----------------

A set of Artefacts can act as one Node in the Tracing Graph


REQ-008: Useful Parser Errors
----------------

Parser Errors give the precise location and type of the problem, for example filename with
line number of the artefact.

REQ-009: Lazy
----------------

Expensive calculations are cached and redone when needed.


REQ-010: No over-caching
----------------

If the user has to flush the cache, this is a bug in the Program.


REQ-011: Match by ID
------------------

A Requirements covers another by its ID.

REQ-012: Check matching title
------------------

When Covering, the title of the covered requirement can be validated as well to make the documents
more readable for humans without introducing contradicting statements.

For example, the implementation of this requirement could state `Covers REQ-012 (Check matching
title)`. Only the ID would be used for matching (and has to be unique) but if the wrong title is
given, this produces an error. The format and validating the real title against the covered title is
the responsibility of the parser.


REQ-013: Report non unique Requirement IF
----------------

An error is reported if two artefacts in the same Tracing Graph define requirements with the same
ID.

REQ-014: Incorrect Coverage
----------------

An error is reported for an item listed as "covered" for which no Requirement exists in the relevant artefacts.


REQ-015: Hierarchical Requirements
----------------

A Requirement can have a Parent Requirement. The Child Requirement has to be covered. If all Child
Requirements are Covered, the parent requirement is covered implicitly.

This allows the Parent Requirement to cover higher level requirements while the child requirements
are covered by more fine grained Requirements downstream.



REQ-016: Graph validation
----------------

An error is reported for an invalid tracing graph. A Tracing Graph is invalid, if there is a loop.

Usecases
========

## UC-001: Validate Configuration

The configuration is loaded, the Tracing Graph validated. This Step should be fast.

## UC-002: Query Parsing State

All artefacts are checked for changes since last parsing


## UC-003: Trace

All requirements are matched up and down the Tracing Graph. The results are stored in a file and bad
tracing is reported.
