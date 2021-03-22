Justification for Unsafe Code
=============================

This file justifies the use of unsafe code in the examples. It uses the standard
markdown requirement Artefact type for simplicity.

## SAFE_EXAMPLE: Example

This is the `Description` of the requirement which covers `UNSAFE_EXAMPLE`.  You
can only cover one requirement from another. Instead of using a Dedicated
Artefact Type, `Markdown Requirements` is used for this file.  The ID of the
Justification Requirement must be unque throughout the Project even though it is
not referenced anywhere again. Exchanging the `UNSAFE_` prefix for `SAFE_` has
no meaning to the tracing, it only serves as a simple way to come up with
a unique ID. The Covering happens through a reference in the `Covers` section of
this Requirement. The Justification of why code uses unsafe should mention why
there is no way without the use of `unsafe` and an analysis of why the code
works as intended without negative side effects.

Necessity:
The keyword has to be used at least once in this project to give a good example.
The real code base does not need `unsafe` so the example was created.

Safe:
The code returns a constant which can have no negative side effects.

Covers: UNSAFE_EXAMPLE

