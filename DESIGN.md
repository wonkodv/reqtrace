# Design


## DSG_REQ_FIELDS: Requirement Fields

Attributes of a requirement that this tool requires:
*   ID: a short string that uniquely identifies this requirement

Optional Attributes that are handled:
*   Title:  Text that briefly summarizes this requirement (on line)
*   Description: Text that gives detailed description
*   Coverage: List of requirement IDs that are covered by this one
*   Dependencies: List of requirement IDs which cover this one
*   Tags:   List of Strings that can be used to categorize requirements

Attributes inferred during requirement Parsing:
*   Location:   Artefact that defines this requirement and the location inside
    the artefact where it is defined

Arbitrary Additional Attributes are possible, for example

*   History: Text about how this requirement changed
*   Comment: Text with even more details, further reading, etc. that has a lower
    priority than Description which may be excluded from reports

Covers:
*   REQ_UP
*   REQ_DOWN
*   REQ_DELEGATION


## DSG_ART_FIELDS: Artefact Fields

*   ID
*   paths:  List of Paths or pattern with which to find the files
*   parser:   id of a parsing strategy
*   parser argument: Object that is passed to the parser
*   caching: boolean, whether to cache or parse on every access


## Data Format Considerations

The data format for internal Caching, data exchange, and machine readable output
needs to be well supported, text based and produce a reasonably small diff if
little changed.

Candidates:
*   JSON
*   ?

for smaller diff, all lists should be sorted 


## Computing effort Considerations

Depending on the artefact type, parsing an artefact may not be slower than
retrieving the information from a cache, but it may also be a lot slower, if the
artefact format is complicated (large pdf) and the cache format good.

To minimize the computing effort for accessing the cache, the following datasets
are stored in a file each:
*   Requirements parsed from one artefact
*   Coverage Links from one artefact to another

To prevent over caching a fingerprint of the Artefact is stored in the cached
requirements, and the fingerprint of all related artefacts is stored with the
coverage links.

### DSG_JSON_CACHE_SORT: JSON Cache sorted

Sort lists, smaller diff

### DSG_JSON_CACHE: JSON for Storing State

JSON is used to store parsing results and computed coverage links

Covers: REQ_VCS, REQ_LAZY

### DSG_JSON_IMPORT: JSON for Importing Requirements

Artefacts which can not be parsed by the tool are

Covers: REQ_VCS, REQ_EXTENSIBLE

### DSG_JSON_IMPORT: JSON for Exporting Results

Artefacts which can not be parsed by the tool are

Covers: REQ_VCS, REQ_MACHINE_READABLE

### DSG_CACHE_FINGERPRINT: Fingerprint in Caches

The fingerprint which indicates whether a cache computed from:
*   The tool's version
*   The hash of all artefacts relevant to this cache

Covers: REQ_LAZY

### DSG_ART_FINGERPRINT: Fingerprint of Artefacts

The fingerprint of an artefact is computed by computing a hash over:
*   hash over the parsing-type of the artefact
*   for all files that make up an Artefact:
    *   The sha256 of the file if it is small
    *   The modification time as and size if it is large

Covers:
*   REQ_NO_OVERCACHING

Comment:
Since the version is included in the fingerprint, the details can be changed
easily.
