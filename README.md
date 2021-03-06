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

An artefact has a unique Id which does not change if its contents change.

Requirement
-----------

A requirement is the central data structure of this tool. It represents
requirements in the classical Software Engineering sense, as well as anything
else which can be traced against these requirements.

A Requirement always has an identifier that makes it unique in the entire
project. It can also have the following attributes:

*   Short representative title
*   Description
*   Version History
*   A set of Requirements it `Covers`
*   A set of Requirements it `Depends` on

Typical Requirements:

*   Use Case
*   Something sales promised
*   Something the end user needs
*   Performance Requirements
*   Design Decisions

Coverage
--------

Requirement "D" covers Requirement "U" if either:
*   "U" is in the set Requirements that "D" `Covers` or
*   "D" is in the set Requirements that "U" `Depends` on

Tracing Graph
-------------

The relationship of artefacts is represented as a Graph.
A Node is an artefact, an edge from A to B means
"every Requirement in A has to be covered by a Requirement in B". A is said to
be directly above B. There are no loops in the graph. If multiple Edges lead
into a Node, it means that it covers requirements from multiple Artefacts. If
multiple edges lead from a Node, it means that its requirements have to be
covered by each of the connected nodes.
An Artefact which only has edges leading into it is called a Leaf Artefact.
An Artefact which only has edges leading out of it is called a Root Artefact.

Requirements can covered

A Graph might Look Like this:


            REQUIREMENTS
              /     \    \
       Performance   \   USECASES
        TestLog       \      \
                       \     IntegrtionTestLog
                      DESIGN+
                      FORMATS
             _________/  \  \________
            /             \          \
          MANUAL           \       UnitTestLog
                          CODE





Leaf Requirement
----------------

A Leaf Requirement is a Requirement specified in a Leaf Artefact. It only covers
and does not itself require anything. So it isn't really a requirement in the
meaning of the word, but represented by the same data structure like
requirements in the tool. Typical Leaf-Requirements:

*   Warning Notice in the User Manual
*   Coverage Information in a Test Log
*   Checkpoints on a Review Checklist

Completeness
------------

The Completeness of a requirement show how completely traced it is, taking the
completeness of all requirements below it into account.

    Completeness(LeafRequirement) = 1
    Completeness(UncoveredReq) = 0
    Completeness(R) = Avg(Completenes(d) for d in R.Depends)

Derived Requirement
-------------------

A Requirement is "derived" if it does not cover another Requirement but comes
from an Artefact which is not a root artefact. It is "made up" by the artefact.




Best Practices
==============

Requirement IDs
---------------

While this tool can deal with any valid unicode string, Requirement IDs are
easier to use the more of the following rules they meet:

*   Stands out in text. This means not mistakable for another word.
*   Easy to parse. A simple regex should reliably find it.
*   Filename safe: No whitespaces, slashes, backslashes, etc. (no tilde?)
*   URL safe: none of `#?&;/`
*   Code Safe: should be embeddable in comments, strings, maybe even as
    identifier
*   DoubleClickable: If a user can double click the id in an editor/browser and
    select exactly the id, not more, not less, this makes it easier to work
    with them.
*   Markdown Safe: if the id does not have special meaning for markdown, it can
    be embedded into the documentation without quoting, escaping, etc.
    This is fairly well defined except for an underscore in the middle of
    a word, which is highlighted as error by vim, but intentionally accepted by
    [GFM](https://github.github.com/gfm/#emphasis-and-strong-emphasis "GitHub
    Flavored Markdown")
*   Allow to partition the ID into multiple fields, for example Type, name and
    version. This means some non-word character is needed.

This all boils down to:

*   only letter, numbers and underscore.
*   underscore only in the middle (markdown)
*   Start with a letter (valid identifier in programming languages)
*   At least 3 letters

The Regular Expression which is used by the build in Formats is therefore:

    [A-Za-z][a-zA-Z0-9_]+[a-zA-Z0-9]


Examples:

    UC_DeleteAllFiles
    REQ123
    REQ_123
    IMP_FuncName
    UT_FuncName
    IT_Req_123

    REQ_123_v1
    REQ

Numbers in the requirement ids are not very useful for the reader, but easy to get
unique. They will often come from a database based tool.

Talking requirement ids require to find a unique short fitting name, which is
easy for a limited number of requirements but becomes difficult when there are
many fine grained requirements.

Namespacing
-----------

It should be obvious from glancing at a requirement where it comes from.
This requires a form of namespacing, typically by using Prefixes that hint at
the artefact a requirement comes from. UC_123 describes a usecase, UT_FuncName
is either the Unit test code that tests Funcname, or better yet, the line in the
testlog which states that the unittest for funcname ran successfull.

Versioning
----------

When a requirement changes significantly, it will usually make sense to break
existing coverage, which can easily be done by appending a version number to the
identifier and incrementing it on substantial changes.

Tags
----

Tags can be used to categorize or filter requirements
