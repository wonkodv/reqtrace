

# Tracing Errors

*   DSG_JSON_CACHE depends on unknown Requirement FMT_JSON
    doc/requirements/DESIGN.md:113
*   DSG_JSON_EXPORT depends on unknown Requirement FMT_JSON
    doc/requirements/DESIGN.md:133
*   DSG_JSON_IMPORT depends on unknown Requirement FMT_JSON
    doc/requirements/DESIGN.md:125
*   FMT_MARKDOWN_REQUIREMENT depends on unknown Requirement FMT_MD_ATTRIBUTES
    doc/requirements/FORMATS.md:95
*   FMT_MARKDOWN_REQUIREMENT depends on unknown Requirement FMT_MD_DESC
    doc/requirements/FORMATS.md:93
*   FMT_MARKDOWN_REQUIREMENT depends on unknown Requirement FMT_MD_DESC_HEADINGS
    doc/requirements/FORMATS.md:94
*   FMT_MARKDOWN_REQUIREMENT depends on unknown Requirement FMT_MD_START
    doc/requirements/FORMATS.md:92
*   README depends on unknown Requirement REQ_LATE_ERRORS
    README.md:27


# Uncovered Requirements

*   [DSG_ART_CACHING](#dsg_art_caching-cache-parsing-results "Cache Parsing Results")
*   [DSG_ART_CONFIG](#dsg_art_config-artefact-configuration-fields "Artefact Configuration Fields")
*   [DSG_ART_EXTERNAL_PARSER](#dsg_art_external_parser-external-artefact-parser "External Artefact Parser")
*   [DSG_ART_PARSE](#dsg_art_parse-artefact-parsing "Artefact Parsing")
*   [DSG_CLI](#dsg_cli-offer-a-simple-command-line-interface "Offer a simple Command Line Interface")
*   [DSG_EXPORT_DATA](#dsg_export_data-export-data-to-stdout "Export data to stdout")
*   [DSG_EXPORT_ERRORS](#dsg_export_errors-export-errors-to-stderr "Export errors to stderr")
*   [DSG_EXPORT_FORMAT](#dsg_export_format-allow-selecting-the-export-format "Allow Selecting the Export Format")
*   [DSG_EXPORT_FORMAT_JSON](#dsg_export_format_json-export-to-json "Export to JSON")
*   [DSG_EXPORT_FORMAT_MARKDOWN](#dsg_export_format_markdown-export-to-markdown "Export to Markdown")
*   [DSG_EXPORT_FORMAT_TEX](#dsg_export_format_tex-export-to-tex "Export to TEX")
*   [DSG_FINGERPRINT_ART](#dsg_fingerprint_art-fingerprint-of-artefacts "Fingerprint of Artefacts")
*   [DSG_FINGERPRINT_TRACING](#dsg_fingerprint_tracing-fingerprint-of-tracing "Fingerprint of Tracing")
*   [DSG_JOBS](#dsg_jobs-jobs-control-what-operations-to-perform "Jobs control what operations to perform")
*   [DSG_JOB_FORMAT](#dsg_job_format-specify-format-of-results "Specify Format of Results")
*   [DSG_JOB_PARSE_SOME](#dsg_job_parse_some-parse-a-set-of-artefacts "Parse a set of Artefacts")
*   [DSG_JSON_CACHE](#dsg_json_cache-json-for-storing-state "JSON for Storing State")
*   [DSG_JSON_CACHE_SORT](#dsg_json_cache_sort-json-cache-sorted "JSON Cache sorted")
*   [DSG_JSON_EXPORT](#dsg_json_export-json-for-exporting-results "JSON for Exporting Results")
*   [DSG_JSON_IMPORT](#dsg_json_import-json-for-importing-requirements "JSON for Importing Requirements")
*   [DSG_REQ_FIELDS](#dsg_req_fields-requirement-fields "Requirement Fields")
*   [DSG_TRACE_COVERS_EXIST](#dsg_trace_covers_exist-cover-links-must-exist "Cover Links must exist")
*   [DSG_TRACE_DELEGATION](#dsg_trace_delegation-trace-requirements-inside-same-artefact "Trace Requirements inside same Artefact")
*   [DSG_TRACE_DEPENDS_EXIST](#dsg_trace_depends_exist-depend-links-must-exist "Depend Links must exist")
*   [DSG_TRACE_DERIVED](#dsg_trace_derived-mark-requirements-that-do-not-cover-anything "Mark requirements that do not cover anything")
*   [DSG_TRACE_DOWNWARDS](#dsg_trace_downwards-trace-downwards-using-depends-attribute "Trace downwards using Depends attribute")
*   [DSG_TRACE_TRACE_TITLE](#dsg_trace_trace_title-when-tracing-upwards-or-downwards-match-title "When tracing upwards or downwards match title")
*   [DSG_TRACE_UNCOVERED](#dsg_trace_uncovered-mark-requirements-that-are-not-covered "Mark requirements that are not covered")
*   [DSG_TRACE_UPWARDS](#dsg_trace_upwards-trace-upwards-using-covers-attribute "Trace upwards using Covers attribute")
*   [FMT_CODE_MANUAL](#fmt_code_manual-code-depends-on-manual-entry "Code depends on Manual Entry")
*   [FMT_FILE_ENCODINGS](#fmt_file_encodings-handle-file-encodings "Handle File Encodings")
*   [FMT_ID_v2](#fmt_id_v2-requirement-identifier "Requirement Identifier")
*   [FMT_JSON_REQUIREMENT](#fmt_json_requirement-json-requirements-format "JSON Requirements Format")
*   [FMT_MARKDOWN_REQUIREMENT](#fmt_markdown_requirement-markdown-file-format "Markdown File Format")
*   [FMT_MD_ATTRIBUTES](#fmt_md_attributes-attributes "Attributes")
*   [FMT_MD_DESC](#fmt_md_desc-description "Description")
*   [FMT_MD_DESC_HEADINGS](#fmt_md_desc_headings-heading-level-in-description-is-adjusted "Heading Level in Description is adjusted")
*   [FMT_MD_OPT_PREFIX](#fmt_md_opt_prefix-list-of-prefixes "List of Prefixes")
*   [FMT_MD_START](#fmt_md_start-requirement-start "Requirement Start")
*   [FMT_MONO](#fmt_mono-mono-requirement-file "Mono Requirement File")
*   [FMT_RUST_COV](#fmt_rust_cov-rust-coverage-marks "Rust Coverage Marks")
*   [FMT_UNICODE_NORMALIZE](#fmt_unicode_normalize-normalize-unicode-during-read "Normalize Unicode during read")
*   [REQ_ARTEFACT_PARSE_ID](#req_artefact_parse_id-parse-artefact-identifier "Parse Artefact Identifier")
*   [REQ_ARTEFACT_QUERY_ID](#req_artefact_query_id-query-tool-for-artefact-identifier "Query Tool for Artefact Identifier")
*   [REQ_CONFIGURABLE_OUTPUT](#req_configurable_output-the-output-format-is-configurable "The Output Format is Configurable")
*   [REQ_ERROR](#req_error-useful-parser-errors "Useful Parser Errors")
*   [REQ_HUMAN_READABLE](#req_human_readable-human-readable-output "Human Readable Output")
*   [REQ_IDENTIFIEABLE](#req_identifieable-show-versions-of-input-artefacts-in-output "Show versions of input artefacts in output")
*   [REQ_INSTALL](#req_install-easy-to-install "Easy to install")
*   [REQ_QUERIES](#req_queries-configurable-information-granularity "Configurable Information Granularity")
*   [REQ_UNIQUE_ID_v2](#req_unique_id_v2-requirements-have-a-unique-identifier "Requirements have a unique Identifier")
*   [REQ_USER_FRIENDLY](#req_user_friendly-simple-to-use-interface "Simple to use Interface")
*   [REQ_VAL_COVERAGE](#req_val_coverage-validate-coverage "Validate Coverage")
*   [REQ_VAL_GRAPH](#req_val_graph-validate-graph "Validate Graph")
*   [UC_ANALYZE_IMPACT](#uc_analyze_impact-analyze-dependencies-of-requirement "Analyze Dependencies of Requirement")
*   [UC_ANALYZE_SINGLE](#uc_analyze_single-analyze-a-requirement "Analyze a Requirement")
*   [UC_CACHE_STATUS](#uc_cache_status-query-parsing-state "Query Parsing State")
*   [UC_VALIDATE](#uc_validate-validate-configuration "Validate Configuration")


# Derived Requirements

*   [DSG_ART_CONFIG](#dsg_art_config-artefact-configuration-fields "Artefact Configuration Fields")
*   [DSG_ART_EXTERNAL_PARSER](#dsg_art_external_parser-external-artefact-parser "External Artefact Parser")
*   [DSG_ART_PARSE](#dsg_art_parse-artefact-parsing "Artefact Parsing")
*   [DSG_EXPORT_DATA](#dsg_export_data-export-data-to-stdout "Export data to stdout")
*   [DSG_EXPORT_ERRORS](#dsg_export_errors-export-errors-to-stderr "Export errors to stderr")
*   [DSG_EXPORT_FORMAT](#dsg_export_format-allow-selecting-the-export-format "Allow Selecting the Export Format")
*   [DSG_EXPORT_FORMAT_JSON](#dsg_export_format_json-export-to-json "Export to JSON")
*   [DSG_EXPORT_FORMAT_MARKDOWN](#dsg_export_format_markdown-export-to-markdown "Export to Markdown")
*   [DSG_EXPORT_FORMAT_TEX](#dsg_export_format_tex-export-to-tex "Export to TEX")
*   [DSG_FINGERPRINT_TRACING](#dsg_fingerprint_tracing-fingerprint-of-tracing "Fingerprint of Tracing")
*   [DSG_JOBS](#dsg_jobs-jobs-control-what-operations-to-perform "Jobs control what operations to perform")
*   [DSG_JSON_CACHE_SORT](#dsg_json_cache_sort-json-cache-sorted "JSON Cache sorted")
*   [FMT_CODE_MANUAL](#fmt_code_manual-code-depends-on-manual-entry "Code depends on Manual Entry")
*   [FMT_MD_ATTRIBUTES](#fmt_md_attributes-attributes "Attributes")
*   [FMT_MD_DESC](#fmt_md_desc-description "Description")
*   [FMT_MD_DESC_HEADINGS](#fmt_md_desc_headings-heading-level-in-description-is-adjusted "Heading Level in Description is adjusted")
*   [FMT_MD_OPT_PREFIX](#fmt_md_opt_prefix-list-of-prefixes "List of Prefixes")
*   [FMT_MD_START](#fmt_md_start-requirement-start "Requirement Start")
*   [REQ_ARTEFACT_PARSE_ID](#req_artefact_parse_id-parse-artefact-identifier "Parse Artefact Identifier")
*   [REQ_ARTEFACT_QUERY_ID](#req_artefact_query_id-query-tool-for-artefact-identifier "Query Tool for Artefact Identifier")
*   [REQ_CONFIGURABLE_OUTPUT](#req_configurable_output-the-output-format-is-configurable "The Output Format is Configurable")
*   [REQ_DELEGATION](#req_delegation-coverage-delegation "Coverage Delegation")
*   [REQ_IDENTIFIEABLE](#req_identifieable-show-versions-of-input-artefacts-in-output "Show versions of input artefacts in output")
*   [REQ_INSTALL](#req_install-easy-to-install "Easy to install")
*   [REQ_MATCH_ID](#req_match_id-match-by-id "Match by ID")
*   [REQ_NO_OVERCACHING](#req_no_overcaching-no-over-caching "No over-caching")
*   [REQ_QUERIES](#req_queries-configurable-information-granularity "Configurable Information Granularity")
*   [REQ_TRACE](#req_trace-determine-whcih-requirements-cover-which "Determine whcih requirements cover which")
*   [REQ_UNICODE_SAFE](#req_unicode_safe-sane-handling-of-unicode "Sane Handling of unicode")
*   [REQ_UNIQUE_ID_v2](#req_unique_id_v2-requirements-have-a-unique-identifier "Requirements have a unique Identifier")
*   [REQ_USER_FRIENDLY](#req_user_friendly-simple-to-use-interface "Simple to use Interface")
*   [REQ_VAL_COVERAGE](#req_val_coverage-validate-coverage "Validate Coverage")
*   [REQ_VAL_GRAPH](#req_val_graph-validate-graph "Validate Graph")
*   [UC_ANALYZE_IMPACT](#uc_analyze_impact-analyze-dependencies-of-requirement "Analyze Dependencies of Requirement")
*   [UC_ANALYZE_SINGLE](#uc_analyze_single-analyze-a-requirement "Analyze a Requirement")
*   [UC_CACHE_STATUS](#uc_cache_status-query-parsing-state "Query Parsing State")
*   [UC_CHECK](#uc_check-check-for-correct-tracing "Check for correct Tracing")
*   [UC_PARSE](#uc_parse-parse-artefacts "Parse Artefacts")
*   [UC_TRACE](#uc_trace-compute-tracing "Compute Tracing")
*   [UC_VALIDATE](#uc_validate-validate-configuration "Validate Configuration")


# Requirements


## Controller::run 

Origin: `src/controller.rs:162:16`

Covers:
*   design
    *   [DSG_JOB_PARSE](#dsg_job_parse-parse-all-artefacts "Parse all Artefacts")
    *   [DSG_JOB_TRACE](#dsg_job_trace-trace-requirements "Trace Requirements")
*   Does not cover: formats

## Controller::run_jobs 

Origin: `src/controller.rs:125:20`

Covers:
*   design
    *   [DSG_JOB_RETURN_CODE](#dsg_job_return_code-return-code-indicates-if-tracing-is-correct "Return Code Indicates if Tracing is Correct")
*   Does not cover: formats

## try_main 

Origin: `src/main.rs:74:4`

Covers:
*   design
    *   [DSG_CONFIG_TOML](#dsg_config_toml-use-a-single-toml-file-as-configuration "Use a Single TOML File as Configuration")
*   Does not cover: formats

## DSG_ART_CACHING :Cache Parsing Results

Origin: `doc/requirements/DESIGN.md:46`

Covers:
*   requirements
    *   [REQ_FAST](#req_fast-fast "Fast")

Covered By:
*   Not Covered by: formats, code

Description:
Artefact manages a cache of already parsed requirements and only

## DSG_ART_CONFIG :Artefact Configuration Fields

Origin: `doc/requirements/DESIGN.md:67`

Covers:
*   Does not cover: requirements

Covered By:
*   Not Covered by: formats, code

Description:
*   ID
*   paths:  List of Paths or pattern with which to find the files
*   parser:   id of a parsing strategy, e.g. `Markdown Requirements`, `Rust
    Coverage Marks`, `External`
*   parser arguments: Object that is passed to the parser
*   caching: boolean, whether to cache or parse on every access

## DSG_ART_EXTERNAL_PARSER :External Artefact Parser

Origin: `doc/requirements/DESIGN.md:53`

Covers:
*   Does not cover: requirements

Covered By:
*   Not Covered by: formats, code

Description:
If files can not be parsed by this tool, an external program is invoked which
writes  the requirements into a temporary file or to its `stdout` stream in the
JSON format, or as Text which is then processed by the regex parsers.

References:
*   FMT_JSON
*   FMT_JSON_REQUIREMENT

## DSG_ART_PARSE :Artefact Parsing

Origin: `doc/requirements/DESIGN.md:42`

Covers:
*   Does not cover: requirements

Covered By:
*   Not Covered by: formats, code

Description:
Artefact parses the requirements in the files it represents.

## DSG_CLI :Offer a simple Command Line Interface

Origin: `doc/requirements/DESIGN.md:223`

Covers:
*   requirements
    *   [REQ_MACHINE_FRIENDLY](#req_machine_friendly-easy-to-include-in-automated-work-flows "Easy to include in automated work flows")

Covered By:
*   Not Covered by: formats, code

Description:
The tool should be invoked via a simple CLI

## DSG_CONFIG_TOML :Use a Single TOML File as Configuration

Origin: `doc/requirements/DESIGN.md:207`

Covers:
*   requirements
    *   [REQ_CONFIG](#req_config-simple-configuration-in-one-file "Simple Configuration in One File")

Covered By:
*   formats, code
    *   [try_main](#try_main)

Description:
The configuration should be placed in one file `requirements.toml`.
The format is TOML.
The structure of the Configuration is detailed in the Manual

Requires:
*   MAN_CONFIG_STRUCTURE: Configuration Structure and Fields

## DSG_EXPORT_DATA :Export data to stdout

Origin: `doc/requirements/DESIGN.md:170`

Covers:
*   Does not cover: requirements

Covered By:
*   Not Covered by: formats, code

Description:
Print results to stdout in a chosen format

## DSG_EXPORT_ERRORS :Export errors to stderr

Origin: `doc/requirements/DESIGN.md:174`

Covers:
*   Does not cover: requirements

Covered By:
*   Not Covered by: formats, code

Description:
Print Errors to stdout in a chosen format

## DSG_EXPORT_FORMAT :Allow Selecting the Export Format

Origin: `doc/requirements/DESIGN.md:178`

Covers:
*   Does not cover: requirements

Covered By:
*   Not Covered by: formats, code

Description:
The format in which errors and results are written to the out streams can be
chosen.

## DSG_EXPORT_FORMAT_JSON :Export to JSON

Origin: `doc/requirements/DESIGN.md:185`

Covers:
*   Does not cover: requirements

Covered By:
*   Not Covered by: formats, code

Description:
Errors, Requirements, Status, Tracing Info can be exported as JSON

## DSG_EXPORT_FORMAT_MARKDOWN :Export to Markdown

Origin: `doc/requirements/DESIGN.md:189`

Covers:
*   Does not cover: requirements

Covered By:
*   Not Covered by: formats, code

Description:
Errors, Requirements, Status, Tracing Info can be exported as a useful
standalone Markdown File

## DSG_EXPORT_FORMAT_TEX :Export to TEX

Origin: `doc/requirements/DESIGN.md:194`

Covers:
*   Does not cover: requirements

Covered By:
*   Not Covered by: formats, code

Description:
Errors, Requirements, Status, Tracing Info can be exported as tex macro calls,
do that the output can be used via `\include{}` in a tex project that defines
the relevant macros.

Tag:
*   TODO

Todo:
*   Define the exact format of each object

## DSG_FINGERPRINT_ART :Fingerprint of Artefacts

Origin: `doc/requirements/DESIGN.md:135`

Covers:
*   requirements
    *   [REQ_NO_OVERCACHING](#req_no_overcaching-no-over-caching "No over-caching")

Covered By:
*   Not Covered by: formats, code

Comment:
Since the version is included in the fingerprint, the details can be changed
easily.

Description:
The Cache of an artefact is checked for up-to-dateness with a fingerprint.

*   version of the tool
*   parsing-type of the artefact
*   for all files that make up an Artefact:
    *   The sha256 of the file if it is small
    *   The modification time and size if it is large

## DSG_FINGERPRINT_TRACING :Fingerprint of Tracing

Origin: `doc/requirements/DESIGN.md:154`

Covers:
*   Does not cover: requirements

Covered By:
*   Not Covered by: formats, code

Description:
The Cache of an artefact is checked for up-to-dateness with a fingerprint.

*   version of the tool
*   The tracing graph
*   The fingerprint of the artefacts

## DSG_JOBS :Jobs control what operations to perform

Origin: `doc/requirements/DESIGN.md:231`

Covers:
*   Does not cover: requirements

Covered By:
*   Not Covered by: formats, code

Description:
One or more Jobs can be configured. Each Job specifies an operation to perform, the format that results
should be presented in and the file to store results in.

## DSG_JOB_FORMAT :Specify Format of Results

Origin: `doc/requirements/DESIGN.md:254`

Covers:
*   requirements
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")

Covered By:
*   Not Covered by: formats, code

Description:
Specify the Format that results are presented in

## DSG_JOB_PARSE :Parse all Artefacts

Origin: `doc/requirements/DESIGN.md:236`

Covers:
*   requirements
    *   [UC_PARSE](#uc_parse-parse-artefacts "Parse Artefacts")

Covered By:
*   formats, code
    *   [Controller::run](#controller--run)

Description:
Parse the Artefacts

## DSG_JOB_PARSE_SOME :Parse a set of Artefacts

Origin: `doc/requirements/DESIGN.md:242`

Covers:
*   requirements
    *   [UC_PARSE](#uc_parse-parse-artefacts "Parse Artefacts")

Covered By:
*   Not Covered by: formats, code

Description:
Parse one or more Artefacts to make finding errors easier.

## DSG_JOB_RETURN_CODE :Return Code Indicates if Tracing is Correct

Origin: `doc/requirements/DESIGN.md:261`

Covers:
*   requirements
    *   [UC_CHECK](#uc_check-check-for-correct-tracing "Check for correct Tracing")

Covered By:
*   formats, code
    *   [Controller::run_jobs](#controller--run_jobs)

Description:
If configured for a Job, the return code will indicate if a Job found Errors or not

## DSG_JOB_TRACE :Trace Requirements

Origin: `doc/requirements/DESIGN.md:248`

Covers:
*   requirements
    *   [UC_TRACE](#uc_trace-compute-tracing "Compute Tracing")

Covered By:
*   formats, code
    *   [Controller::run](#controller--run)

Description:
Trace Requirements

## DSG_JSON_CACHE :JSON for Storing State

Origin: `doc/requirements/DESIGN.md:107`

Covers:
*   requirements
    *   [REQ_VCS](#req_vcs-allow-version-control "Allow Version Control")
    *   [REQ_FAST](#req_fast-fast "Fast")

Covered By:
*   Not Covered by: formats, code

Description:
JSON is used to store parsing results and computed coverage

## DSG_JSON_CACHE_SORT :JSON Cache sorted

Origin: `doc/requirements/DESIGN.md:115`

Covers:
*   Does not cover: requirements

Covered By:
*   Not Covered by: formats, code

Description:
Sort lists, smaller diff

## DSG_JSON_EXPORT :JSON for Exporting Results

Origin: `doc/requirements/DESIGN.md:127`

Covers:
*   requirements
    *   [REQ_VCS](#req_vcs-allow-version-control "Allow Version Control")
    *   [REQ_MACHINE_READABLE](#req_machine_readable-machine-readable-output "Machine Readable Output")

Covered By:
*   Not Covered by: formats, code

Description:
Tracing results can be exported as json

## DSG_JSON_IMPORT :JSON for Importing Requirements

Origin: `doc/requirements/DESIGN.md:119`

Covers:
*   requirements
    *   [REQ_VCS](#req_vcs-allow-version-control "Allow Version Control")
    *   [REQ_EXTENSIBLE](#req_extensible-extensible-parsing "Extensible Parsing")

Covered By:
*   Not Covered by: formats, code

Description:
Artefacts which can not be parsed by the tool are generated by other tools and imported via JSON

## DSG_REQ_FIELDS :Requirement Fields

Origin: `doc/requirements/DESIGN.md:8`

Covers:
*   requirements
    *   [REQ_UP](#req_up-upward-coverage "Upward Coverage")
    *   [REQ_DOWN](#req_down-downward-coverage "Downward Coverage")
    *   [REQ_DELEGATION](#req_delegation-coverage-delegation "Coverage Delegation")

Covered By:
*   Not Covered by: formats, code

Description:
Attributes of a requirement that this tool requires:
*   ID: a short string that uniquely identifies this requirement

*   Title:  Text that briefly summarizes this requirement (on line)
*   Description: Text that gives detailed description
*   Coverage: List of requirement IDs that are covered by this one
*   Dependencies: List of requirement IDs which cover this one
*   Tags:   List of Strings that can be used to categorize requirements

*   Location:   Artefact that defines this requirement and the location inside
    the artefact where it is defined


*   Comment: Text with even more details, further reading, etc. that has a lower
    priority than Description which may be excluded from reports

## DSG_TRACE_COVERS_EXIST :Cover Links must exist

Origin: `doc/requirements/DESIGN.md:337`

Covers:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-whcih-requirements-cover-which "Determine whcih requirements cover which")
    *   [REQ_UP](#req_up-upward-coverage "Upward Coverage")

Covered By:
*   Not Covered by: formats, code

Description:
For every Link L in  D.Depends of a Requirement D,
there must exist a Requirement U with with U.Id == L.id and 
D.Artefact traces against U.Artefact

## DSG_TRACE_DELEGATION :Trace Requirements inside same Artefact

Origin: `doc/requirements/DESIGN.md:286`

Covers:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-whcih-requirements-cover-which "Determine whcih requirements cover which")
    *   [REQ_DELEGATION](#req_delegation-coverage-delegation "Coverage Delegation")
    *   [REQ_MATCH_ID](#req_match_id-match-by-id "Match by ID")

Covered By:
*   Not Covered by: formats, code

Description:
A Requirement can delegate to a requirement in the same artefact, so that itself
does not need to be covered.

D.Artefact directly traces against U.Artefact.

## DSG_TRACE_DEPENDS_EXIST :Depend Links must exist

Origin: `doc/requirements/DESIGN.md:330`

Covers:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-whcih-requirements-cover-which "Determine whcih requirements cover which")
    *   [REQ_DOWN](#req_down-downward-coverage "Downward Coverage")

Covered By:
*   Not Covered by: formats, code

Description:
For every Link L in  U.Depends of a Requirement U,
there must exist a Requirement D with with D.Id == L.id and 
D.Artefact traces against U.Artefact

## DSG_TRACE_DERIVED :Mark requirements that do not cover anything

Origin: `doc/requirements/DESIGN.md:296`

Covers:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-whcih-requirements-cover-which "Determine whcih requirements cover which")

Covered By:
*   Not Covered by: formats, code

Description:
Requirement R is derived if there is no Requirement U so that R covers U.

## DSG_TRACE_DOWNWARDS :Trace downwards using Depends attribute

Origin: `doc/requirements/DESIGN.md:279`

Covers:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-whcih-requirements-cover-which "Determine whcih requirements cover which")
    *   [REQ_DOWN](#req_down-downward-coverage "Downward Coverage")
    *   [REQ_MATCH_ID](#req_match_id-match-by-id "Match by ID")

Covered By:
*   Not Covered by: formats, code

Description:
Requirement U covers Requirement D if D.id appears in U.Depends and
D.Artefact directly traces against U.Artefact

## DSG_TRACE_TRACE_TITLE :When tracing upwards or downwards match title

Origin: `doc/requirements/DESIGN.md:308`

Covers:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-whcih-requirements-cover-which "Determine whcih requirements cover which")
    *   [REQ_VAL_TITLE](#req_val_title-check-matching-title "Check matching title")

Covered By:
*   Not Covered by: formats, code

Description:
When tracing Upwards or Downwards, emit an error if the title of the coverage does
not match the title of the covered requirement

Example:
### REQ_U: Title of Upper

    An Upper Requirement

    ### REQ_D: Title of Lower

    A Lower Requirement that covers REQ_U with an exactly mathcing title.

    Covers:
    *   REQ_U: Title of Upper

## DSG_TRACE_UNCOVERED :Mark requirements that are not covered

Origin: `doc/requirements/DESIGN.md:302`

Covers:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-whcih-requirements-cover-which "Determine whcih requirements cover which")

Covered By:
*   Not Covered by: formats, code

Description:
Requirement R is uncovered if there is no Requirement D so that D covers R.

## DSG_TRACE_UPWARDS :Trace upwards using Covers attribute

Origin: `doc/requirements/DESIGN.md:272`

Covers:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-whcih-requirements-cover-which "Determine whcih requirements cover which")
    *   [REQ_UP](#req_up-upward-coverage "Upward Coverage")
    *   [REQ_MATCH_ID](#req_match_id-match-by-id "Match by ID")

Covered By:
*   Not Covered by: formats, code

Description:
Requirement U covers Requirement D if U.id appears in D.Covers and
D.Artefact directly traces against U.Artefact

## FMT_CODE_MANUAL :Code depends on Manual Entry

Origin: `doc/requirements/FORMATS.md:235`

Covers:
*   Does not cover: requirements
*   Does not cover: design

Covered By:
*   Not Covered by: code

Description:
Code with a comment like `=> MAN_ID(:TITLE)` makes a requirement of the code line that depends on a manual entry `MAN_ID`.

## FMT_FILE_ENCODINGS :Handle File Encodings

Origin: `doc/requirements/FORMATS.md:48`

Covers:
*   requirements
    *   [REQ_UNICODE_SAFE](#req_unicode_safe-sane-handling-of-unicode "Sane Handling of unicode")
*   Does not cover: design

Covered By:
*   Not Covered by: code

Description:
When not otherwise specified, Text Files are read as UTF-8 and encoding errors are
replaced.

## FMT_ID_v2 :Requirement Identifier

Origin: `doc/requirements/FORMATS.md:29`

Covers:
*   requirements
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
*   Does not cover: design

Covered By:
*   Not Covered by: code

Comment:
Discussion in the [README](README.md#requirement-ids)

Description:
Requirement identifier consist of letters, digits and underscore, specifically
they match the Regular Expression

History:
*   v2: use to Unicode Identifiers

## FMT_JSON_REQUIREMENT :JSON Requirements Format

Origin: `doc/requirements/FORMATS.md:222`

Covers:
*   requirements
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
*   Does not cover: design

Covered By:
*   Not Covered by: code

## FMT_MARKDOWN_REQUIREMENT :Markdown File Format

Origin: `doc/requirements/FORMATS.md:81`

Covers:
*   requirements
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
*   Does not cover: design

Covered By:
*   Not Covered by: code

Description:
The artefact is a Markdown file with freely chosen layout.  A Requirement is in
a heading line with requirement ID and title, followed by description and other
attributes.

## FMT_MD_ATTRIBUTES :Attributes

Origin: `doc/requirements/FORMATS.md:121`

Covers:
*   Does not cover: requirements
*   Does not cover: design

Covered By:
*   Not Covered by: code

Comment:
`Covers: Some, IDs,`

or

    `Covers:
    *   Some
    *   IDs

Description:
Attributes are parsed from paragraphs that start with a single word followed by
a colon. Once the first Attribute Paragraph is encountered, the Parser will add
to the Attribute Value until:

*   The start of another Attribute Paragraph
*   A Heading the same level or less. This ends the Requirement.

*   Text: can span multiple Paragraphs
*   Short List: Comma separated list of IDs
*   Long list:  one paragraph that only consists of bullet points

the same format as requirement IDs.

long lists, where each item starts with a requirement id, optionally followed by
a colon and the title of the referenced requirement.

## FMT_MD_DESC :Description

Origin: `doc/requirements/FORMATS.md:103`

Covers:
*   Does not cover: requirements
*   Does not cover: design

Covered By:
*   Not Covered by: code

Description:
The paragraphs following the start of the requirement make up the description of
the requirement.

*   The Start of another Requirement.
*   The start of an Attribute Paragraph
*   A Heading the same level or less. This ends the Requirement.

## FMT_MD_DESC_HEADINGS :Heading Level in Description is adjusted

Origin: `doc/requirements/FORMATS.md:115`

Covers:
*   Does not cover: requirements
*   Does not cover: design

Covered By:
*   Not Covered by: code

Description:
Headings with a lower level than the starting one, that do not start a nested
requirement are added to the description. Their heading level is adjusted by
removing as many leading `#` as the requirement had

## FMT_MD_OPT_PREFIX :List of Prefixes

Origin: `doc/requirements/FORMATS.md:180`

Covers:
*   Does not cover: requirements
*   Does not cover: design

Covered By:
*   Not Covered by: code

Description:
A List of strings can be passed, which is used to prevent the parser from
creating unintended requirements from headlines which accidentally have the
right form.

normal headings, if the identifier of the would be requirement does not start
with one of the list of prefixes. If the list is empty, no prefix matching is
performed and all matching lines lead to a requirement.

## FMT_MD_START :Requirement Start

Origin: `doc/requirements/FORMATS.md:98`

Covers:
*   Does not cover: requirements
*   Does not cover: design

Covered By:
*   Not Covered by: code

Description:
A Requirement starts with a `#` heading of any level that has the form `ID:
TITLE`.

## FMT_MONO :Mono Requirement File

Origin: `doc/requirements/FORMATS.md:197`

Covers:
*   requirements
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
*   Does not cover: design

Covered By:
*   Not Covered by: code

Comment:
See this projects README for examples.

Description:
Artefact of type MonoRequirement emit exactly one Requirement with the following
attributes:
*   Id: The stem of the file path (i.e. `README.md`)
*   Title:  The first line containing Word-Characters with all non-word
    characters trimmed of both ends of the line. (Allowing Markdown heading,
    C style comments, ...)
*   Depends: Every Requirement-Id that immediately follows a fat arrow (`=>`).

## FMT_RUST_COV :Rust Coverage Marks

Origin: `doc/requirements/FORMATS.md:228`

Covers:
*   requirements
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
*   Does not cover: design

Covered By:
*   Not Covered by: code

Description:
Parse `cov_mark::hit!(REQ_ID)` and `cov_mark::hit!(REQ_ID) # TITLE`

## FMT_UNICODE_NORMALIZE :Normalize Unicode during read

Origin: `doc/requirements/FORMATS.md:57`

Covers:
*   requirements
    *   [REQ_UNICODE_SAFE](#req_unicode_safe-sane-handling-of-unicode "Sane Handling of unicode")
*   Does not cover: design

Covered By:
*   Not Covered by: code

Comment:
See [Rust RFC 2457](https://rust-lang.github.io/rfcs/2457-non-ascii-idents.html) on the topic.

This means two requirement ids are equal if their NFC forms are equal.

Description:
All input strings are unicode normalizes as
[NFC](https://www.unicode.org/reports/tr15/#Normalization_Forms_Table).
This means that
*   All output derived from input will be NFC normalized
*   Identifier Matching can be done on the byte level

## README :Requirement Tracing

Origin: `README.md`

Covered By:
*   requirements
    *   [REQ_EXTENSIBLE](#req_extensible-extensible-parsing "Extensible Parsing")
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
    *   [REQ_UP](#req_up-upward-coverage "Upward Coverage")
    *   [REQ_DOWN](#req_down-downward-coverage "Downward Coverage")
    *   [REQ_ERROR](#req_error-useful-parser-errors "Useful Parser Errors")
    *   [REQ_VCS](#req_vcs-allow-version-control "Allow Version Control")
    *   [REQ_FAST](#req_fast-fast "Fast")
    *   [REQ_VAL_TITLE](#req_val_title-check-matching-title "Check matching title")
    *   [REQ_CONFIG](#req_config-simple-configuration-in-one-file "Simple Configuration in One File")
    *   [REQ_MACHINE_FRIENDLY](#req_machine_friendly-easy-to-include-in-automated-work-flows "Easy to include in automated work flows")
    *   [REQ_HUMAN_READABLE](#req_human_readable-human-readable-output "Human Readable Output")
    *   [REQ_MACHINE_READABLE](#req_machine_readable-machine-readable-output "Machine Readable Output")

## REQ_ARTEFACT_PARSE_ID :Parse Artefact Identifier

Origin: `doc/requirements/REQUIREMENTS.md:76`

Covers:
*   Does not cover: readme

Covered By:
*   Not Covered by: design, formats

Description:
An identifier can be parsed from the line, for example a version string, or
an expanded RCS Keyword (like `$Id: /path/to/artefact.md$42$`).

## REQ_ARTEFACT_QUERY_ID :Query Tool for Artefact Identifier

Origin: `doc/requirements/REQUIREMENTS.md:81`

Covers:
*   Does not cover: readme

Covered By:
*   Not Covered by: design, formats

Description:
An identifier can be gotten from an external tool like `git describe --tags`.

## REQ_CONFIG :Simple Configuration in One File

Origin: `doc/requirements/REQUIREMENTS.md:158`

Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats
    *   [DSG_CONFIG_TOML](#dsg_config_toml-use-a-single-toml-file-as-configuration "Use a Single TOML File as Configuration")

Description:
All Configuration is stored in a single file using a common Format that is
editable for humans and machine readable.

## REQ_CONFIGURABLE_OUTPUT :The Output Format is Configurable

Origin: `doc/requirements/REQUIREMENTS.md:51`

Covers:
*   Does not cover: readme

Covered By:
*   Not Covered by: design, formats

Description:
The Format in which Information is returned is configurable

## REQ_DELEGATION :Coverage Delegation

Origin: `doc/requirements/REQUIREMENTS.md:96`

Covers:
*   Does not cover: readme

Covered By:
*   design, formats
    *   [DSG_REQ_FIELDS](#dsg_req_fields-requirement-fields "Requirement Fields")
    *   [DSG_TRACE_DELEGATION](#dsg_trace_delegation-trace-requirements-inside-same-artefact "Trace Requirements inside same Artefact")

Description:
A Requirement delegates to another requirement in the same artefact by including
the id of the lower one in its Dependencies attribute.

requirement.

## REQ_DOWN :Downward Coverage

Origin: `doc/requirements/REQUIREMENTS.md:90`

Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats
    *   [DSG_REQ_FIELDS](#dsg_req_fields-requirement-fields "Requirement Fields")
    *   [DSG_TRACE_DOWNWARDS](#dsg_trace_downwards-trace-downwards-using-depends-attribute "Trace downwards using Depends attribute")
    *   [DSG_TRACE_DEPENDS_EXIST](#dsg_trace_depends_exist-depend-links-must-exist "Depend Links must exist")

Description:
A Requirement is covered by a lower one by including the id of the lower one in
its Dependencies attribute.

## REQ_ERROR :Useful Parser Errors

Origin: `doc/requirements/REQUIREMENTS.md:104`

Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   Not Covered by: design, formats

Description:
Parser Errors give the precise location and type of the problem, for example filename with
line number of the artefact.

## REQ_EXTENSIBLE :Extensible Parsing

Origin: `doc/requirements/REQUIREMENTS.md:38`

Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats
    *   [DSG_JSON_IMPORT](#dsg_json_import-json-for-importing-requirements "JSON for Importing Requirements")

Description:
If internal parsers are not able to work on an Artefact, external tools can be
incorporated.

## REQ_FAST :Fast

Origin: `doc/requirements/REQUIREMENTS.md:170`

Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats
    *   [DSG_ART_CACHING](#dsg_art_caching-cache-parsing-results "Cache Parsing Results")
    *   [DSG_JSON_CACHE](#dsg_json_cache-json-for-storing-state "JSON for Storing State")

Description:
Show results quickly, especially if only a small query is given.

## REQ_FORMATS :Well defined Formats

Origin: `doc/requirements/REQUIREMENTS.md:63`

Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats
    *   [DSG_JOB_FORMAT](#dsg_job_format-specify-format-of-results "Specify Format of Results")
    *   [FMT_ID_v2](#fmt_id_v2-requirement-identifier "Requirement Identifier")
    *   [FMT_MARKDOWN_REQUIREMENT](#fmt_markdown_requirement-markdown-file-format "Markdown File Format")
    *   [FMT_MONO](#fmt_mono-mono-requirement-file "Mono Requirement File")
    *   [FMT_JSON_REQUIREMENT](#fmt_json_requirement-json-requirements-format "JSON Requirements Format")
    *   [FMT_RUST_COV](#fmt_rust_cov-rust-coverage-marks "Rust Coverage Marks")

Description:
To work with external programs as parsers or to process the output, the formats used must be well
defined.

## REQ_HUMAN_READABLE :Human Readable Output

Origin: `doc/requirements/REQUIREMENTS.md:59`

Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   Not Covered by: design, formats

Description:
Information can be returned in a format that can easily be read by humans

## REQ_IDENTIFIEABLE :Show versions of input artefacts in output

Origin: `doc/requirements/REQUIREMENTS.md:43`

Covers:
*   Does not cover: readme

Covered By:
*   Not Covered by: design, formats

Description:
When reading the output, each input must be clearly identifiable.
For example by its:
*   git describe
*   hash
*   file modification time

## REQ_INSTALL :Easy to install

Origin: `doc/requirements/REQUIREMENTS.md:33`

Covers:
*   Does not cover: readme

Covered By:
*   Not Covered by: design, formats

Description:
no package management, libraries, dependencies

## REQ_MACHINE_FRIENDLY :Easy to include in automated work flows

Origin: `doc/requirements/REQUIREMENTS.md:23`

Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats
    *   [DSG_CLI](#dsg_cli-offer-a-simple-command-line-interface "Offer a simple Command Line Interface")

Description:
For ease of integration into other tools, all functionality must be available via a CLI.

## REQ_MACHINE_READABLE :Machine Readable Output

Origin: `doc/requirements/REQUIREMENTS.md:55`

Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats
    *   [DSG_JSON_EXPORT](#dsg_json_export-json-for-exporting-results "JSON for Exporting Results")

Description:
Information can be returned in a format that can easily be read by other tools

## REQ_MATCH_ID :Match by ID

Origin: `doc/requirements/REQUIREMENTS.md:110`

Covers:
*   Does not cover: readme

Covered By:
*   design, formats
    *   [DSG_TRACE_UPWARDS](#dsg_trace_upwards-trace-upwards-using-covers-attribute "Trace upwards using Covers attribute")
    *   [DSG_TRACE_DOWNWARDS](#dsg_trace_downwards-trace-downwards-using-depends-attribute "Trace downwards using Depends attribute")
    *   [DSG_TRACE_DELEGATION](#dsg_trace_delegation-trace-requirements-inside-same-artefact "Trace Requirements inside same Artefact")

Description:
A Requirements covers another by its ID.

## REQ_NO_OVERCACHING :No over-caching

Origin: `doc/requirements/REQUIREMENTS.md:174`

Covers:
*   Does not cover: readme

Covered By:
*   design, formats
    *   [DSG_FINGERPRINT_ART](#dsg_fingerprint_art-fingerprint-of-artefacts "Fingerprint of Artefacts")

Description:
If the user has to flush the cache, this is a bug in the Program.

## REQ_QUERIES :Configurable Information Granularity

Origin: `doc/requirements/REQUIREMENTS.md:164`

Covers:
*   Does not cover: readme

Covered By:
*   Not Covered by: design, formats

Description:
Instead of returning every detail about the Tracing, specific parts of
information can be queried, so that the output is as slim as possible and only
relevant information is computed.

## REQ_TRACE :Determine whcih requirements cover which

Origin: `doc/requirements/REQUIREMENTS.md:6`

Covers:
*   Does not cover: readme

Covered By:
*   design, formats
    *   [DSG_TRACE_UPWARDS](#dsg_trace_upwards-trace-upwards-using-covers-attribute "Trace upwards using Covers attribute")
    *   [DSG_TRACE_DOWNWARDS](#dsg_trace_downwards-trace-downwards-using-depends-attribute "Trace downwards using Depends attribute")
    *   [DSG_TRACE_DELEGATION](#dsg_trace_delegation-trace-requirements-inside-same-artefact "Trace Requirements inside same Artefact")
    *   [DSG_TRACE_DERIVED](#dsg_trace_derived-mark-requirements-that-do-not-cover-anything "Mark requirements that do not cover anything")
    *   [DSG_TRACE_UNCOVERED](#dsg_trace_uncovered-mark-requirements-that-are-not-covered "Mark requirements that are not covered")
    *   [DSG_TRACE_TRACE_TITLE](#dsg_trace_trace_title-when-tracing-upwards-or-downwards-match-title "When tracing upwards or downwards match title")
    *   [DSG_TRACE_DEPENDS_EXIST](#dsg_trace_depends_exist-depend-links-must-exist "Depend Links must exist")
    *   [DSG_TRACE_COVERS_EXIST](#dsg_trace_covers_exist-cover-links-must-exist "Cover Links must exist")

Description:
Compute tracing for each Requirement, wether it is covered, uncovered, covers
another requirement or is derived.

## REQ_UNICODE_SAFE :Sane Handling of unicode

Origin: `doc/requirements/REQUIREMENTS.md:68`

Covers:
*   Does not cover: readme

Covered By:
*   design, formats
    *   [FMT_FILE_ENCODINGS](#fmt_file_encodings-handle-file-encodings "Handle File Encodings")
    *   [FMT_UNICODE_NORMALIZE](#fmt_unicode_normalize-normalize-unicode-during-read "Normalize Unicode during read")

Description:
Some Characters can be represented by multiple different sequences of Unicode
Code Points. Also Unicode Encodings like UTF-8 can encode the same Codepoint
as different bytes.

## REQ_UNIQUE_ID_v2 :Requirements have a unique Identifier

Origin: `doc/requirements/REQUIREMENTS.md:11`

Covers:
*   Does not cover: readme

Covered By:
*   Not Covered by: design, formats

Description:
Each requirement must be identifiable by a short, unique string.
All unicode symbols must be possible, though parsers may restrict this

History:
*   v2: Unicode

## REQ_UP :Upward Coverage

Origin: `doc/requirements/REQUIREMENTS.md:85`

Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats
    *   [DSG_REQ_FIELDS](#dsg_req_fields-requirement-fields "Requirement Fields")
    *   [DSG_TRACE_UPWARDS](#dsg_trace_upwards-trace-upwards-using-covers-attribute "Trace upwards using Covers attribute")
    *   [DSG_TRACE_COVERS_EXIST](#dsg_trace_covers_exist-cover-links-must-exist "Cover Links must exist")

Description:
A Requirement covers a higher one by including the id of the higer one in its
Coverage attribute.

## REQ_USER_FRIENDLY :Simple to use Interface

Origin: `doc/requirements/REQUIREMENTS.md:19`

Covers:
*   Does not cover: readme

Covered By:
*   Not Covered by: design, formats

Description:
The User Interface should be slim and straight forward.

## REQ_VAL_COVERAGE :Validate Coverage

Origin: `doc/requirements/REQUIREMENTS.md:145`

Covers:
*   Does not cover: readme

Covered By:
*   Not Covered by: design, formats

Description:
An error is reported for a Coverage claim for which no Requirement exists in the
relevant artefacts.

## REQ_VAL_GRAPH :Validate Graph

Origin: `doc/requirements/REQUIREMENTS.md:150`

Covers:
*   Does not cover: readme

Covered By:
*   Not Covered by: design, formats

Description:
An error is reported for an invalid tracing graph. A Tracing Graph is invalid,
if:
*   there is a loop
*   a Node has no edges leading in or out

## REQ_VAL_TITLE :Check matching title

Origin: `doc/requirements/REQUIREMENTS.md:114`

Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats
    *   [DSG_TRACE_TRACE_TITLE](#dsg_trace_trace_title-when-tracing-upwards-or-downwards-match-title "When tracing upwards or downwards match title")

Comment:
This is only really necessary where the requirement ids are not informative.
For example a Requirement with the id `DSG_123` and the title `Delete Everything`
could be covered by a line of code like:

    create_temporary_file();    COVERS(DSG_123)

Which gives little information to the reader. It is not obvious if the
requirement has nothing to do with the object that covers it.

By also providing the title of the requirement, things can get even worse, if
the title is wrong as the reader now believes to know which requirement is
meant.

    create_temporary_file();    COVERS(DSG_123, "Create Temporary File")

This is is prevented, by checking the tile. The above case would produce
a verification Error. The code would have to be changed like the following for
the tool to accept it without warning:

    create_temporary_file();    COVERS(DSG_123, "Delete Everything")

At this point you have defeated the tool, but now a review can easily discover
the wrong coverage.

Description:
A Coverage link that is established by requirement ID can be verified by
comparing the requirement's title.

## REQ_VCS :Allow Version Control

Origin: `doc/requirements/REQUIREMENTS.md:27`

Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats
    *   [DSG_JSON_CACHE](#dsg_json_cache-json-for-storing-state "JSON for Storing State")
    *   [DSG_JSON_IMPORT](#dsg_json_import-json-for-importing-requirements "JSON for Importing Requirements")
    *   [DSG_JSON_EXPORT](#dsg_json_export-json-for-exporting-results "JSON for Exporting Results")

Description:
All Config, state, intermediate results are in Plain Text based formats that are usable in
Version Control and produce usable diff.

## UC_ANALYZE_IMPACT :Analyze Dependencies of Requirement

Origin: `doc/requirements/REQUIREMENTS.md:216`

Covers:
*   Does not cover: readme

Covered By:
*   Not Covered by: design, formats

Description:
For a Requirement, look up all requirements that it depends upon transitively.

Parameters:
*   Requirement Id

## UC_ANALYZE_SINGLE :Analyze a Requirement

Origin: `doc/requirements/REQUIREMENTS.md:208`

Covers:
*   Does not cover: readme

Covered By:
*   Not Covered by: design, formats

Description:
For a Requirement, look up what it covers, and where it is covered itself and
where coverage is missing.

Parameters:
*   Requirement Id

## UC_CACHE_STATUS :Query Parsing State

Origin: `doc/requirements/REQUIREMENTS.md:184`

Covers:
*   Does not cover: readme

Covered By:
*   Not Covered by: design, formats

Description:
All artefacts are checked for changes since last parsing

## UC_CHECK :Check for correct Tracing

Origin: `doc/requirements/REQUIREMENTS.md:204`

Covers:
*   Does not cover: readme

Covered By:
*   design, formats
    *   [DSG_JOB_RETURN_CODE](#dsg_job_return_code-return-code-indicates-if-tracing-is-correct "Return Code Indicates if Tracing is Correct")

Description:
Like `UC_TRACE` but the only output of interest is whether there were tracing errors or not

## UC_PARSE :Parse Artefacts

Origin: `doc/requirements/REQUIREMENTS.md:188`

Covers:
*   Does not cover: readme

Covered By:
*   design, formats
    *   [DSG_JOB_PARSE](#dsg_job_parse-parse-all-artefacts "Parse all Artefacts")
    *   [DSG_JOB_PARSE_SOME](#dsg_job_parse_some-parse-a-set-of-artefacts "Parse a set of Artefacts")

Description:
A Set of artefacts are parsed, reporting all requirements and errors.

Parameters:
*   Artefacts to Parse

## UC_TRACE :Compute Tracing

Origin: `doc/requirements/REQUIREMENTS.md:196`

Covers:
*   Does not cover: readme

Covered By:
*   design, formats
    *   [DSG_JOB_TRACE](#dsg_job_trace-trace-requirements "Trace Requirements")

Description:
All requirements are matched up and down the Tracing Graph. The results are
stored in a file and bad tracing is reported.

Parameters:
*   Tracing Report

## UC_VALIDATE :Validate Configuration

Origin: `doc/requirements/REQUIREMENTS.md:180`

Covers:
*   Does not cover: readme

Covered By:
*   Not Covered by: design, formats

Description:
The configuration is loaded, the Tracing Graph validated. This Step should be fast.
