

# Uncovered Requirements

*   [ARCH_CLI](#arch_cli-command-line-interface "Command Line Interface")
*   [ARCH_FORMATTER](#arch_formatter-format-output-in-requested-format "Format output in requested Format")
*   [ARCH_PARSER](#arch_parser-parser "Parser")
*   [DSG_ART_CONFIG](#dsg_art_config-artefact-configuration-fields "Artefact Configuration Fields")
*   [DSG_ART_FILES](#dsg_art_files-artefact-loads-one-or-more-files "Artefact loads one or more Files")
*   [DSG_ART_IGNORE_DERIVED](#dsg_art_ignore_derived-ignore-derived-requirements "Ignore Derived Requirements")
*   [DSG_ART_PARSE_COLLECT_ERRORS](#dsg_art_parse_collect_errors-collect-errors-but-keep-parsing "Collect errors but keep parsing")
*   [DSG_CTRL_CONFIG](#dsg_ctrl_config-single-config-file "Single Config File")
*   [DSG_GRAPH](#dsg_graph-artefact-graph "Artefact Graph")
*   [DSG_GRAPH_FORKS](#dsg_graph_forks-trace-edge-groups "Trace Edge Groups")
*   [DSG_GRAPH_VAL_NO_LOOP](#dsg_graph_val_no_loop-validate-that-the-graph-has-no-loops "Validate that the Graph has no Loops")
*   [DSG_JOB_FILE](#dsg_job_file-specify-file-to-store-reports-in "Specify File to Store Reports in")
*   [DSG_JOB_FORMAT](#dsg_job_format-specify-format-of-reports "Specify Format of Reports")
*   [DSG_REQ_FIELDS](#dsg_req_fields-requirement-fields "Requirement Fields")
*   [DSG_TRACE](#dsg_trace-walk-the-graph-and-trace-requirements "Walk the Graph and trace requirements")
*   [DSG_TRACE_CHECK_TITLE](#dsg_trace_check_title-when-tracing-upwards-or-downwards-match-title "When tracing upwards or downwards match title")
*   [DSG_TRACE_COVERS_EXIST](#dsg_trace_covers_exist-cover-links-must-exist "Cover Links must exist")
*   [DSG_TRACE_DEPENDS_EXIST](#dsg_trace_depends_exist-depend-links-must-exist "Depend Links must exist")
*   [DSG_TRACE_DERIVED](#dsg_trace_derived-record-requirements-that-do-not-cover-anything "Record requirements that do not cover anything")
*   [DSG_TRACE_DOWNWARDS](#dsg_trace_downwards-trace-downwards-using-depends-attribute "Trace downwards using Depends attribute")
*   [DSG_TRACE_UNCOVERED](#dsg_trace_uncovered-record-requirements-that-are-not-completely-covered "Record requirements that are not completely covered")
*   [DSG_TRACE_UPWARDS](#dsg_trace_upwards-trace-upwards-using-covers-attribute "Trace upwards using Covers attribute")
*   [DSG_TRACE_VALIDATE_EDGE](#dsg_trace_validate_edge-validate-edge-is-used-at-least-once "Validate Edge is used at least once")
*   [FMT_CTAGS](#fmt_ctags-export-requirements-as-ctags "Export Requirements as CTags")
*   [FMT_EXP_MD_ERROR](#fmt_exp_md_error-markdown-export-format-for-errors "Markdown Export Format for Errors")
*   [FMT_EXP_MD_REQ](#fmt_exp_md_req-markdown-export-format-for-requirements "Markdown Export Format for Requirements")
*   [FMT_FILE_ENCODINGS](#fmt_file_encodings-handle-file-encodings "Handle File Encodings")
*   [FMT_ID_v2](#fmt_id_v2-requirement-identifier "Requirement Identifier")
*   [FMT_JSON](#fmt_json-json-as-data-format "Json as Data Format")
*   [FMT_JSON_REQUIREMENT](#fmt_json_requirement-json-requirements-format "JSON Requirements Format")
*   [FMT_MARKDOWN_REQUIREMENT](#fmt_markdown_requirement-markdown-file-format "Markdown File Format")
*   [FMT_MD_ATTRIBUTES](#fmt_md_attributes-attributes "Attributes")
*   [FMT_MD_DESC](#fmt_md_desc-description "Description")
*   [FMT_MD_DESC_HEADINGS](#fmt_md_desc_headings-heading-level-in-description-is-adjusted "Heading Level in Description is adjusted")
*   [FMT_MD_OPT_PREFIX](#fmt_md_opt_prefix-list-of-prefixes "List of Prefixes")
*   [FMT_MD_START](#fmt_md_start-requirement-start "Requirement Start")
*   [FMT_UNICODE_NORMALIZE](#fmt_unicode_normalize-normalize-unicode-during-read "Normalize Unicode during read")
*   [REQ_IDENTIFIEABLE](#req_identifieable-show-versions-of-input-artefacts-in-output "Show versions of input artefacts in output")


# Derived Requirements

*   [ARCH_ARTEFACT](#arch_artefact-artefact "Artefact")
*   [ARCH_CONTROLLER](#arch_controller-controller "Controller")
*   [ARCH_GRAPH](#arch_graph-graph "Graph")
*   [DSG_ART_IGNORE_DERIVED](#dsg_art_ignore_derived-ignore-derived-requirements "Ignore Derived Requirements")
*   [DSG_JOBS](#dsg_jobs-jobs-control-what-operations-to-perform "Jobs control what operations to perform")
*   [DSG_JOB_FILE](#dsg_job_file-specify-file-to-store-reports-in "Specify File to Store Reports in")
*   [DSG_JOB_RETURN_CODE](#dsg_job_return_code-return-code-indicates-if-tracing-is-correct "Return Code Indicates if Tracing is Correct")
*   [FMT_CTAGS](#fmt_ctags-export-requirements-as-ctags "Export Requirements as CTags")
*   [FMT_MD_ATTRIBUTES](#fmt_md_attributes-attributes "Attributes")
*   [FMT_MD_DESC](#fmt_md_desc-description "Description")
*   [FMT_MD_DESC_HEADINGS](#fmt_md_desc_headings-heading-level-in-description-is-adjusted "Heading Level in Description is adjusted")
*   [FMT_MD_OPT_PREFIX](#fmt_md_opt_prefix-list-of-prefixes "List of Prefixes")
*   [FMT_MD_START](#fmt_md_start-requirement-start "Requirement Start")


# Requirements


## ARCH_ARTEFACT: Artefact

Origin: [doc/requirements/ARCHITECTURE.md:41](../doc/requirements/ARCHITECTURE.md?plain=1#L41)


An artefact represents Requirements from a file, or several related files.
Artefacts parse files with Parsers and store requirements



Covers:
*   Does not cover: requirements

Covered By:
*   design
    *   [DSG_ART_FILES](#dsg_art_files-artefact-loads-one-or-more-files "Artefact loads one or more Files")
    *   [DSG_ART_PARSE](#dsg_art_parse-artefact-parses-input-lazily "Artefact parses input lazily")
    *   [DSG_ART_CONFIG](#dsg_art_config-artefact-configuration-fields "Artefact Configuration Fields")

## ARCH_CLI: Command Line Interface

Origin: [doc/requirements/ARCHITECTURE.md:22](../doc/requirements/ARCHITECTURE.md?plain=1#L22)


All functionality of the tool is exposed in a simple command line interface.
The CLI is shipped as a statically linked binary.
Other Tools can interface with the CLI to provide caching, lazy parsing of complicated input formats
and generation of complicated output formats



Covers:
*   requirements
    *   [REQ_MACHINE_FRIENDLY](#req_machine_friendly-easy-to-include-in-automated-work-flows "Easy to include in automated work flows")
    *   [REQ_INSTALL](#req_install-easy-to-install "Easy to install")
    *   [REQ_FAST](#req_fast-fast "Fast")

Covered By:
*   Not Covered by: design

## ARCH_CONTROLLER: Controller

Origin: [doc/requirements/ARCHITECTURE.md:34](../doc/requirements/ARCHITECTURE.md?plain=1#L34)


The Controller Reads the Configuration file, and builds the tracing graph as
needed to answer the query. It then obtains the queried information, stores it
to the requested location in the requested format




Covers:
*   Does not cover: requirements

Covered By:
*   design
    *   [DSG_CTRL_CONFIG](#dsg_ctrl_config-single-config-file "Single Config File")

## ARCH_FORMATTER: Format output in requested Format

Origin: [doc/requirements/ARCHITECTURE.md:73](../doc/requirements/ARCHITECTURE.md?plain=1#L73)


The formatter stores all available kinds of information in different
selectable formats



Covers:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")

Covered By:
*   Not Covered by: design

## ARCH_GRAPH: Graph

Origin: [doc/requirements/ARCHITECTURE.md:53](../doc/requirements/ARCHITECTURE.md?plain=1#L53)


The Graph organizes Artefacts into a directed graph without loops.
Each Node in the Graph represents a single Artefact. An Edge from Artefact `A`
to Artefact `B` expresses, that one or more Requirements in `B` cover one or
more requirements in `A`.

least once for each group of edges that lead out of it.
Covers:
*   REQ_TRACE



Covers:
*   Does not cover: requirements

Covered By:
*   design
    *   [DSG_GRAPH](#dsg_graph-artefact-graph "Artefact Graph")
    *   [DSG_GRAPH_FORKS](#dsg_graph_forks-trace-edge-groups "Trace Edge Groups")
    *   [DSG_GRAPH_VAL_NO_LOOP](#dsg_graph_val_no_loop-validate-that-the-graph-has-no-loops "Validate that the Graph has no Loops")

## ARCH_PARSER: Parser

Origin: [doc/requirements/ARCHITECTURE.md:46](../doc/requirements/ARCHITECTURE.md?plain=1#L46)


A Parser processes an input file and emits Requirements



Covers:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")

Covered By:
*   Not Covered by: design

## ARCH_TRACE: Tracer

Origin: [doc/requirements/ARCHITECTURE.md:65](../doc/requirements/ARCHITECTURE.md?plain=1#L65)


The tracer walks the graph and calculates tracing information



Covers:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
    *   [UC_TMX](#uc_tmx-create-traceability-matrix "Create Traceability Matrix")

Covered By:
*   design
    *   [DSG_TRACE](#dsg_trace-walk-the-graph-and-trace-requirements "Walk the Graph and trace requirements")
    *   [DSG_TRACE_VALIDATE_EDGE](#dsg_trace_validate_edge-validate-edge-is-used-at-least-once "Validate Edge is used at least once")
    *   [DSG_TRACE_UPWARDS](#dsg_trace_upwards-trace-upwards-using-covers-attribute "Trace upwards using Covers attribute")
    *   [DSG_TRACE_DOWNWARDS](#dsg_trace_downwards-trace-downwards-using-depends-attribute "Trace downwards using Depends attribute")
    *   [DSG_TRACE_DERIVED](#dsg_trace_derived-record-requirements-that-do-not-cover-anything "Record requirements that do not cover anything")
    *   [DSG_TRACE_UNCOVERED](#dsg_trace_uncovered-record-requirements-that-are-not-completely-covered "Record requirements that are not completely covered")
    *   [DSG_TRACE_CHECK_TITLE](#dsg_trace_check_title-when-tracing-upwards-or-downwards-match-title "When tracing upwards or downwards match title")
    *   [DSG_TRACE_COVERS_EXIST](#dsg_trace_covers_exist-cover-links-must-exist "Cover Links must exist")
    *   [DSG_TRACE_DEPENDS_EXIST](#dsg_trace_depends_exist-depend-links-must-exist "Depend Links must exist")

## Artefact::new

Origin: [src/common.rs:261:8](../src/common.rs?plain=1#L261)

Covers:
*   design
    *   [DSG_ART_PARSE](#dsg_art_parse-artefact-parses-input-lazily "Artefact parses input lazily")
*   Does not cover: formats

## ArtefactParser::parse

Origin: [src/parsers/mod.rs:114:16](../src/parsers/mod.rs?plain=1#L114)

Covers:
*   design
    *   [DSG_ART_PARSE_DETECT_DUPLICATE](#dsg_art_parse_detect_duplicate-detect-duplicates "Detect duplicates")
*   Does not cover: formats

## Controller::run

Origin: [src/controller.rs:149:16](../src/controller.rs?plain=1#L149)

Covers:
*   design
    *   [DSG_JOB_PARSE](#dsg_job_parse-parse-all-artefacts "Parse all Artefacts")
    *   [DSG_JOB_TRACE](#dsg_job_trace-trace-requirements "Trace Requirements")
*   Does not cover: formats

## Controller::run_jobs

Origin: [src/controller.rs:110:16](../src/controller.rs?plain=1#L110)

Covers:
*   design
    *   [DSG_JOB_RETURN_CODE](#dsg_job_return_code-return-code-indicates-if-tracing-is-correct "Return Code Indicates if Tracing is Correct")
*   Does not cover: formats

## Parser < '_ >::parse_macro

Origin: [src/parsers/rust.rs:110:8](../src/parsers/rust.rs?plain=1#L110)

Covers:
*   Does not cover: design
*   formats
    *   [FMT_RUST_COV](#fmt_rust_cov-rust-coverage-marks "Rust Coverage Marks")

## Tracing < 'graph >::add_req

Origin: [src/trace.rs:249:20](../src/trace.rs?plain=1#L249)

Covers:
*   design
    *   [DSG_TRACE_DETECT_DUPLICATE](#dsg_trace_detect_duplicate-detect-duplicate-requirements-in-different-artefacts "Detect duplicate Requirements in different Artefacts")
*   Does not cover: formats

## get_config

Origin: [src/main.rs:90:4](../src/main.rs?plain=1#L90)

Covers:
*   design
    *   [DSG_CONFIG_TOML](#dsg_config_toml-use-a-single-toml-file-as-configuration "Use a Single TOML File as Configuration")
*   Does not cover: formats

## main_rc

Origin: [src/main.rs:121:4](../src/main.rs?plain=1#L121)

Covers:
*   design
    *   [DSG_CLI_RETURN_CODE](#dsg_cli_return_code-set-return-code-to-indicate-success "Set return Code to indicate success")
*   Does not cover: formats

## parse

Origin: [src/parsers/readme.rs:72:4](../src/parsers/readme.rs?plain=1#L72)

Covers:
*   Does not cover: design
*   formats
    *   [FMT_README](#fmt_readme-single-requirement-per-file "Single Requirement Per File")

## run_cli_jobs

Origin: [src/main.rs:102:8](../src/main.rs?plain=1#L102)

Covers:
*   design
    *   [DSG_CLI](#dsg_cli-offer-a-simple-command-line-interface "Offer a simple Command Line Interface")
    *   [DSG_JOBS](#dsg_jobs-jobs-control-what-operations-to-perform "Jobs control what operations to perform")
*   Does not cover: formats

## DSG_ART_CONFIG: Artefact Configuration Fields

Origin: [doc/requirements/DESIGN.md:73](../doc/requirements/DESIGN.md?plain=1#L73)


*   ID
*   paths:  List of Paths or pattern with which to find the files
*   parser:   id of a parsing strategy, e.g. `Markdown Requirements`, `Rust
    Coverage Marks`, `JSON`
*   parser arguments: Object that is passed to the parser
*   caching: boolean, whether to cache or parse on every access



Covers:
*   Does not cover: requirements
*   architecture
    *   [ARCH_ARTEFACT](#arch_artefact-artefact "Artefact")

Covered By:
*   Not Covered by: formats, code

## DSG_ART_FILES: Artefact loads one or more Files

Origin: [doc/requirements/DESIGN.md:43](../doc/requirements/DESIGN.md?plain=1#L43)


An Artefact represents one or more files of the same type.



Covers:
*   Does not cover: requirements
*   architecture
    *   [ARCH_ARTEFACT](#arch_artefact-artefact "Artefact")

Covered By:
*   Not Covered by: formats, code

## DSG_ART_IGNORE_DERIVED: Ignore Derived Requirements

Origin: [doc/requirements/DESIGN.md:85](../doc/requirements/DESIGN.md?plain=1#L85)


Artefacts can be configured to ignore derived requirements




Covers:
*   Does not cover: requirements
*   Does not cover: architecture

Covered By:
*   Not Covered by: formats, code

## DSG_ART_PARSE: Artefact parses input lazily

Origin: [doc/requirements/DESIGN.md:51](../doc/requirements/DESIGN.md?plain=1#L51)


On demand, the Artefact parses the requirements in the files it represents.
The artefact stores the (untraced) requirements and all parsing errors



Covers:
*   Does not cover: requirements
*   architecture
    *   [ARCH_ARTEFACT](#arch_artefact-artefact "Artefact")

Covered By:
*   formats, code
    *   [Artefact::new](#artefactnew)

## DSG_ART_PARSE_COLLECT_ERRORS: Collect errors but keep parsing

Origin: [doc/requirements/DESIGN.md:66](../doc/requirements/DESIGN.md?plain=1#L66)


While parsing artefacts, all encountered errors are stored and parsing continues.



Covers:
*   requirements
    *   [REQ_PARSER_ERROR](#req_parser_error-useful-parser-errors "Useful Parser Errors")
    *   [REQ_LATE_ERROR](#req_late_error-collect-errors-but-continue-processing "Collect Errors but continue processing")
*   Does not cover: architecture

Covered By:
*   Not Covered by: formats, code

## DSG_ART_PARSE_DETECT_DUPLICATE: Detect duplicates

Origin: [doc/requirements/DESIGN.md:59](../doc/requirements/DESIGN.md?plain=1#L59)


While parsing, if there are two requirements the same identifier, log an error.



Covers:
*   requirements
    *   [REQ_UNIQUE_ID_v2](#req_unique_id_v2-requirements-have-a-unique-identifier "Requirements have a unique Identifier")
*   Does not cover: architecture

Covered By:
*   formats, code
    *   [ArtefactParser::parse](#artefactparserparse)

## DSG_CLI: Offer a simple Command Line Interface

Origin: [doc/requirements/DESIGN.md:160](../doc/requirements/DESIGN.md?plain=1#L160)


The tool should be invoked via a simple CLI and set the exit code to indicate if a job was successful.



Covers:
*   requirements
    *   [REQ_MACHINE_FRIENDLY](#req_machine_friendly-easy-to-include-in-automated-work-flows "Easy to include in automated work flows")
*   Does not cover: architecture

Covered By:
*   formats, code
    *   [run_cli_jobs](#run_cli_jobs)

## DSG_CLI_RETURN_CODE: Set return Code to indicate success

Origin: [doc/requirements/DESIGN.md:167](../doc/requirements/DESIGN.md?plain=1#L167)


Set the process' return code to:
*   `2` if there were fatal errors (invalid configuration or similar)
*   `1` if there were errors (file not found, parser errors, uncovered requirement, derived
    requirement, ...)
*   `0` otherwise



Covers:
*   requirements
    *   [REQ_MACHINE_FRIENDLY](#req_machine_friendly-easy-to-include-in-automated-work-flows "Easy to include in automated work flows")
    *   [UC_CHECK](#uc_check-check-for-correct-tracing "Check for correct Tracing")
*   Does not cover: architecture

Covered By:
*   formats, code
    *   [main_rc](#main_rc)

## DSG_CONFIG_TOML: Use a Single TOML File as Configuration

Origin: [doc/requirements/DESIGN.md:145](../doc/requirements/DESIGN.md?plain=1#L145)


The configuration should be placed in one file `requirements.toml`.
The format is TOML.
The structure of the Configuration is detailed in the Manual



Covers:
*   requirements
    *   [REQ_CONFIG](#req_config-simple-configuration-in-one-file "Simple Configuration in One File")
*   Does not cover: architecture

Covered By:
*   formats, code
    *   [get_config](#get_config)

## DSG_CTRL_CONFIG: Single Config File

Origin: [doc/requirements/DESIGN.md:180](../doc/requirements/DESIGN.md?plain=1#L180)


The Controller reads all information about the project structure from one single
file.



Covers:
*   requirements
    *   [REQ_CONFIG](#req_config-simple-configuration-in-one-file "Simple Configuration in One File")
*   architecture
    *   [ARCH_CONTROLLER](#arch_controller-controller "Controller")

Covered By:
*   Not Covered by: formats, code

## DSG_EXPORT_FORMAT_MARKDOWN: Export to Markdown

Origin: [doc/requirements/DESIGN.md:138](../doc/requirements/DESIGN.md?plain=1#L138)


Errors, Requirements, Status, Tracing Info can be exported as a useful
standalone Markdown File



Covers:
*   requirements
    *   [REQ_HUMAN_READABLE](#req_human_readable-human-readable-output "Human Readable Output")
*   Does not cover: architecture

Covered By:
*   formats, code
    *   [FMT_EXP_MD_ERROR](#fmt_exp_md_error-markdown-export-format-for-errors "Markdown Export Format for Errors")
    *   [FMT_EXP_MD_REQ](#fmt_exp_md_req-markdown-export-format-for-requirements "Markdown Export Format for Requirements")

## DSG_GRAPH: Artefact Graph

Origin: [doc/requirements/DESIGN.md:228](../doc/requirements/DESIGN.md?plain=1#L228)


The Class `Graph` holds a graph of `Artefact` objects as
given by the Configuration.



Covers:
*   Does not cover: requirements
*   architecture
    *   [ARCH_GRAPH](#arch_graph-graph "Graph")

Covered By:
*   Not Covered by: formats, code

## DSG_GRAPH_FORKS: Trace Edge Groups

Origin: [doc/requirements/DESIGN.md:236](../doc/requirements/DESIGN.md?plain=1#L236)


The Graph of artefacts has the following properties:

    * Forks
    * Artefacts
*   Edges in the Graph have a direction.
*   Edges always connect an Artefact to a Fork

covered either by code, or by `FMT` Requirements from `FORMATS.md`.
Code an `FORMATS.md` both have an edge to the same Fork below
`DESIGN.md`.
Each Design also has to be covered by a unit test, so unit tests are
below a separate fork below `DESIGN.md`



Covers:
*   Does not cover: requirements
*   architecture
    *   [ARCH_GRAPH](#arch_graph-graph "Graph")

Covered By:
*   Not Covered by: formats, code

## DSG_GRAPH_VAL_NO_LOOP: Validate that the Graph has no Loops

Origin: [doc/requirements/DESIGN.md:256](../doc/requirements/DESIGN.md?plain=1#L256)


After assembling of the graph, if a loop can be found in the graph of artefacts, an error is emitted.
This prevents further tracing.



Covers:
*   Does not cover: requirements
*   architecture
    *   [ARCH_GRAPH](#arch_graph-graph "Graph")

Covered By:
*   Not Covered by: formats, code

## DSG_JOBS: Jobs control what operations to perform

Origin: [doc/requirements/DESIGN.md:193](../doc/requirements/DESIGN.md?plain=1#L193)


One or more Jobs can be configured. Each Job specifies an operation to perform, the format that results
should be presented in and the file to store results in.



Covers:
*   Does not cover: requirements
*   Does not cover: architecture

Covered By:
*   formats, code
    *   [run_cli_jobs](#run_cli_jobs)

## DSG_JOB_FILE: Specify File to Store Reports in

Origin: [doc/requirements/DESIGN.md:218](../doc/requirements/DESIGN.md?plain=1#L218)


For each job, user can choose the file, that results are stored in, treating `-` as the stdout channel.



Covers:
*   Does not cover: requirements
*   Does not cover: architecture

Covered By:
*   Not Covered by: formats, code

## DSG_JOB_FORMAT: Specify Format of Reports

Origin: [doc/requirements/DESIGN.md:211](../doc/requirements/DESIGN.md?plain=1#L211)


For each job, user can choose the format that results are presented in



Covers:
*   requirements
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
*   Does not cover: architecture

Covered By:
*   Not Covered by: formats, code

## DSG_JOB_PARSE: Parse all Artefacts

Origin: [doc/requirements/DESIGN.md:198](../doc/requirements/DESIGN.md?plain=1#L198)


Parse the Artefacts, see infos about (untraced) requirements and parsing
errors



Covers:
*   requirements
    *   [UC_PARSE](#uc_parse-parse-artefacts "Parse Artefacts")
*   Does not cover: architecture

Covered By:
*   formats, code
    *   [Controller::run](#controllerrun)

## DSG_JOB_RETURN_CODE: Return Code Indicates if Tracing is Correct

Origin: [doc/requirements/DESIGN.md:222](../doc/requirements/DESIGN.md?plain=1#L222)


For each job, user can choose, if the return code should be set or not,
allowing for example, to update the tmx without indicating an error if the tracing is not
perfect yet.



Covers:
*   Does not cover: requirements
*   Does not cover: architecture

Covered By:
*   formats, code
    *   [Controller::run_jobs](#controllerrun_jobs)

## DSG_JOB_TRACE: Trace Requirements

Origin: [doc/requirements/DESIGN.md:205](../doc/requirements/DESIGN.md?plain=1#L205)


Trace Requirements and view info about traced requirements and errors



Covers:
*   requirements
    *   [UC_TMX](#uc_tmx-create-traceability-matrix "Create Traceability Matrix")
*   Does not cover: architecture

Covered By:
*   formats, code
    *   [Controller::run](#controllerrun)

## DSG_JSON_EXPORT: JSON for Exporting Results

Origin: [doc/requirements/DESIGN.md:129](../doc/requirements/DESIGN.md?plain=1#L129)


Errors, Requirements, Status, Tracing Info can be exported as JSON
The input files can be cached if tracing has not changed, checked into VCS or reviewed.



Covers:
*   requirements
    *   [REQ_MACHINE_READABLE](#req_machine_readable-machine-readable-output "Machine Readable Output")
    *   [REQ_CACHE_FRIENDLY](#req_cache_friendly-work-well-with-build-systems-that-cache "Work well with build systems that cache")
*   Does not cover: architecture

Covered By:
*   formats, code
    *   [FMT_JSON](#fmt_json-json-as-data-format "Json as Data Format")

## DSG_JSON_IMPORT: JSON for Importing Requirements

Origin: [doc/requirements/DESIGN.md:120](../doc/requirements/DESIGN.md?plain=1#L120)


Artefacts which can not be parsed by the tool are generated by other tools and imported via JSON.
The input files can be cached if their inputs have not changed, checked into VCS or reviewed.



Covers:
*   requirements
    *   [REQ_EXTENSIBLE](#req_extensible-extensible-parsing "Extensible Parsing")
    *   [REQ_CACHE_FRIENDLY](#req_cache_friendly-work-well-with-build-systems-that-cache "Work well with build systems that cache")
*   Does not cover: architecture

Covered By:
*   formats, code
    *   [FMT_JSON](#fmt_json-json-as-data-format "Json as Data Format")

## DSG_REQ_FIELDS: Requirement Fields

Origin: [doc/requirements/DESIGN.md:8](../doc/requirements/DESIGN.md?plain=1#L8)


Attributes of a requirement that this tool requires:
*   ID: a short string that uniquely identifies this requirement

*   Title:  Text that briefly summarizes this requirement (on line)
*   Description: Text that gives detailed description
*   Coverage: List of requirement IDs that are covered by this one
*   Dependencies: List of requirement IDs which cover this one
*   Tags:   List of Strings that can be used to categorize requirements
*   Delegates: List of requirements in the same artefact that can be covered instead of this one

*   Location:   Artefact that defines this requirement and the location inside
    the artefact where it is defined


*   Comment: Text with even more details, further reading, etc. that has a lower
    priority than Description which may be excluded from reports



Covers:
*   requirements
    *   [REQ_UP](#req_up-upward-coverage "Upward Coverage")
    *   [REQ_DOWN](#req_down-downward-coverage "Downward Coverage")
    *   [REQ_DELEGATION](#req_delegation-coverage-delegation "Coverage Delegation")
*   Does not cover: architecture

Covered By:
*   Not Covered by: formats, code

## DSG_TRACE: Walk the Graph and trace requirements

Origin: [doc/requirements/DESIGN.md:268](../doc/requirements/DESIGN.md?plain=1#L268)


Tracing is performed by inspecting all forks of the graph, recording
tracing information as it is encountered.
After all forks are processed, a final validation pass turns problems
encountered on the way into errors.


    requirements:
    1. Add all requirements of lower artefacts to the internal
        TracedRequirements if the requirement was not seen before.
    2. Add the requirement's `derived` and `covers` references to lists of
        invalid references
    3. if the requirement was new in (1), add the requirement to the list of
        derived requirements
2.  for all upper requirements:
    1.  find lower requirements that match upper's `depends`
    2.  for all lower artefacts, find lower requirements that cover upper
    3. for all coverages found:
        1.  remove them from the list of invalid references
        2.  remove lower from the list of derived requirements
        3.  add lower to `upper.lower[tine]` and vice versa
        4. if covered with title, add an error if the title is not matched
        correctly
    5.  if no coverage was found, add `(upper, fork)` to the list of uncovered
        requirements



Covers:
*   Does not cover: requirements
*   architecture
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")

Covered By:
*   Not Covered by: formats, code

## DSG_TRACE_CHECK_TITLE: When tracing upwards or downwards match title

Origin: [doc/requirements/DESIGN.md:354](../doc/requirements/DESIGN.md?plain=1#L354)


When tracing Upwards or Downwards, emit an error if the title of the coverage does
not match the title of the covered requirement



Covers:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
    *   [REQ_VAL_TITLE](#req_val_title-check-matching-title "Check matching title")
*   architecture
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")

Covered By:
*   Not Covered by: formats, code

Example:
### REQ_U: Title of Upper

    An Upper Requirement

    ### REQ_D: Title of Lower

    A Lower Requirement that covers REQ_U with an exactly mathcing title.

    Covers:
    *   REQ_U: Title of Upper

## DSG_TRACE_COVERS_EXIST: Cover Links must exist

Origin: [doc/requirements/DESIGN.md:376](../doc/requirements/DESIGN.md?plain=1#L376)


For each Requirement that is encountered, store all "covers" references
in a list of invalid references.
When the Requirement is successfully covered against a requirement matching
that reference, it is removed from the list of invalid references.




Covers:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
    *   [REQ_DOWN](#req_down-downward-coverage "Downward Coverage")
    *   [REQ_VAL_COVERAGE](#req_val_coverage-validate-coverage "Validate Coverage")
*   architecture
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")

Covered By:
*   Not Covered by: formats, code

## DSG_TRACE_DEPENDS_EXIST: Depend Links must exist

Origin: [doc/requirements/DESIGN.md:391](../doc/requirements/DESIGN.md?plain=1#L391)


Add all "depends" references to a list of invalid references and remove
when traced. See DSG_TRACE_COVERS_EXIST.



Covers:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
    *   [REQ_UP](#req_up-upward-coverage "Upward Coverage")
    *   [REQ_VAL_COVERAGE](#req_val_coverage-validate-coverage "Validate Coverage")
*   architecture
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")

Covered By:
*   Not Covered by: formats, code

## DSG_TRACE_DERIVED: Record requirements that do not cover anything

Origin: [doc/requirements/DESIGN.md:332](../doc/requirements/DESIGN.md?plain=1#L332)


Requirement R is derived if there is no Requirement U so that R covers U.



Covers:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
*   architecture
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")

Covered By:
*   Not Covered by: formats, code

## DSG_TRACE_DETECT_DUPLICATE: Detect duplicate Requirements in different Artefacts

Origin: [doc/requirements/DESIGN.md:403](../doc/requirements/DESIGN.md?plain=1#L403)


While tracing, if there are two requirements the same identifier, log an error.




Covers:
*   requirements
    *   [REQ_UNIQUE_ID_v2](#req_unique_id_v2-requirements-have-a-unique-identifier "Requirements have a unique Identifier")
*   Does not cover: architecture

Covered By:
*   formats, code
    *   [Tracing < 'graph >::add_req](#tracing--graph-add_req)

## DSG_TRACE_DOWNWARDS: Trace downwards using Depends attribute

Origin: [doc/requirements/DESIGN.md:321](../doc/requirements/DESIGN.md?plain=1#L321)


Requirement U covers Requirement D if D.id appears in U.Depends and
D.Artefact directly traces against U.Artefact



Covers:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
    *   [REQ_DOWN](#req_down-downward-coverage "Downward Coverage")
    *   [REQ_MATCH_ID](#req_match_id-match-by-id "Match by ID")
*   architecture
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")

Covered By:
*   Not Covered by: formats, code

## DSG_TRACE_UNCOVERED: Record requirements that are not completely covered

Origin: [doc/requirements/DESIGN.md:340](../doc/requirements/DESIGN.md?plain=1#L340)


Record Requirement R as Uncovered along Fork F if there is no
Requirement D which covers R, where R is from the Artefact above F and
D is from one of the artefacts below D.

covered by a unittest AND (it is covered by Code OR it is covered by
a FMT requirement).



Covers:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
*   architecture
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")

Covered By:
*   Not Covered by: formats, code

## DSG_TRACE_UPWARDS: Trace upwards using Covers attribute

Origin: [doc/requirements/DESIGN.md:310](../doc/requirements/DESIGN.md?plain=1#L310)


Requirement U covers Requirement D if U.id appears in D.Covers and
D.Artefact directly traces against U.Artefact



Covers:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
    *   [REQ_UP](#req_up-upward-coverage "Upward Coverage")
    *   [REQ_MATCH_ID](#req_match_id-match-by-id "Match by ID")
*   architecture
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")

Covered By:
*   Not Covered by: formats, code

## DSG_TRACE_VALIDATE_EDGE: Validate Edge is used at least once

Origin: [doc/requirements/DESIGN.md:300](../doc/requirements/DESIGN.md?plain=1#L300)


After tracing, if an edge can be found, along which no requirement is
covered, an error is emitted. This is likely a misconfiguration.




Covers:
*   Does not cover: requirements
*   architecture
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")

Covered By:
*   Not Covered by: formats, code

## FMT_CTAGS: Export Requirements as CTags

Origin: [doc/requirements/FORMATS.md:321](../doc/requirements/FORMATS.md?plain=1#L321)


Emit Tag with type R for the Requirement
Type C for where a req is covered
Type D for where it is depended on.



Covers:
*   Does not cover: requirements
*   Does not cover: design

Covered By:
*   Not Covered by: code

## FMT_EXP_MD_ERROR: Markdown Export Format for Errors

Origin: [doc/requirements/FORMATS.md:297](../doc/requirements/FORMATS.md?plain=1#L297)


Errors are printed as one chapter per type, each error a bullet point with description and
links to relevant error detail inside the same document.



Covers:
*   Does not cover: requirements
*   design
    *   [DSG_EXPORT_FORMAT_MARKDOWN](#dsg_export_format_markdown-export-to-markdown "Export to Markdown")

Covered By:
*   Not Covered by: code

## FMT_EXP_MD_REQ: Markdown Export Format for Requirements

Origin: [doc/requirements/FORMATS.md:305](../doc/requirements/FORMATS.md?plain=1#L305)


Each Requirement is a chapter with `ID: Title`.
The chapter contains:

*   Description
*   Covers and Depends
    *   as specified in the requirement
    *   links to actually traced requirements grouped by edge
*   All other Attributes



Covers:
*   Does not cover: requirements
*   design
    *   [DSG_EXPORT_FORMAT_MARKDOWN](#dsg_export_format_markdown-export-to-markdown "Export to Markdown")

Covered By:
*   Not Covered by: code

## FMT_FILE_ENCODINGS: Handle File Encodings

Origin: [doc/requirements/FORMATS.md:48](../doc/requirements/FORMATS.md?plain=1#L48)


When not otherwise specified, Text Files are read as UTF-8 and encoding errors are
replaced.



Covers:
*   requirements
    *   [REQ_UNICODE_SAFE](#req_unicode_safe-sane-handling-of-unicode "Sane Handling of unicode")
*   Does not cover: design

Covered By:
*   Not Covered by: code

## FMT_ID_v2: Requirement Identifier

Origin: [doc/requirements/FORMATS.md:29](../doc/requirements/FORMATS.md?plain=1#L29)


Requirement identifier consist of letters, digits and underscore, specifically
they match the Regular Expression





Covers:
*   requirements
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
*   Does not cover: design

Covered By:
*   Not Covered by: code

Comment:
Discussion in the [README](README.md#requirement-ids)

History:
*   v2: use to Unicode Identifiers

## FMT_JSON: Json as Data Format

Origin: [doc/requirements/FORMATS.md:258](../doc/requirements/FORMATS.md?plain=1#L258)


each list sorted to minimize diff !

  * Unique ID
  * Version Info
  * If File
    * Path
    * Hash over File
  * If Group
    * Child Artefacts (have no upwards or downwards artefact links)
  * Upwards Artefact IDs
  * Downwards Aretefact IDs
  * Requirements defined in that Artefact
    * ID
    * Title
    * Description
    * Location
    * Covers REQ_Id\*
    * Covered-by Req_Id


For human readability,
each value or key-value pair should be on its own line,
indented with 4 space per level.



Covers:
*   Does not cover: requirements
*   design
    *   [DSG_JSON_IMPORT](#dsg_json_import-json-for-importing-requirements "JSON for Importing Requirements")
    *   [DSG_JSON_EXPORT](#dsg_json_export-json-for-exporting-results "JSON for Exporting Results")

Covered By:
*   Not Covered by: code

## FMT_JSON_REQUIREMENT: JSON Requirements Format

Origin: [doc/requirements/FORMATS.md:216](../doc/requirements/FORMATS.md?plain=1#L216)

Covers:
*   requirements
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
*   Does not cover: design

Covered By:
*   Not Covered by: code

## FMT_MARKDOWN_REQUIREMENT: Markdown File Format

Origin: [doc/requirements/FORMATS.md:81](../doc/requirements/FORMATS.md?plain=1#L81)


The artefact is a Markdown file with freely chosen layout.  A Requirement is in
a heading line with requirement ID and title, followed by description and other
attributes.



Covers:
*   requirements
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
*   Does not cover: design

Covered By:
*   Not Covered by: code

## FMT_MD_ATTRIBUTES: Attributes

Origin: [doc/requirements/FORMATS.md:115](../doc/requirements/FORMATS.md?plain=1#L115)


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

## FMT_MD_DESC: Description

Origin: [doc/requirements/FORMATS.md:97](../doc/requirements/FORMATS.md?plain=1#L97)


The paragraphs following the start of the requirement make up the description of
the requirement.

*   The Start of another Requirement.
*   The start of an Attribute Paragraph
*   A Heading the same level or less. This ends the Requirement.





Covers:
*   Does not cover: requirements
*   Does not cover: design

Covered By:
*   Not Covered by: code

## FMT_MD_DESC_HEADINGS: Heading Level in Description is adjusted

Origin: [doc/requirements/FORMATS.md:109](../doc/requirements/FORMATS.md?plain=1#L109)


Headings with a lower level than the starting one, that do not start a nested
requirement are added to the description. Their heading level is adjusted by
removing as many leading `#` as the requirement had



Covers:
*   Does not cover: requirements
*   Does not cover: design

Covered By:
*   Not Covered by: code

## FMT_MD_OPT_PREFIX: List of Prefixes

Origin: [doc/requirements/FORMATS.md:174](../doc/requirements/FORMATS.md?plain=1#L174)


A List of strings can be passed, which is used to prevent the parser from
creating unintended requirements from headlines which accidentally have the
right form.

normal headings, if the identifier of the would be requirement does not start
with one of the list of prefixes. If the list is empty, no prefix matching is
performed and all matching lines lead to a requirement.




Covers:
*   Does not cover: requirements
*   Does not cover: design

Covered By:
*   Not Covered by: code

## FMT_MD_START: Requirement Start

Origin: [doc/requirements/FORMATS.md:92](../doc/requirements/FORMATS.md?plain=1#L92)


A Requirement starts with a `#` heading of any level that has the form `ID:
TITLE`.



Covers:
*   Does not cover: requirements
*   Does not cover: design

Covered By:
*   Not Covered by: code

## FMT_README: Single Requirement Per File

Origin: [doc/requirements/FORMATS.md:191](../doc/requirements/FORMATS.md?plain=1#L191)


Artefact of type MonoRequirement emit exactly one Requirement with the following
attributes:
*   Id: The stem of the file path (i.e. `README.md`)
*   Title:  The first line containing Word-Characters with all non-word
    characters trimmed of both ends of the line. (Allowing Markdown heading,
    C style comments, ...)
*   Depends: Every Requirement-Id that immediately follows a fat arrow (`=>`).



Covers:
*   requirements
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
*   Does not cover: design

Covered By:
*   code
    *   [parse](#parse)

Comment:
See this projects README for examples.

## FMT_RUST_COV: Rust Coverage Marks

Origin: [doc/requirements/FORMATS.md:222](../doc/requirements/FORMATS.md?plain=1#L222)


Parse `requirement_covered!(REQ_ID)` and `requirement_covered!(REQ_ID,"TITLE")`



Covers:
*   requirements
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
*   Does not cover: design

Covered By:
*   code
    *   [Parser < '_ >::parse_macro](#parser--_-parse_macro)

## FMT_UNICODE_NORMALIZE: Normalize Unicode during read

Origin: [doc/requirements/FORMATS.md:57](../doc/requirements/FORMATS.md?plain=1#L57)


All input strings are unicode normalizes as
[NFC](https://www.unicode.org/reports/tr15/#Normalization_Forms_Table).
This means that
*   All output derived from input will be NFC normalized
*   Identifier Matching can be done on the byte level



Covers:
*   requirements
    *   [REQ_UNICODE_SAFE](#req_unicode_safe-sane-handling-of-unicode "Sane Handling of unicode")
*   Does not cover: design

Covered By:
*   Not Covered by: code

Comment:
See [Rust RFC 2457](https://rust-lang.github.io/rfcs/2457-non-ascii-idents.html) on the topic.

This means two requirement ids are equal if their NFC forms are equal.

## README: Requirement Tracing

Origin: [README.md](../README.md)

Covered By:
*   requirements
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
    *   [REQ_UP](#req_up-upward-coverage "Upward Coverage")
    *   [REQ_DOWN](#req_down-downward-coverage "Downward Coverage")
    *   [REQ_EXTENSIBLE](#req_extensible-extensible-parsing "Extensible Parsing")
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
    *   [REQ_HUMAN_READABLE](#req_human_readable-human-readable-output "Human Readable Output")
    *   [REQ_MACHINE_READABLE](#req_machine_readable-machine-readable-output "Machine Readable Output")
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
    *   [REQ_FAST](#req_fast-fast "Fast")
    *   [REQ_CACHE_FRIENDLY](#req_cache_friendly-work-well-with-build-systems-that-cache "Work well with build systems that cache")
    *   [REQ_VAL_TITLE](#req_val_title-check-matching-title "Check matching title")
    *   [REQ_CONFIG](#req_config-simple-configuration-in-one-file "Simple Configuration in One File")
    *   [REQ_MACHINE_FRIENDLY](#req_machine_friendly-easy-to-include-in-automated-work-flows "Easy to include in automated work flows")
    *   [REQ_LATE_ERROR](#req_late_error-collect-errors-but-continue-processing "Collect Errors but continue processing")

## REQ_CACHE_FRIENDLY: Work well with build systems that cache

Origin: [doc/requirements/REQUIREMENTS.md:141](../doc/requirements/REQUIREMENTS.md?plain=1#L141)


Report all files which are consumed, so that build systems like make or
ninja can know when an input has changed an Given Data and a requested format, the formatter formats the given data in the
given format.rerun the tool.



Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats, architecture
    *   [DSG_JSON_IMPORT](#dsg_json_import-json-for-importing-requirements "JSON for Importing Requirements")
    *   [DSG_JSON_EXPORT](#dsg_json_export-json-for-exporting-results "JSON for Exporting Results")

## REQ_CONFIG: Simple Configuration in One File

Origin: [doc/requirements/REQUIREMENTS.md:132](../doc/requirements/REQUIREMENTS.md?plain=1#L132)


All Configuration is stored in a single file using a common Format that is
editable for humans and machine readable.



Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats, architecture
    *   [DSG_CONFIG_TOML](#dsg_config_toml-use-a-single-toml-file-as-configuration "Use a Single TOML File as Configuration")
    *   [DSG_CTRL_CONFIG](#dsg_ctrl_config-single-config-file "Single Config File")

## REQ_DELEGATION: Coverage Delegation

Origin: [doc/requirements/REQUIREMENTS.md:73](../doc/requirements/REQUIREMENTS.md?plain=1#L73)


A Requirement delegates to another requirement in the same artefact by including
the id of the lower one in its Dependencies attribute.

requirement.



Covers:
*   Does not cover: readme

Covered By:
*   design, formats, architecture
    *   [DSG_REQ_FIELDS](#dsg_req_fields-requirement-fields "Requirement Fields")

## REQ_DOWN: Downward Coverage

Origin: [doc/requirements/REQUIREMENTS.md:67](../doc/requirements/REQUIREMENTS.md?plain=1#L67)


A Requirement is covered by a lower one by including the id of the lower one in
its Dependencies attribute.



Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats, architecture
    *   [DSG_REQ_FIELDS](#dsg_req_fields-requirement-fields "Requirement Fields")
    *   [DSG_TRACE_DOWNWARDS](#dsg_trace_downwards-trace-downwards-using-depends-attribute "Trace downwards using Depends attribute")
    *   [DSG_TRACE_COVERS_EXIST](#dsg_trace_covers_exist-cover-links-must-exist "Cover Links must exist")

## REQ_EXTENSIBLE: Extensible Parsing

Origin: [doc/requirements/REQUIREMENTS.md:28](../doc/requirements/REQUIREMENTS.md?plain=1#L28)


If internal parsers are not able to work on an Artefact, external tools can be
incorporated.



Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats, architecture
    *   [DSG_JSON_IMPORT](#dsg_json_import-json-for-importing-requirements "JSON for Importing Requirements")

## REQ_FAST: Fast

Origin: [doc/requirements/REQUIREMENTS.md:137](../doc/requirements/REQUIREMENTS.md?plain=1#L137)


Show results quickly, especially if only a small query is given.



Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats, architecture
    *   [ARCH_CLI](#arch_cli-command-line-interface "Command Line Interface")

## REQ_FORMATS: Well defined Formats

Origin: [doc/requirements/REQUIREMENTS.md:49](../doc/requirements/REQUIREMENTS.md?plain=1#L49)


To work with external programs as parsers or to process the output, the formats used must be well
defined.



Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats, architecture
    *   [DSG_JOB_FORMAT](#dsg_job_format-specify-format-of-reports "Specify Format of Reports")
    *   [FMT_ID_v2](#fmt_id_v2-requirement-identifier "Requirement Identifier")
    *   [FMT_MARKDOWN_REQUIREMENT](#fmt_markdown_requirement-markdown-file-format "Markdown File Format")
    *   [FMT_README](#fmt_readme-single-requirement-per-file "Single Requirement Per File")
    *   [FMT_JSON_REQUIREMENT](#fmt_json_requirement-json-requirements-format "JSON Requirements Format")
    *   [FMT_RUST_COV](#fmt_rust_cov-rust-coverage-marks "Rust Coverage Marks")

## REQ_HUMAN_READABLE: Human Readable Output

Origin: [doc/requirements/REQUIREMENTS.md:45](../doc/requirements/REQUIREMENTS.md?plain=1#L45)


Information can be returned in a format that can easily be read by humans



Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats, architecture
    *   [DSG_EXPORT_FORMAT_MARKDOWN](#dsg_export_format_markdown-export-to-markdown "Export to Markdown")

## REQ_IDENTIFIEABLE: Show versions of input artefacts in output

Origin: [doc/requirements/REQUIREMENTS.md:33](../doc/requirements/REQUIREMENTS.md?plain=1#L33)


When reading the output, each input must be clearly identifiable.
For example by its:
*   git describe
*   hash
*   file modification time



Covers:
*   Does not cover: readme

Covered By:
*   Not Covered by: design, formats, architecture

## REQ_INSTALL: Easy to install

Origin: [doc/requirements/REQUIREMENTS.md:23](../doc/requirements/REQUIREMENTS.md?plain=1#L23)


The tool should be distributed as an executable without depending on
libraries, files, etc.



Covers:
*   Does not cover: readme

Covered By:
*   design, formats, architecture
    *   [ARCH_CLI](#arch_cli-command-line-interface "Command Line Interface")

## REQ_LATE_ERROR: Collect Errors but continue processing

Origin: [doc/requirements/REQUIREMENTS.md:86](../doc/requirements/REQUIREMENTS.md?plain=1#L86)


When errors are encountered in parsing, tracing or outputting, processing
continues as long as possible and then all errors are reported.



Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats, architecture
    *   [DSG_ART_PARSE_COLLECT_ERRORS](#dsg_art_parse_collect_errors-collect-errors-but-keep-parsing "Collect errors but keep parsing")

## REQ_MACHINE_FRIENDLY: Easy to include in automated work flows

Origin: [doc/requirements/REQUIREMENTS.md:19](../doc/requirements/REQUIREMENTS.md?plain=1#L19)


For ease of integration into other tools, all functionality must be available via a CLI.



Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats, architecture
    *   [DSG_CLI](#dsg_cli-offer-a-simple-command-line-interface "Offer a simple Command Line Interface")
    *   [DSG_CLI_RETURN_CODE](#dsg_cli_return_code-set-return-code-to-indicate-success "Set return Code to indicate success")
    *   [ARCH_CLI](#arch_cli-command-line-interface "Command Line Interface")

## REQ_MACHINE_READABLE: Machine Readable Output

Origin: [doc/requirements/REQUIREMENTS.md:41](../doc/requirements/REQUIREMENTS.md?plain=1#L41)


Information can be returned in a format that can easily be read by other tools



Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats, architecture
    *   [DSG_JSON_EXPORT](#dsg_json_export-json-for-exporting-results "JSON for Exporting Results")

## REQ_MATCH_ID: Match by ID

Origin: [doc/requirements/REQUIREMENTS.md:91](../doc/requirements/REQUIREMENTS.md?plain=1#L91)


A Requirements covers another by its ID.



Covers:
*   Does not cover: readme

Covered By:
*   design, formats, architecture
    *   [DSG_TRACE_UPWARDS](#dsg_trace_upwards-trace-upwards-using-covers-attribute "Trace upwards using Covers attribute")
    *   [DSG_TRACE_DOWNWARDS](#dsg_trace_downwards-trace-downwards-using-depends-attribute "Trace downwards using Depends attribute")

## REQ_PARSER_ERROR: Useful Parser Errors

Origin: [doc/requirements/REQUIREMENTS.md:81](../doc/requirements/REQUIREMENTS.md?plain=1#L81)


Parser Errors give the precise location and type of the problem, for example filename with
line number of the artefact.



Covers:
*   Does not cover: readme

Covered By:
*   design, formats, architecture
    *   [DSG_ART_PARSE_COLLECT_ERRORS](#dsg_art_parse_collect_errors-collect-errors-but-keep-parsing "Collect errors but keep parsing")

## REQ_TRACE: Determine which requirements cover which

Origin: [doc/requirements/REQUIREMENTS.md:5](../doc/requirements/REQUIREMENTS.md?plain=1#L5)


Compute tracing for each Requirement, whether it is covered, uncovered, covers
another requirement or is derived.



Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats, architecture
    *   [DSG_TRACE_UPWARDS](#dsg_trace_upwards-trace-upwards-using-covers-attribute "Trace upwards using Covers attribute")
    *   [DSG_TRACE_DOWNWARDS](#dsg_trace_downwards-trace-downwards-using-depends-attribute "Trace downwards using Depends attribute")
    *   [DSG_TRACE_DERIVED](#dsg_trace_derived-record-requirements-that-do-not-cover-anything "Record requirements that do not cover anything")
    *   [DSG_TRACE_UNCOVERED](#dsg_trace_uncovered-record-requirements-that-are-not-completely-covered "Record requirements that are not completely covered")
    *   [DSG_TRACE_CHECK_TITLE](#dsg_trace_check_title-when-tracing-upwards-or-downwards-match-title "When tracing upwards or downwards match title")
    *   [DSG_TRACE_COVERS_EXIST](#dsg_trace_covers_exist-cover-links-must-exist "Cover Links must exist")
    *   [DSG_TRACE_DEPENDS_EXIST](#dsg_trace_depends_exist-depend-links-must-exist "Depend Links must exist")
    *   [ARCH_PARSER](#arch_parser-parser "Parser")
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")
    *   [ARCH_FORMATTER](#arch_formatter-format-output-in-requested-format "Format output in requested Format")

## REQ_UNICODE_SAFE: Sane Handling of unicode

Origin: [doc/requirements/REQUIREMENTS.md:54](../doc/requirements/REQUIREMENTS.md?plain=1#L54)


Some Characters can be represented by multiple different sequences of Unicode
Code Points. Also Unicode Encodings like UTF-8 can encode the same Codepoint
as different bytes.




Covers:
*   Does not cover: readme

Covered By:
*   design, formats, architecture
    *   [FMT_FILE_ENCODINGS](#fmt_file_encodings-handle-file-encodings "Handle File Encodings")
    *   [FMT_UNICODE_NORMALIZE](#fmt_unicode_normalize-normalize-unicode-during-read "Normalize Unicode during read")

## REQ_UNIQUE_ID_v2: Requirements have a unique Identifier

Origin: [doc/requirements/REQUIREMENTS.md:10](../doc/requirements/REQUIREMENTS.md?plain=1#L10)


Each requirement must be identifiable by a short, unique string.
All unicode symbols typically used as identifiers must be possible,
though parsers may restrict this



Covers:
*   Does not cover: readme

Covered By:
*   design, formats, architecture
    *   [DSG_ART_PARSE_DETECT_DUPLICATE](#dsg_art_parse_detect_duplicate-detect-duplicates "Detect duplicates")
    *   [DSG_TRACE_DETECT_DUPLICATE](#dsg_trace_detect_duplicate-detect-duplicate-requirements-in-different-artefacts "Detect duplicate Requirements in different Artefacts")

History:
*   v2: Unicode

## REQ_UP: Upward Coverage

Origin: [doc/requirements/REQUIREMENTS.md:62](../doc/requirements/REQUIREMENTS.md?plain=1#L62)


A Requirement covers a higher one by including the id of the higer one in its
Coverage attribute.



Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats, architecture
    *   [DSG_REQ_FIELDS](#dsg_req_fields-requirement-fields "Requirement Fields")
    *   [DSG_TRACE_UPWARDS](#dsg_trace_upwards-trace-upwards-using-covers-attribute "Trace upwards using Covers attribute")
    *   [DSG_TRACE_DEPENDS_EXIST](#dsg_trace_depends_exist-depend-links-must-exist "Depend Links must exist")

## REQ_VAL_COVERAGE: Validate Coverage

Origin: [doc/requirements/REQUIREMENTS.md:126](../doc/requirements/REQUIREMENTS.md?plain=1#L126)


An error is reported for a Coverage claim for which no Requirement exists in the
relevant artefacts.




Covers:
*   Does not cover: readme

Covered By:
*   design, formats, architecture
    *   [DSG_TRACE_COVERS_EXIST](#dsg_trace_covers_exist-cover-links-must-exist "Cover Links must exist")
    *   [DSG_TRACE_DEPENDS_EXIST](#dsg_trace_depends_exist-depend-links-must-exist "Depend Links must exist")

## REQ_VAL_TITLE: Check matching title

Origin: [doc/requirements/REQUIREMENTS.md:95](../doc/requirements/REQUIREMENTS.md?plain=1#L95)


A Coverage link that is established by requirement ID can be verified by
comparing the requirement's title.



Covers:
*   readme
    *   [README](#readme-requirement-tracing "Requirement Tracing")

Covered By:
*   design, formats, architecture
    *   [DSG_TRACE_CHECK_TITLE](#dsg_trace_check_title-when-tracing-upwards-or-downwards-match-title "When tracing upwards or downwards match title")

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

## UC_CHECK: Check for correct Tracing

Origin: [doc/requirements/REQUIREMENTS.md:165](../doc/requirements/REQUIREMENTS.md?plain=1#L165)


Like `UC_TRACE` but the only output of interest is whether there were tracing errors or not,
for use in CI/CD Pipelines.


Covers:
*   Does not cover: readme

Covered By:
*   design, formats, architecture
    *   [DSG_CLI_RETURN_CODE](#dsg_cli_return_code-set-return-code-to-indicate-success "Set return Code to indicate success")

## UC_PARSE: Parse Artefacts

Origin: [doc/requirements/REQUIREMENTS.md:149](../doc/requirements/REQUIREMENTS.md?plain=1#L149)


A Set of artefacts are parsed, reporting all requirements and errors.



Covers:
*   Does not cover: readme

Covered By:
*   design, formats, architecture
    *   [DSG_JOB_PARSE](#dsg_job_parse-parse-all-artefacts "Parse all Artefacts")

Parameters:
*   Artefacts to Parse

## UC_TMX: Create Traceability Matrix

Origin: [doc/requirements/REQUIREMENTS.md:156](../doc/requirements/REQUIREMENTS.md?plain=1#L156)


All requirements are matched up and down the Tracing Graph. The results are
stored in a file and bad tracing is reported.



Covers:
*   Does not cover: readme

Covered By:
*   design, formats, architecture
    *   [DSG_JOB_TRACE](#dsg_job_trace-trace-requirements "Trace Requirements")
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")

Parameters:
*   Tracing Report Format
*   Tracing Report File
