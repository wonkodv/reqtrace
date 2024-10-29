

# Uncovered Requirements


## requirements => [design, formats, architecture]

*   [REQ_IDENTIFIEABLE](#req_identifieable-show-versions-of-input-artefacts-in-output "Show versions of input artefacts in output")

## design => [formats, code]

*   [DSG_AGGREGATOR](#dsg_aggregator-cross-referenced-trace-graph "Cross Referenced Trace Graph")
*   [DSG_ART_IGNORE_DERIVED](#dsg_art_ignore_derived-ignore-derived-requirements "Ignore Derived Requirements")
*   [DSG_GRAPH](#dsg_graph-artefact-graph "Artefact Graph")
*   [DSG_GRAPH_RELATION](#dsg_graph_relation-artefact-relationships "Artefact Relationships")
*   [DSG_PARSER](#dsg_parser-parse-data "Parse Data")
*   [DSG_REQ_FIELDS](#dsg_req_fields-requirement-fields "Requirement Fields")
*   [DSG_TRACE](#dsg_trace-walk-the-graph-and-trace-requirements "Walk the Graph and trace requirements")
*   [DSG_TRACED_GRAPH](#dsg_traced_graph-tracing-information-of-grpah "Tracing Information of Grpah")
*   [DSG_TRACE_CHECK_TITLE](#dsg_trace_check_title-when-tracing-upwards-or-downwards-match-title "When tracing upwards or downwards match title")
*   [DSG_TRACE_COLLECT](#dsg_trace_collect-collect-requirements-from-artefact "Collect Requirements from Artefact")
*   [DSG_TRACE_DERIVED](#dsg_trace_derived-record-requirements-that-do-not-cover-anything "Record requirements that do not cover anything")
*   [DSG_TRACE_RELATION](#dsg_trace_relation-trace-relation "Trace Relation")
*   [DSG_TRACE_UNCOVERED](#dsg_trace_uncovered-record-requirements-that-are-not-completely-covered "Record requirements that are not completely covered")
*   [DSG_TRACE_VALIDATE_EDGE](#dsg_trace_validate_edge-validate-edge-is-used-at-least-once "Validate Edge is used at least once")

## formats => [code]

*   [FMT_EXPORT_CTAGS](#fmt_export_ctags-export-requirements-as-ctags "Export Requirements as CTags")
*   [FMT_EXPORT_JSON](#fmt_export_json-json-for-exporting-results "JSON for Exporting Results")
*   [FMT_EXPORT_MARKDOWN](#fmt_export_markdown-export-to-markdown "Export to Markdown")
*   [FMT_FILE_ENCODINGS](#fmt_file_encodings-handle-file-encodings "Handle File Encodings")
*   [FMT_ID_v2](#fmt_id_v2-requirement-identifier "Requirement Identifier")
*   [FMT_IMPORT_JSON](#fmt_import_json-json-for-importing-requirements "JSON for Importing Requirements")
*   [FMT_IMPORT_MARKDOWN_REQUIREMENT](#fmt_import_markdown_requirement-markdown-file-format "Markdown File Format")
*   [FMT_MD_ATTRIBUTES](#fmt_md_attributes-attributes "Attributes")
*   [FMT_MD_DESC](#fmt_md_desc-description "Description")
*   [FMT_MD_DESC_HEADINGS](#fmt_md_desc_headings-heading-level-in-description-is-adjusted "Heading Level in Description is adjusted")
*   [FMT_MD_OPT_PREFIX](#fmt_md_opt_prefix-list-of-prefixes "List of Prefixes")
*   [FMT_MD_START](#fmt_md_start-requirement-start "Requirement Start")
*   [FMT_UNICODE_NORMALIZE](#fmt_unicode_normalize-normalize-unicode-during-read "Normalize Unicode during read")


# Derived Requirements


## architecture

*   [ARCH_ARTEFACT](#arch_artefact-artefact "Artefact")
*   [ARCH_CONTROLLER](#arch_controller-controller "Controller")
*   [ARCH_REQUIREMENT](#arch_requirement-requirement "Requirement")

## design

*   [DSG_AGGREGATOR](#dsg_aggregator-cross-referenced-trace-graph "Cross Referenced Trace Graph")

## formats

*   [FMT_EXPORT_CTAGS](#fmt_export_ctags-export-requirements-as-ctags "Export Requirements as CTags")


# Requirements

## architecture

## ARCH_ARTEFACT: Artefact

Origin: architecture, [doc/requirements/ARCHITECTURE.md:89](../doc/requirements/ARCHITECTURE.md?plain=1#L89)


An Artefact is a list of requirements, parsed from one or more files by a parser




Upwards Tracing:
*   requirements => [design, formats, architecture]
    *   Derived

Downwards Tracing:
*   architecture => [design]
    *   [DSG_ART_FILES](#dsg_art_files-artefact-loads-one-or-more-files "Artefact loads one or more Files")
        Reference: [doc/requirements/DESIGN.md:353](../doc/requirements/DESIGN.md?plain=1#L353)
    *   [DSG_ART_IGNORE_DERIVED](#dsg_art_ignore_derived-ignore-derived-requirements "Ignore Derived Requirements")
        Reference: [doc/requirements/DESIGN.md:369](../doc/requirements/DESIGN.md?plain=1#L369)
    *   [DSG_ART_PARSE_COLLECT_ERRORS](#dsg_art_parse_collect_errors-collect-errors-but-keep-parsing "Collect errors but keep parsing")
        Reference: [doc/requirements/DESIGN.md:361](../doc/requirements/DESIGN.md?plain=1#L361)
    *   [DSG_CTRL_DETECT_DUPLICATE_REQS](#dsg_ctrl_detect_duplicate_reqs-detect-duplicates "Detect duplicates")
        Reference: [doc/requirements/DESIGN.md:82](../doc/requirements/DESIGN.md?plain=1#L82)

## ARCH_CLI: Command Line Interface

Origin: architecture, [doc/requirements/ARCHITECTURE.md:22](../doc/requirements/ARCHITECTURE.md?plain=1#L22)


All functionality of the tool is exposed in a simple command line interface.
The CLI is shipped as a statically linked binary.
Other Tools can interface with the CLI to provide caching, lazy parsing of complicated input formats
and generation of complicated output formats



Upwards Tracing:
*   requirements => [design, formats, architecture]
    *   [REQ_FAST](#req_fast-fast "Fast")
        Reference: [doc/requirements/ARCHITECTURE.md:32](../doc/requirements/ARCHITECTURE.md?plain=1#L32)
    *   [REQ_INSTALL](#req_install-easy-to-install "Easy to install")
        Reference: [doc/requirements/ARCHITECTURE.md:31](../doc/requirements/ARCHITECTURE.md?plain=1#L31)
    *   [REQ_MACHINE_FRIENDLY](#req_machine_friendly-easy-to-include-in-automated-work-flows "Easy to include in automated work flows")
        Reference: [doc/requirements/ARCHITECTURE.md:30](../doc/requirements/ARCHITECTURE.md?plain=1#L30)

Downwards Tracing:
*   architecture => [design]
    *   [DSG_CLI](#dsg_cli-command-line-interface "Command Line Interface")
        Reference: [doc/requirements/DESIGN.md:16](../doc/requirements/DESIGN.md?plain=1#L16)
    *   [DSG_CLI_RETURN_CODE](#dsg_cli_return_code-set-return-code-to-indicate-success "Set return Code to indicate success")
        Reference: [doc/requirements/DESIGN.md:29](../doc/requirements/DESIGN.md?plain=1#L29)

## ARCH_CONTROLLER: Controller

Origin: architecture, [doc/requirements/ARCHITECTURE.md:34](../doc/requirements/ARCHITECTURE.md?plain=1#L34)


The controller orchestrates the other components into a pipeline

*   Pass Files to Parsers to obtain Requirements
*   Put Requirements into Artefacts
*   Assemble Artefacts into Graph
*   Give Graph to Tracer, which computes Tracing Information
*   Give Artefacts, the Graph, Tracing and any errors to Formatter which puts them into files



Upwards Tracing:
*   requirements => [design, formats, architecture]
    *   Derived

Downwards Tracing:
*   architecture => [design]
    *   [DSG_CTRL_CONFIG](#dsg_ctrl_config-single-config-file "Single Config File")
        Reference: [doc/requirements/DESIGN.md:64](../doc/requirements/DESIGN.md?plain=1#L64)
    *   [DSG_CTRL_FORMAT](#dsg_ctrl_format-format-job-output "Format Job Output")
        Reference: [doc/requirements/DESIGN.md:110](../doc/requirements/DESIGN.md?plain=1#L110)
    *   [DSG_CTRL_GRAPH](#dsg_ctrl_graph-construct-graph "Construct Graph")
        Reference: [doc/requirements/DESIGN.md:92](../doc/requirements/DESIGN.md?plain=1#L92)
    *   [DSG_CTRL_PARSE](#dsg_ctrl_parse-parse-all-artefacts "Parse all Artefacts")
        Reference: [doc/requirements/DESIGN.md:73](../doc/requirements/DESIGN.md?plain=1#L73)
    *   [DSG_CTRL_RETURN_CODE](#dsg_ctrl_return_code-return-code-indicates-if-tracing-is-correct "Return Code Indicates if Tracing is Correct")
        Reference: [doc/requirements/DESIGN.md:118](../doc/requirements/DESIGN.md?plain=1#L118)
    *   [DSG_CTRL_TRACE](#dsg_ctrl_trace-trace-requirements "Trace Requirements")
        Reference: [doc/requirements/DESIGN.md:100](../doc/requirements/DESIGN.md?plain=1#L100)
    *   [DSG_JOBS](#dsg_jobs-jobs-encode-requested-behavior "Jobs encode requested behavior")
        Reference: [doc/requirements/DESIGN.md:50](../doc/requirements/DESIGN.md?plain=1#L50)

## ARCH_FORMATTER: Format output in requested Format

Origin: architecture, [doc/requirements/ARCHITECTURE.md:63](../doc/requirements/ARCHITECTURE.md?plain=1#L63)


The formatter takes Artefacts, the Graph, the Tracing or a list of Errors and
turns them into machine or human readable form.



Upwards Tracing:
*   requirements => [design, formats, architecture]
    *   [REQ_HUMAN_READABLE](#req_human_readable-human-readable-output "Human Readable Output")
        Reference: [doc/requirements/ARCHITECTURE.md:70](../doc/requirements/ARCHITECTURE.md?plain=1#L70)
    *   [REQ_MACHINE_READABLE](#req_machine_readable-machine-readable-output "Machine Readable Output")
        Reference: [doc/requirements/ARCHITECTURE.md:71](../doc/requirements/ARCHITECTURE.md?plain=1#L71)
    *   [UC_TMX](#uc_tmx-create-traceability-matrix "Create Traceability Matrix")
        Reference: [doc/requirements/ARCHITECTURE.md:69](../doc/requirements/ARCHITECTURE.md?plain=1#L69)

Downwards Tracing:
*   architecture => [design]
    *   [DSG_FORMATTER](#dsg_formatter-formatter "Formatter")
        Reference: [doc/requirements/DESIGN.md:300](../doc/requirements/DESIGN.md?plain=1#L300)

## ARCH_GRAPH: Graph

Origin: architecture, [doc/requirements/ARCHITECTURE.md:94](../doc/requirements/ARCHITECTURE.md?plain=1#L94)


The Graph organizes Artefacts into a directed graph:

*   either a single Artefact,
*   or a Relation.

*   point from an Artefact to a Relation or,
*   from a Relation to an artefact.



*   an Edge from Artefact U to Relation R
*   one or more Edges from Relation R to Artefacts D1, D2, ...
All requirements of U must be covered by at least one requirement one of the
artefacts D1, D2,  ....




Upwards Tracing:
*   requirements => [design, formats, architecture]
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
        Reference: [doc/requirements/ARCHITECTURE.md:118](../doc/requirements/ARCHITECTURE.md?plain=1#L118)

Downwards Tracing:
*   architecture => [design]
    *   [DSG_GRAPH](#dsg_graph-artefact-graph "Artefact Graph")
        Reference: [doc/requirements/DESIGN.md:377](../doc/requirements/DESIGN.md?plain=1#L377)
    *   [DSG_GRAPH_RELATION](#dsg_graph_relation-artefact-relationships "Artefact Relationships")
        Reference: [doc/requirements/DESIGN.md:386](../doc/requirements/DESIGN.md?plain=1#L386)

## ARCH_PARSER: Parser

Origin: architecture, [doc/requirements/ARCHITECTURE.md:45](../doc/requirements/ARCHITECTURE.md?plain=1#L45)


A Parser processes an input file and emits Requirements.
There are Parsers for several File formats, dependant on the configuration for an
input file.



Upwards Tracing:
*   requirements => [design, formats, architecture]
    *   [REQ_EXTENSIBLE](#req_extensible-extensible-parsing "Extensible Parsing")
        Reference: [doc/requirements/ARCHITECTURE.md:52](../doc/requirements/ARCHITECTURE.md?plain=1#L52)
    *   [UC_PARSE](#uc_parse-parse-artefacts "Parse Artefacts")
        Reference: [doc/requirements/ARCHITECTURE.md:53](../doc/requirements/ARCHITECTURE.md?plain=1#L53)

Downwards Tracing:
*   architecture => [design]
    *   [DSG_PARSER](#dsg_parser-parse-data "Parse Data")
        Reference: [doc/requirements/DESIGN.md:130](../doc/requirements/DESIGN.md?plain=1#L130)

## ARCH_REQUIREMENT: Requirement

Origin: architecture, [doc/requirements/ARCHITECTURE.md:80](../doc/requirements/ARCHITECTURE.md?plain=1#L80)


A Requirement is the basic unit of information that this tool operates on.
A Requirement object stores one typical software requirement (for example
"DSG_CLI_RETURN_CODE")

(for example: `main_rc`  which covers DSG_CLI_RETURN_CODE)



Upwards Tracing:
*   requirements => [design, formats, architecture]
    *   Derived

Downwards Tracing:
*   architecture => [design]
    *   [DSG_REQ_FIELDS](#dsg_req_fields-requirement-fields "Requirement Fields")
        Reference: [doc/requirements/DESIGN.md:339](../doc/requirements/DESIGN.md?plain=1#L339)

## ARCH_TRACE: Tracer

Origin: architecture, [doc/requirements/ARCHITECTURE.md:55](../doc/requirements/ARCHITECTURE.md?plain=1#L55)


The tracer walks the graph and calculates tracing information



Upwards Tracing:
*   requirements => [design, formats, architecture]
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
        Reference: [doc/requirements/ARCHITECTURE.md:60](../doc/requirements/ARCHITECTURE.md?plain=1#L60)
    *   [UC_TMX](#uc_tmx-create-traceability-matrix "Create Traceability Matrix")
        Reference: [doc/requirements/ARCHITECTURE.md:61](../doc/requirements/ARCHITECTURE.md?plain=1#L61)

Downwards Tracing:
*   architecture => [design]
    *   [DSG_TRACE](#dsg_trace-walk-the-graph-and-trace-requirements "Walk the Graph and trace requirements")
        Reference: [doc/requirements/DESIGN.md:147](../doc/requirements/DESIGN.md?plain=1#L147)
    *   [DSG_TRACE_CHECK_TITLE](#dsg_trace_check_title-when-tracing-upwards-or-downwards-match-title "When tracing upwards or downwards match title")
        Reference: [doc/requirements/DESIGN.md:257](../doc/requirements/DESIGN.md?plain=1#L257)
    *   [DSG_TRACE_COLLECT](#dsg_trace_collect-collect-requirements-from-artefact "Collect Requirements from Artefact")
        Reference: [doc/requirements/DESIGN.md:157](../doc/requirements/DESIGN.md?plain=1#L157)
    *   [DSG_TRACE_DERIVED](#dsg_trace_derived-record-requirements-that-do-not-cover-anything "Record requirements that do not cover anything")
        Reference: [doc/requirements/DESIGN.md:218](../doc/requirements/DESIGN.md?plain=1#L218)
    *   [DSG_TRACE_DOWNWARDS](#dsg_trace_downwards-trace-downwards-using-depends-attribute "Trace downwards using `depends` attribute")
        Reference: [doc/requirements/DESIGN.md:207](../doc/requirements/DESIGN.md?plain=1#L207)
    *   [DSG_TRACE_REFERENCE_EXIST](#dsg_trace_reference_exist-coverage-links-must-exist "Coverage Links must exist")
        Reference: [doc/requirements/DESIGN.md:272](../doc/requirements/DESIGN.md?plain=1#L272)
    *   [DSG_TRACE_RELATION](#dsg_trace_relation-trace-relation "Trace Relation")
        Reference: [doc/requirements/DESIGN.md:186](../doc/requirements/DESIGN.md?plain=1#L186)
    *   [DSG_TRACE_UNCOVERED](#dsg_trace_uncovered-record-requirements-that-are-not-completely-covered "Record requirements that are not completely covered")
        Reference: [doc/requirements/DESIGN.md:234](../doc/requirements/DESIGN.md?plain=1#L234)
    *   [DSG_TRACE_UPWARDS](#dsg_trace_upwards-trace-upwards-using-covers-attribute "Trace upwards using `covers` attribute")
        Reference: [doc/requirements/DESIGN.md:197](../doc/requirements/DESIGN.md?plain=1#L197)
    *   [DSG_TRACE_VALIDATE_EDGE](#dsg_trace_validate_edge-validate-edge-is-used-at-least-once "Validate Edge is used at least once")
        Reference: [doc/requirements/DESIGN.md:283](../doc/requirements/DESIGN.md?plain=1#L283)

## ARCH_TRACED_GRAPH: Tracing Information of Grpah

Origin: architecture, [doc/requirements/ARCHITECTURE.md:120](../doc/requirements/ARCHITECTURE.md?plain=1#L120)


The Traced Graph holds all the information of the Graph, along with Tracing
Information.



Upwards Tracing:
*   requirements => [design, formats, architecture]
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
        Reference: [doc/requirements/ARCHITECTURE.md:126](../doc/requirements/ARCHITECTURE.md?plain=1#L126)

Downwards Tracing:
*   architecture => [design]
    *   [DSG_TRACED_GRAPH](#dsg_traced_graph-tracing-information-of-grpah "Tracing Information of Grpah")
        Reference: [doc/requirements/DESIGN.md:403](../doc/requirements/DESIGN.md?plain=1#L403)
## code

## Controller::run

Origin: code, [src/controller.rs:258:11](../src/controller.rs?plain=1#L258)

Upwards Tracing:
*   design => [formats, code]
    *   [DSG_CTRL_FORMAT](#dsg_ctrl_format-format-job-output "Format Job Output")
        Reference: [src/controller.rs:291:16](../src/controller.rs?plain=1#L291)
    *   [DSG_CTRL_FORMAT](#dsg_ctrl_format-format-job-output "Format Job Output")
        Reference: [src/controller.rs:305:16](../src/controller.rs?plain=1#L305)
    *   [DSG_CTRL_GRAPH](#dsg_ctrl_graph-construct-graph "Construct Graph")
        Reference: [src/controller.rs:296:16](../src/controller.rs?plain=1#L296)
    *   [DSG_CTRL_PARSE](#dsg_ctrl_parse-parse-all-artefacts "Parse all Artefacts")
        Reference: [src/controller.rs:295:16](../src/controller.rs?plain=1#L295)
    *   [DSG_CTRL_RETURN_CODE](#dsg_ctrl_return_code-return-code-indicates-if-tracing-is-correct "Return Code Indicates if Tracing is Correct")
        Reference: [src/controller.rs:320:8](../src/controller.rs?plain=1#L320)
    *   [DSG_CTRL_TRACE](#dsg_ctrl_trace-trace-requirements "Trace Requirements")
        Reference: [src/controller.rs:282:16](../src/controller.rs?plain=1#L282)
*   formats => [code]

Downwards Tracing:

## Controller::run_jobs

Origin: code, [src/controller.rs:229:11](../src/controller.rs?plain=1#L229)

Upwards Tracing:
*   design => [formats, code]
    *   [DSG_CTRL_RETURN_CODE](#dsg_ctrl_return_code-return-code-indicates-if-tracing-is-correct "Return Code Indicates if Tracing is Correct")
        Reference: [src/controller.rs:241:24](../src/controller.rs?plain=1#L241)
*   formats => [code]

Downwards Tracing:

## Parser < '_ >::parse_macro

Origin: code, [src/parsers/rust.rs:69:7](../src/parsers/rust.rs?plain=1#L69)

Upwards Tracing:
*   design => [formats, code]
*   formats => [code]
    *   [FMT_IMPORT_RUS_COV_MARK](#fmt_import_rus_cov_mark-rust-coverage-marks "Rust Coverage Marks")
        Reference: [src/parsers/rust.rs:70:8](../src/parsers/rust.rs?plain=1#L70)

Downwards Tracing:

## Tracer::add_artefact

Origin: code, [src/trace.rs:102:7](../src/trace.rs?plain=1#L102)

Upwards Tracing:
*   design => [formats, code]
    *   [DSG_TRACE_DETECT_DUPLICATE](#dsg_trace_detect_duplicate-detect-duplicate-requirements-in-different-artefacts "Detect duplicate Requirements in different Artefacts")
        Reference: [src/trace.rs:111:20](../src/trace.rs?plain=1#L111)
    *   [DSG_TRACE_REFERENCE_EXIST](#dsg_trace_reference_exist-coverage-links-must-exist "Coverage Links must exist")
        Reference: [src/trace.rs:121:24](../src/trace.rs?plain=1#L121)
    *   [DSG_TRACE_REFERENCE_EXIST](#dsg_trace_reference_exist-coverage-links-must-exist "Coverage Links must exist")
        Reference: [src/trace.rs:126:24](../src/trace.rs?plain=1#L126)
*   formats => [code]

Downwards Tracing:

## Tracer::trace_relation

Origin: code, [src/trace.rs:160:7](../src/trace.rs?plain=1#L160)

Upwards Tracing:
*   design => [formats, code]
    *   [DSG_TRACE_DOWNWARDS](#dsg_trace_downwards-trace-downwards-using-depends-attribute "Trace downwards using `depends` attribute")
        Reference: [src/trace.rs:172:16](../src/trace.rs?plain=1#L172)
    *   [DSG_TRACE_REFERENCE_EXIST](#dsg_trace_reference_exist-coverage-links-must-exist "Coverage Links must exist")
        Reference: [src/trace.rs:207:24](../src/trace.rs?plain=1#L207)
    *   [DSG_TRACE_REFERENCE_EXIST](#dsg_trace_reference_exist-coverage-links-must-exist "Coverage Links must exist")
        Reference: [src/trace.rs:249:24](../src/trace.rs?plain=1#L249)
    *   [DSG_TRACE_UPWARDS](#dsg_trace_upwards-trace-upwards-using-covers-attribute "Trace upwards using `covers` attribute")
        Reference: [src/trace.rs:223:24](../src/trace.rs?plain=1#L223)
*   formats => [code]

Downwards Tracing:

## Tracer::validate_downwards

Origin: code, [src/trace.rs:295:7](../src/trace.rs?plain=1#L295)

Upwards Tracing:
*   design => [formats, code]
    *   [DSG_TRACE_REFERENCE_EXIST](#dsg_trace_reference_exist-coverage-links-must-exist "Coverage Links must exist")
        Reference: [src/trace.rs:297:12](../src/trace.rs?plain=1#L297)
*   formats => [code]

Downwards Tracing:

## Tracer::validate_upwards

Origin: code, [src/trace.rs:275:7](../src/trace.rs?plain=1#L275)

Upwards Tracing:
*   design => [formats, code]
    *   [DSG_TRACE_REFERENCE_EXIST](#dsg_trace_reference_exist-coverage-links-must-exist "Coverage Links must exist")
        Reference: [src/trace.rs:277:12](../src/trace.rs?plain=1#L277)
*   formats => [code]

Downwards Tracing:

## get_config

Origin: code, [src/main.rs:77:3](../src/main.rs?plain=1#L77)

Upwards Tracing:
*   design => [formats, code]
    *   [DSG_CTRL_CONFIG](#dsg_ctrl_config-single-config-file "Single Config File")
        Reference: [src/main.rs:78:4](../src/main.rs?plain=1#L78)
*   formats => [code]
    *   [FMT_CONFIG_TOML](#fmt_config_toml-use-a-single-toml-file-as-configuration "Use a Single TOML File as Configuration")
        Reference: [src/main.rs:98:4](../src/main.rs?plain=1#L98)

Downwards Tracing:

## main_rc

Origin: code, [src/main.rs:137:3](../src/main.rs?plain=1#L137)

Upwards Tracing:
*   design => [formats, code]
    *   [DSG_CLI_RETURN_CODE](#dsg_cli_return_code-set-return-code-to-indicate-success "Set return Code to indicate success")
        Reference: [src/main.rs:140:4](../src/main.rs?plain=1#L140)
*   formats => [code]

Downwards Tracing:

## parse

Origin: code, [src/parsers/monoreq.rs:23:7](../src/parsers/monoreq.rs?plain=1#L23)

Upwards Tracing:
*   design => [formats, code]
*   formats => [code]
    *   [FMT_IMPORT_MONO_REQ](#fmt_import_mono_req-single-requirement-per-file "Single Requirement Per File")
        Reference: [src/parsers/monoreq.rs:28:4](../src/parsers/monoreq.rs?plain=1#L28)

Downwards Tracing:

## parse_from_config

Origin: code, [src/controller.rs:128:7](../src/controller.rs?plain=1#L128)

Upwards Tracing:
*   design => [formats, code]
    *   [DSG_ART_PARSE_COLLECT_ERRORS](#dsg_art_parse_collect_errors-collect-errors-but-keep-parsing "Collect errors but keep parsing")
        Reference: [src/controller.rs:135:4](../src/controller.rs?plain=1#L135)
    *   [DSG_CTRL_DETECT_DUPLICATE_REQS](#dsg_ctrl_detect_duplicate_reqs-detect-duplicates "Detect duplicates")
        Reference: [src/controller.rs:146:16](../src/controller.rs?plain=1#L146)
*   formats => [code]

Downwards Tracing:

## parse_multiple_files

Origin: code, [src/controller.rs:84:3](../src/controller.rs?plain=1#L84)

Upwards Tracing:
*   design => [formats, code]
    *   [DSG_ART_FILES](#dsg_art_files-artefact-loads-one-or-more-files "Artefact loads one or more Files")
        Reference: [src/controller.rs:108:16](../src/controller.rs?plain=1#L108)
*   formats => [code]

Downwards Tracing:

## parse_single_file

Origin: code, [src/controller.rs:51:3](../src/controller.rs?plain=1#L51)

Upwards Tracing:
*   design => [formats, code]
    *   [DSG_ART_FILES](#dsg_art_files-artefact-loads-one-or-more-files "Artefact loads one or more Files")
        Reference: [src/controller.rs:64:4](../src/controller.rs?plain=1#L64)
*   formats => [code]

Downwards Tracing:

## requirements

Origin: code, [src/formatters/mod.rs:29:7](../src/formatters/mod.rs?plain=1#L29)

Upwards Tracing:
*   design => [formats, code]
    *   [DSG_FORMATTER](#dsg_formatter-formatter "Formatter")
        Reference: [src/formatters/mod.rs:30:4](../src/formatters/mod.rs?plain=1#L30)
*   formats => [code]

Downwards Tracing:

## run_cli_jobs

Origin: code, [src/main.rs:114:3](../src/main.rs?plain=1#L114)

Upwards Tracing:
*   design => [formats, code]
    *   [DSG_CLI](#dsg_cli-command-line-interface "Command Line Interface")
        Reference: [src/main.rs:124:4](../src/main.rs?plain=1#L124)
    *   [DSG_JOBS](#dsg_jobs-jobs-encode-requested-behavior "Jobs encode requested behavior")
        Reference: [src/main.rs:121:8](../src/main.rs?plain=1#L121)
*   formats => [code]

Downwards Tracing:

## tracing

Origin: code, [src/formatters/mod.rs:54:7](../src/formatters/mod.rs?plain=1#L54)

Upwards Tracing:
*   design => [formats, code]
    *   [DSG_FORMATTER](#dsg_formatter-formatter "Formatter")
        Reference: [src/formatters/mod.rs:59:4](../src/formatters/mod.rs?plain=1#L59)
*   formats => [code]

Downwards Tracing:
## design

## DSG_AGGREGATOR: Cross Referenced Trace Graph

Origin: design, [doc/requirements/DESIGN.md:302](../doc/requirements/DESIGN.md?plain=1#L302)


The aggregator creates lookup tables, where all tracing information is indexed by either
requirement id or artefact id, so formatter does not have to crawl through relations when
trying to write coverage information for a requirement.





Upwards Tracing:
*   architecture => [design]
*   requirements => [design, formats, architecture]
    *   Derived

Downwards Tracing:
*   design => [formats, code]
    *    UNCOVERED

## DSG_ART_FILES: Artefact loads one or more Files

Origin: design, [doc/requirements/DESIGN.md:348](../doc/requirements/DESIGN.md?plain=1#L348)


An Artefact represents one or more files of the same type.



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_ARTEFACT](#arch_artefact-artefact "Artefact")
        Reference: [doc/requirements/DESIGN.md:353](../doc/requirements/DESIGN.md?plain=1#L353)
*   requirements => [design, formats, architecture]

Downwards Tracing:
*   design => [formats, code]
    *   [parse_multiple_files](#parse_multiple_files)
        Reference: [src/controller.rs:108:16](../src/controller.rs?plain=1#L108)
    *   [parse_single_file](#parse_single_file)
        Reference: [src/controller.rs:64:4](../src/controller.rs?plain=1#L64)

## DSG_ART_IGNORE_DERIVED: Ignore Derived Requirements

Origin: design, [doc/requirements/DESIGN.md:364](../doc/requirements/DESIGN.md?plain=1#L364)


Artefacts can be configured to ignore derived requirements



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_ARTEFACT](#arch_artefact-artefact "Artefact")
        Reference: [doc/requirements/DESIGN.md:369](../doc/requirements/DESIGN.md?plain=1#L369)
*   requirements => [design, formats, architecture]

Downwards Tracing:
*   design => [formats, code]
    *    UNCOVERED

## DSG_ART_PARSE_COLLECT_ERRORS: Collect errors but keep parsing

Origin: design, [doc/requirements/DESIGN.md:355](../doc/requirements/DESIGN.md?plain=1#L355)


While parsing artefacts, all encountered errors are stored and parsing continues.



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_ARTEFACT](#arch_artefact-artefact "Artefact")
        Reference: [doc/requirements/DESIGN.md:361](../doc/requirements/DESIGN.md?plain=1#L361)
*   requirements => [design, formats, architecture]
    *   [REQ_LATE_ERROR](#req_late_error-collect-errors-but-continue-processing "Collect Errors but continue processing")
        Reference: [doc/requirements/DESIGN.md:359](../doc/requirements/DESIGN.md?plain=1#L359)
    *   [REQ_PARSER_ERROR](#req_parser_error-useful-parser-errors "Useful Parser Errors")
        Reference: [doc/requirements/DESIGN.md:360](../doc/requirements/DESIGN.md?plain=1#L360)

Downwards Tracing:
*   design => [formats, code]
    *   [parse_from_config](#parse_from_config)
        Reference: [src/controller.rs:135:4](../src/controller.rs?plain=1#L135)

## DSG_CLI: Command Line Interface

Origin: design, [doc/requirements/DESIGN.md:9](../doc/requirements/DESIGN.md?plain=1#L9)


The tool should be invoked via a simple CLI and set the exit code to indicate if a job was
successful.



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_CLI](#arch_cli-command-line-interface "Command Line Interface")
        Reference: [doc/requirements/DESIGN.md:16](../doc/requirements/DESIGN.md?plain=1#L16)
*   requirements => [design, formats, architecture]
    *   [REQ_MACHINE_FRIENDLY](#req_machine_friendly-easy-to-include-in-automated-work-flows "Easy to include in automated work flows")
        Reference: [doc/requirements/DESIGN.md:15](../doc/requirements/DESIGN.md?plain=1#L15)

Downwards Tracing:
*   design => [formats, code]
    *   [run_cli_jobs](#run_cli_jobs)
        Reference: [src/main.rs:124:4](../src/main.rs?plain=1#L124)

## DSG_CLI_RETURN_CODE: Set return Code to indicate success

Origin: design, [doc/requirements/DESIGN.md:18](../doc/requirements/DESIGN.md?plain=1#L18)


Set the process' return code to:
*   `2` if there were fatal errors (invalid configuration or similar)
*   `1` if there were errors (file not found, parser errors, uncovered requirement, derived
    requirement, ...)
*   `0` otherwise



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_CLI](#arch_cli-command-line-interface "Command Line Interface")
        Reference: [doc/requirements/DESIGN.md:29](../doc/requirements/DESIGN.md?plain=1#L29)
*   requirements => [design, formats, architecture]
    *   [REQ_MACHINE_FRIENDLY](#req_machine_friendly-easy-to-include-in-automated-work-flows "Easy to include in automated work flows")
        Reference: [doc/requirements/DESIGN.md:27](../doc/requirements/DESIGN.md?plain=1#L27)
    *   [UC_CHECK](#uc_check-check-for-correct-tracing "Check for correct Tracing")
        Reference: [doc/requirements/DESIGN.md:28](../doc/requirements/DESIGN.md?plain=1#L28)

Downwards Tracing:
*   design => [formats, code]
    *   [main_rc](#main_rc)
        Reference: [src/main.rs:140:4](../src/main.rs?plain=1#L140)

## DSG_CTRL_CONFIG: Single Config File

Origin: design, [doc/requirements/DESIGN.md:54](../doc/requirements/DESIGN.md?plain=1#L54)


The Controller the following information from a single config file:

*   Relation between Artefacts
*   Jobs



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_CONTROLLER](#arch_controller-controller "Controller")
        Reference: [doc/requirements/DESIGN.md:64](../doc/requirements/DESIGN.md?plain=1#L64)
*   requirements => [design, formats, architecture]
    *   [REQ_CONFIG](#req_config-simple-configuration-in-one-file "Simple Configuration in One File")
        Reference: [doc/requirements/DESIGN.md:63](../doc/requirements/DESIGN.md?plain=1#L63)

Downwards Tracing:
*   design => [formats, code]
    *   [get_config](#get_config)
        Reference: [src/main.rs:78:4](../src/main.rs?plain=1#L78)

## DSG_CTRL_DETECT_DUPLICATE_REQS: Detect duplicates

Origin: design, [doc/requirements/DESIGN.md:76](../doc/requirements/DESIGN.md?plain=1#L76)


After Parsing, when assembling Requirements, detect duplicate requirements in the same Artefact



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_ARTEFACT](#arch_artefact-artefact "Artefact")
        Reference: [doc/requirements/DESIGN.md:82](../doc/requirements/DESIGN.md?plain=1#L82)
*   requirements => [design, formats, architecture]
    *   [REQ_UNIQUE_ID_v2](#req_unique_id_v2-requirements-have-a-unique-identifier "Requirements have a unique Identifier")
        Reference: [doc/requirements/DESIGN.md:81](../doc/requirements/DESIGN.md?plain=1#L81)

Downwards Tracing:
*   design => [formats, code]
    *   [parse_from_config](#parse_from_config)
        Reference: [src/controller.rs:146:16](../src/controller.rs?plain=1#L146)

## DSG_CTRL_FORMAT: Format Job Output

Origin: design, [doc/requirements/DESIGN.md:104](../doc/requirements/DESIGN.md?plain=1#L104)


Pass the output of running Job.Query to the Formatter specified by Job.Format and Write to
Job.File.



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_CONTROLLER](#arch_controller-controller "Controller")
        Reference: [doc/requirements/DESIGN.md:110](../doc/requirements/DESIGN.md?plain=1#L110)
*   requirements => [design, formats, architecture]

Downwards Tracing:
*   design => [formats, code]
    *   [Controller::run](#controllerrun)
        Reference: [src/controller.rs:291:16](../src/controller.rs?plain=1#L291)
    *   [Controller::run](#controllerrun)
        Reference: [src/controller.rs:305:16](../src/controller.rs?plain=1#L305)

## DSG_CTRL_GRAPH: Construct Graph

Origin: design, [doc/requirements/DESIGN.md:85](../doc/requirements/DESIGN.md?plain=1#L85)


If the Job.Query is Parse or Trace:
After Parsing, Assemble the artefacts in a Graph, which contains all artefacts,
the relations between artefacts, and graph configuration errors.



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_CONTROLLER](#arch_controller-controller "Controller")
        Reference: [doc/requirements/DESIGN.md:92](../doc/requirements/DESIGN.md?plain=1#L92)
*   requirements => [design, formats, architecture]

Downwards Tracing:
*   design => [formats, code]
    *   [Controller::run](#controllerrun)
        Reference: [src/controller.rs:296:16](../src/controller.rs?plain=1#L296)

## DSG_CTRL_PARSE: Parse all Artefacts

Origin: design, [doc/requirements/DESIGN.md:66](../doc/requirements/DESIGN.md?plain=1#L66)


If the Job.Query is Parse or Trace:
For all Artefacts, parse the content of all files of the artefact to the correct parser.
Collect all requirement, any encountered errors and any inspected files in the Artefact.



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_CONTROLLER](#arch_controller-controller "Controller")
        Reference: [doc/requirements/DESIGN.md:73](../doc/requirements/DESIGN.md?plain=1#L73)
*   requirements => [design, formats, architecture]
    *   [UC_PARSE](#uc_parse-parse-artefacts "Parse Artefacts")
        Reference: [doc/requirements/DESIGN.md:74](../doc/requirements/DESIGN.md?plain=1#L74)

Downwards Tracing:
*   design => [formats, code]
    *   [Controller::run](#controllerrun)
        Reference: [src/controller.rs:295:16](../src/controller.rs?plain=1#L295)

## DSG_CTRL_RETURN_CODE: Return Code Indicates if Tracing is Correct

Origin: design, [doc/requirements/DESIGN.md:112](../doc/requirements/DESIGN.md?plain=1#L112)


If Job.SetReturnCode, and running job.query produced any errors, indicate them to the CLI,
so it can set an error return code.



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_CONTROLLER](#arch_controller-controller "Controller")
        Reference: [doc/requirements/DESIGN.md:118](../doc/requirements/DESIGN.md?plain=1#L118)
*   requirements => [design, formats, architecture]

Downwards Tracing:
*   design => [formats, code]
    *   [Controller::run](#controllerrun)
        Reference: [src/controller.rs:320:8](../src/controller.rs?plain=1#L320)
    *   [Controller::run_jobs](#controllerrun_jobs)
        Reference: [src/controller.rs:241:24](../src/controller.rs?plain=1#L241)

## DSG_CTRL_TRACE: Trace Requirements

Origin: design, [doc/requirements/DESIGN.md:94](../doc/requirements/DESIGN.md?plain=1#L94)


If the Job.Query is Trace, pass the Graph to the Tracer to get a Traced Graph.




Upwards Tracing:
*   architecture => [design]
    *   [ARCH_CONTROLLER](#arch_controller-controller "Controller")
        Reference: [doc/requirements/DESIGN.md:100](../doc/requirements/DESIGN.md?plain=1#L100)
*   requirements => [design, formats, architecture]
    *   [UC_CHECK](#uc_check-check-for-correct-tracing "Check for correct Tracing")
        Reference: [doc/requirements/DESIGN.md:102](../doc/requirements/DESIGN.md?plain=1#L102)
    *   [UC_TMX](#uc_tmx-create-traceability-matrix "Create Traceability Matrix")
        Reference: [doc/requirements/DESIGN.md:101](../doc/requirements/DESIGN.md?plain=1#L101)

Downwards Tracing:
*   design => [formats, code]
    *   [Controller::run](#controllerrun)
        Reference: [src/controller.rs:282:16](../src/controller.rs?plain=1#L282)

## DSG_FORMATTER: Formatter

Origin: design, [doc/requirements/DESIGN.md:289](../doc/requirements/DESIGN.md?plain=1#L289)


The formatter is called with
*   either a Graph, or a Traced Graph
*   a Format
and formats the given information according to format.

Graph to the Aggregator, and the formats an aggregated Graph.



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_FORMATTER](#arch_formatter-format-output-in-requested-format "Format output in requested Format")
        Reference: [doc/requirements/DESIGN.md:300](../doc/requirements/DESIGN.md?plain=1#L300)
*   requirements => [design, formats, architecture]

Downwards Tracing:
*   design => [formats, code]
    *   [requirements](#requirements)
        Reference: [src/formatters/mod.rs:30:4](../src/formatters/mod.rs?plain=1#L30)
    *   [tracing](#tracing)
        Reference: [src/formatters/mod.rs:59:4](../src/formatters/mod.rs?plain=1#L59)

## DSG_GRAPH: Artefact Graph

Origin: design, [doc/requirements/DESIGN.md:371](../doc/requirements/DESIGN.md?plain=1#L371)


The Graph holds all Artefacts, and a list of relations describing which artefact covers
which



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_GRAPH](#arch_graph-graph "Graph")
        Reference: [doc/requirements/DESIGN.md:377](../doc/requirements/DESIGN.md?plain=1#L377)
*   requirements => [design, formats, architecture]

Downwards Tracing:
*   design => [formats, code]
    *    UNCOVERED

## DSG_GRAPH_RELATION: Artefact Relationships

Origin: design, [doc/requirements/DESIGN.md:379](../doc/requirements/DESIGN.md?plain=1#L379)


A Relation has fields:
*   Upper Requirement ID
*   List of Lower Requirement IDs



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_GRAPH](#arch_graph-graph "Graph")
        Reference: [doc/requirements/DESIGN.md:386](../doc/requirements/DESIGN.md?plain=1#L386)
*   requirements => [design, formats, architecture]

Downwards Tracing:
*   design => [formats, code]
    *    UNCOVERED

## DSG_JOBS: Jobs encode requested behavior

Origin: design, [doc/requirements/DESIGN.md:39](../doc/requirements/DESIGN.md?plain=1#L39)


Jobs configure what work should be done, and in what format the results should be
presented. a Job has the following Fields:
*   Query: What to do
*   Format: How results are Presented
*   File: Where results should be stored (`-` acts as stdout)
*   SetReturncode: Wether errors encountered by this query affect the CLI Process's return
    code.



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_CONTROLLER](#arch_controller-controller "Controller")
        Reference: [doc/requirements/DESIGN.md:50](../doc/requirements/DESIGN.md?plain=1#L50)
*   requirements => [design, formats, architecture]
    *   [REQ_CONFIG](#req_config-simple-configuration-in-one-file "Simple Configuration in One File")
        Reference: [doc/requirements/DESIGN.md:51](../doc/requirements/DESIGN.md?plain=1#L51)

Downwards Tracing:
*   design => [formats, code]
    *   [run_cli_jobs](#run_cli_jobs)
        Reference: [src/main.rs:121:8](../src/main.rs?plain=1#L121)

## DSG_PARSER: Parse Data

Origin: design, [doc/requirements/DESIGN.md:122](../doc/requirements/DESIGN.md?plain=1#L122)


The Parser is called with data from one file, and the format of that data.
It returns Lists with:
*   All found Requirements
*   All encountered Errors



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_PARSER](#arch_parser-parser "Parser")
        Reference: [doc/requirements/DESIGN.md:130](../doc/requirements/DESIGN.md?plain=1#L130)
*   requirements => [design, formats, architecture]

Downwards Tracing:
*   design => [formats, code]
    *    UNCOVERED

## DSG_REQ_FIELDS: Requirement Fields

Origin: design, [doc/requirements/DESIGN.md:314](../doc/requirements/DESIGN.md?plain=1#L314)


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



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_REQUIREMENT](#arch_requirement-requirement "Requirement")
        Reference: [doc/requirements/DESIGN.md:339](../doc/requirements/DESIGN.md?plain=1#L339)
*   requirements => [design, formats, architecture]
    *   [REQ_DOWN](#req_down-downward-coverage "Downward Coverage")
        Reference: [doc/requirements/DESIGN.md:338](../doc/requirements/DESIGN.md?plain=1#L338)
    *   [REQ_UP](#req_up-upward-coverage "Upward Coverage")
        Reference: [doc/requirements/DESIGN.md:337](../doc/requirements/DESIGN.md?plain=1#L337)

Downwards Tracing:
*   design => [formats, code]
    *    UNCOVERED

## DSG_TRACE: Walk the Graph and trace requirements

Origin: design, [doc/requirements/DESIGN.md:134](../doc/requirements/DESIGN.md?plain=1#L134)


The Tracer is passed a Graph. The Tracer inspects all relations of the graph,
recording tracing information as it is encountered.
After all Relations are processed, a final validation pass uncovers any unresolved
problems and records Errors for them.

1.  Collect Requirements from all Artefacts
2.  Trace all Relations
3.  Validate



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")
        Reference: [doc/requirements/DESIGN.md:147](../doc/requirements/DESIGN.md?plain=1#L147)
*   requirements => [design, formats, architecture]

Downwards Tracing:
*   design => [formats, code]
    *    UNCOVERED

## DSG_TRACED_GRAPH: Tracing Information of Grpah

Origin: design, [doc/requirements/DESIGN.md:388](../doc/requirements/DESIGN.md?plain=1#L388)


The Traced Graph has Fields:
*   Artefacts, indexed by artefact id
*   Lists of Derived Requirements, indexed by artefact id
*   List of Traced Relations
    *   Relations
    *   list of Coverages
        *   Upper Requirement
        *   Lower Requirement
        *   Location of the Reference (the Covers Line)
    *   List of derived Requirements
    *   List of Tracing Errors



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_TRACED_GRAPH](#arch_traced_graph-tracing-information-of-grpah "Tracing Information of Grpah")
        Reference: [doc/requirements/DESIGN.md:403](../doc/requirements/DESIGN.md?plain=1#L403)
*   requirements => [design, formats, architecture]

Downwards Tracing:
*   design => [formats, code]
    *    UNCOVERED

## DSG_TRACE_CHECK_TITLE: When tracing upwards or downwards match title

Origin: design, [doc/requirements/DESIGN.md:236](../doc/requirements/DESIGN.md?plain=1#L236)


When tracing Upwards or Downwards, emit an error if the title of the coverage does
not match the title of the covered requirement



Example:
### REQ_U: Title of Upper

    An Upper Requirement

    ### REQ_D: Title of Lower

    A Lower Requirement that covers REQ_U with an exactly mathcing title.

    Covers:
    *   REQ_U: Title of Upper

Upwards Tracing:
*   architecture => [design]
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")
        Reference: [doc/requirements/DESIGN.md:257](../doc/requirements/DESIGN.md?plain=1#L257)
*   requirements => [design, formats, architecture]
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
        Reference: [doc/requirements/DESIGN.md:255](../doc/requirements/DESIGN.md?plain=1#L255)
    *   [REQ_VAL_TITLE](#req_val_title-check-matching-title "Check matching title")
        Reference: [doc/requirements/DESIGN.md:256](../doc/requirements/DESIGN.md?plain=1#L256)

Downwards Tracing:
*   design => [formats, code]
    *    UNCOVERED

## DSG_TRACE_COLLECT: Collect Requirements from Artefact

Origin: design, [doc/requirements/DESIGN.md:149](../doc/requirements/DESIGN.md?plain=1#L149)


All requirements from all artefacts are added to  index.
Each requirement is also added to a list of derived requirements of its artefact
All Covers and Depends references of each requirement are added to a list of illegal
references, from which they will be remove if they traced validly.



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")
        Reference: [doc/requirements/DESIGN.md:157](../doc/requirements/DESIGN.md?plain=1#L157)
*   requirements => [design, formats, architecture]

Downwards Tracing:
*   design => [formats, code]
    *    UNCOVERED

## DSG_TRACE_DERIVED: Record requirements that do not cover anything

Origin: design, [doc/requirements/DESIGN.md:209](../doc/requirements/DESIGN.md?plain=1#L209)


When Tracing, All requirements are first added to their artefacts' deerived list.
Whenever a requirement R covers an upper requirement U, R is removed from it's artefact's
derived list.
All remaing requirements are derived.



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")
        Reference: [doc/requirements/DESIGN.md:218](../doc/requirements/DESIGN.md?plain=1#L218)
*   requirements => [design, formats, architecture]
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
        Reference: [doc/requirements/DESIGN.md:217](../doc/requirements/DESIGN.md?plain=1#L217)

Downwards Tracing:
*   design => [formats, code]
    *    UNCOVERED

## DSG_TRACE_DETECT_DUPLICATE: Detect duplicate Requirements in different Artefacts

Origin: design, [doc/requirements/DESIGN.md:159](../doc/requirements/DESIGN.md?plain=1#L159)


While collecting requirements, if there are two requirements the same identifier, log an error.




Upwards Tracing:
*   architecture => [design]
*   requirements => [design, formats, architecture]
    *   [REQ_UNIQUE_ID_v2](#req_unique_id_v2-requirements-have-a-unique-identifier "Requirements have a unique Identifier")
        Reference: [doc/requirements/DESIGN.md:166](../doc/requirements/DESIGN.md?plain=1#L166)

Downwards Tracing:
*   design => [formats, code]
    *   [Tracer::add_artefact](#traceradd_artefact)
        Reference: [src/trace.rs:111:20](../src/trace.rs?plain=1#L111)

## DSG_TRACE_DOWNWARDS: Trace downwards using `depends` attribute

Origin: design, [doc/requirements/DESIGN.md:199](../doc/requirements/DESIGN.md?plain=1#L199)


Requirement U covers Requirement D if D.id appears in U.Depends.



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")
        Reference: [doc/requirements/DESIGN.md:207](../doc/requirements/DESIGN.md?plain=1#L207)
*   requirements => [design, formats, architecture]
    *   [REQ_DOWN](#req_down-downward-coverage "Downward Coverage")
        Reference: [doc/requirements/DESIGN.md:206](../doc/requirements/DESIGN.md?plain=1#L206)
    *   [REQ_MATCH_ID](#req_match_id-match-by-id "Match by ID")
        Reference: [doc/requirements/DESIGN.md:205](../doc/requirements/DESIGN.md?plain=1#L205)
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
        Reference: [doc/requirements/DESIGN.md:204](../doc/requirements/DESIGN.md?plain=1#L204)

Downwards Tracing:
*   design => [formats, code]
    *   [Tracer::trace_relation](#tracertrace_relation)
        Reference: [src/trace.rs:172:16](../src/trace.rs?plain=1#L172)

## DSG_TRACE_REFERENCE_EXIST: Coverage Links must exist

Origin: design, [doc/requirements/DESIGN.md:259](../doc/requirements/DESIGN.md?plain=1#L259)


For each Requirement that is encountered, store all "covers" and "depends" references
in a list of invalid references.
When the Requirement is successfully covered against a requirement matching
that reference, it is removed from the list of invalid references.




Upwards Tracing:
*   architecture => [design]
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")
        Reference: [doc/requirements/DESIGN.md:272](../doc/requirements/DESIGN.md?plain=1#L272)
*   requirements => [design, formats, architecture]
    *   [REQ_DOWN](#req_down-downward-coverage "Downward Coverage")
        Reference: [doc/requirements/DESIGN.md:270](../doc/requirements/DESIGN.md?plain=1#L270)
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
        Reference: [doc/requirements/DESIGN.md:269](../doc/requirements/DESIGN.md?plain=1#L269)
    *   [REQ_UP](#req_up-upward-coverage "Upward Coverage")
        Reference: [doc/requirements/DESIGN.md:271](../doc/requirements/DESIGN.md?plain=1#L271)
    *   [REQ_VAL_COVERAGE](#req_val_coverage-validate-coverage "Validate Coverage")
        Reference: [doc/requirements/DESIGN.md:273](../doc/requirements/DESIGN.md?plain=1#L273)

Downwards Tracing:
*   design => [formats, code]
    *   [Tracer::add_artefact](#traceradd_artefact)
        Reference: [src/trace.rs:121:24](../src/trace.rs?plain=1#L121)
    *   [Tracer::add_artefact](#traceradd_artefact)
        Reference: [src/trace.rs:126:24](../src/trace.rs?plain=1#L126)
    *   [Tracer::trace_relation](#tracertrace_relation)
        Reference: [src/trace.rs:207:24](../src/trace.rs?plain=1#L207)
    *   [Tracer::trace_relation](#tracertrace_relation)
        Reference: [src/trace.rs:249:24](../src/trace.rs?plain=1#L249)
    *   [Tracer::validate_downwards](#tracervalidate_downwards)
        Reference: [src/trace.rs:297:12](../src/trace.rs?plain=1#L297)
    *   [Tracer::validate_upwards](#tracervalidate_upwards)
        Reference: [src/trace.rs:277:12](../src/trace.rs?plain=1#L277)

## DSG_TRACE_RELATION: Trace Relation

Origin: design, [doc/requirements/DESIGN.md:169](../doc/requirements/DESIGN.md?plain=1#L169)


For each Relation, inspect the requirements of the upper artefact and perform the
following steps:

2.  for all lower artefacts, find lower requirements that cover upper
3.  for all coverages found:
    1.  remove them from the list of invalid references
    2.  remove lower from the list of derived requirements
    3.  Record the coverage information with the relation
    4.  if covered with title, add an error if the title is not matched
    correctly
4.  if no coverage was found, add the requirement to the list of uncovered
    requirements of the relation



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")
        Reference: [doc/requirements/DESIGN.md:186](../doc/requirements/DESIGN.md?plain=1#L186)
*   requirements => [design, formats, architecture]

Downwards Tracing:
*   design => [formats, code]
    *    UNCOVERED

## DSG_TRACE_UNCOVERED: Record requirements that are not completely covered

Origin: design, [doc/requirements/DESIGN.md:220](../doc/requirements/DESIGN.md?plain=1#L220)


When tracing a Requirement R along a Relation, if neither downward nor upward coverage is
found, that requirement is added to the list of uncovered requirements along that
relation.


`design => [unittests]` and by either code or a format specification `design => [format,
code]`.



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")
        Reference: [doc/requirements/DESIGN.md:234](../doc/requirements/DESIGN.md?plain=1#L234)
*   requirements => [design, formats, architecture]
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
        Reference: [doc/requirements/DESIGN.md:233](../doc/requirements/DESIGN.md?plain=1#L233)

Downwards Tracing:
*   design => [formats, code]
    *    UNCOVERED

## DSG_TRACE_UPWARDS: Trace upwards using `covers` attribute

Origin: design, [doc/requirements/DESIGN.md:189](../doc/requirements/DESIGN.md?plain=1#L189)


Requirement U covers Requirement D if U.id appears in D.Covers.



Upwards Tracing:
*   architecture => [design]
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")
        Reference: [doc/requirements/DESIGN.md:197](../doc/requirements/DESIGN.md?plain=1#L197)
*   requirements => [design, formats, architecture]
    *   [REQ_MATCH_ID](#req_match_id-match-by-id "Match by ID")
        Reference: [doc/requirements/DESIGN.md:195](../doc/requirements/DESIGN.md?plain=1#L195)
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
        Reference: [doc/requirements/DESIGN.md:194](../doc/requirements/DESIGN.md?plain=1#L194)
    *   [REQ_UP](#req_up-upward-coverage "Upward Coverage")
        Reference: [doc/requirements/DESIGN.md:196](../doc/requirements/DESIGN.md?plain=1#L196)

Downwards Tracing:
*   design => [formats, code]
    *   [Tracer::trace_relation](#tracertrace_relation)
        Reference: [src/trace.rs:223:24](../src/trace.rs?plain=1#L223)

## DSG_TRACE_VALIDATE_EDGE: Validate Edge is used at least once

Origin: design, [doc/requirements/DESIGN.md:275](../doc/requirements/DESIGN.md?plain=1#L275)


After tracing, if an edge can be found, along which no requirement is
covered, an error is emitted. This is likely a misconfiguration.




Upwards Tracing:
*   architecture => [design]
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")
        Reference: [doc/requirements/DESIGN.md:283](../doc/requirements/DESIGN.md?plain=1#L283)
*   requirements => [design, formats, architecture]

Downwards Tracing:
*   design => [formats, code]
    *    UNCOVERED
## formats

## FMT_CONFIG_TOML: Use a Single TOML File as Configuration

Origin: formats, [doc/requirements/FORMATS.md:86](../doc/requirements/FORMATS.md?plain=1#L86)


The configuration should be placed in one file `requirements.toml`.
The format is TOML.



Upwards Tracing:
*   design => [formats, code]
*   requirements => [design, formats, architecture]
    *   [REQ_CONFIG](#req_config-simple-configuration-in-one-file "Simple Configuration in One File")
        Reference: [doc/requirements/FORMATS.md:92](../doc/requirements/FORMATS.md?plain=1#L92)

Downwards Tracing:
*   formats => [code]
    *   [get_config](#get_config)
        Reference: [src/main.rs:98:4](../src/main.rs?plain=1#L98)

## FMT_EXPORT_CTAGS: Export Requirements as CTags

Origin: formats, [doc/requirements/FORMATS.md:131](../doc/requirements/FORMATS.md?plain=1#L131)


Export Requirements as `tags` file for easy navigation with tools like vim or emacs.

*   with requirement ID
*   File
*   Line Number
*   Type `r`

add the location a requirement was covered at with `c` or depended on with `d`.




Upwards Tracing:
*   design => [formats, code]
*   requirements => [design, formats, architecture]
    *   Derived

Downwards Tracing:
*   formats => [code]
    *    UNCOVERED

## FMT_EXPORT_JSON: JSON for Exporting Results

Origin: formats, [doc/requirements/FORMATS.md:106](../doc/requirements/FORMATS.md?plain=1#L106)


The Json Formatter emits Json objects for:
*   Lists of Errors
*   Lists of Requirements
*   Graph (Artefacts + Relations)
*   Tracing Info

Supported Versions:



Version:
*   `0`: **unstable**. What serde can serialize from current `crate::models::*`

Upwards Tracing:
*   design => [formats, code]
*   requirements => [design, formats, architecture]
    *   [REQ_CACHE_FRIENDLY](#req_cache_friendly-work-well-with-build-systems-that-cache "Work well with build systems that cache")
        Reference: [doc/requirements/FORMATS.md:120](../doc/requirements/FORMATS.md?plain=1#L120)
    *   [REQ_MACHINE_READABLE](#req_machine_readable-machine-readable-output "Machine Readable Output")
        Reference: [doc/requirements/FORMATS.md:120](../doc/requirements/FORMATS.md?plain=1#L120)

Downwards Tracing:
*   formats => [code]
    *    UNCOVERED

## FMT_EXPORT_MARKDOWN: Export to Markdown

Origin: formats, [doc/requirements/FORMATS.md:122](../doc/requirements/FORMATS.md?plain=1#L122)


Errors, Requirements, Status, Tracing Info are exported as a useful
standalone Markdown File. The format may change with future versions.



Upwards Tracing:
*   design => [formats, code]
*   requirements => [design, formats, architecture]
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
        Reference: [doc/requirements/FORMATS.md:128](../doc/requirements/FORMATS.md?plain=1#L128)
    *   [REQ_HUMAN_READABLE](#req_human_readable-human-readable-output "Human Readable Output")
        Reference: [doc/requirements/FORMATS.md:129](../doc/requirements/FORMATS.md?plain=1#L129)

Downwards Tracing:
*   formats => [code]
    *    UNCOVERED

## FMT_FILE_ENCODINGS: Handle File Encodings

Origin: formats, [doc/requirements/FORMATS.md:55](../doc/requirements/FORMATS.md?plain=1#L55)


When not otherwise specified, Text Files are read as UTF-8 and encoding errors are
replaced.



Upwards Tracing:
*   design => [formats, code]
*   requirements => [design, formats, architecture]
    *   [REQ_UNICODE_SAFE](#req_unicode_safe-sane-handling-of-unicode "Sane Handling of unicode")
        Reference: [doc/requirements/FORMATS.md:61](../doc/requirements/FORMATS.md?plain=1#L61)

Downwards Tracing:
*   formats => [code]
    *    UNCOVERED

## FMT_ID_v2: Requirement Identifier

Origin: formats, [doc/requirements/FORMATS.md:36](../doc/requirements/FORMATS.md?plain=1#L36)


Requirement identifier consist of letters, digits and underscore, specifically
they match the Regular Expression





Comment:
Discussion in the [README](README.md#requirement-ids)

History:
*   v2: use to Unicode Identifiers

Upwards Tracing:
*   design => [formats, code]
*   requirements => [design, formats, architecture]
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
        Reference: [doc/requirements/FORMATS.md:46](../doc/requirements/FORMATS.md?plain=1#L46)

Downwards Tracing:
*   formats => [code]
    *    UNCOVERED

## FMT_IMPORT_JSON: JSON for Importing Requirements

Origin: formats, [doc/requirements/FORMATS.md:148](../doc/requirements/FORMATS.md?plain=1#L148)


The Json Parser loads lists of requirements from Json files, matching the following specification:

*   `"version"`: the version of the file format
*   `"requirements"`: a List of Requirement Objects.

*   `0`: **unstable** Only what serde can deserialize into the current `crate::models::Requirement`



Upwards Tracing:
*   design => [formats, code]
*   requirements => [design, formats, architecture]
    *   [REQ_CACHE_FRIENDLY](#req_cache_friendly-work-well-with-build-systems-that-cache "Work well with build systems that cache")
        Reference: [doc/requirements/FORMATS.md:160](../doc/requirements/FORMATS.md?plain=1#L160)
    *   [REQ_EXTENSIBLE](#req_extensible-extensible-parsing "Extensible Parsing")
        Reference: [doc/requirements/FORMATS.md:161](../doc/requirements/FORMATS.md?plain=1#L161)
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
        Reference: [doc/requirements/FORMATS.md:162](../doc/requirements/FORMATS.md?plain=1#L162)

Downwards Tracing:
*   formats => [code]
    *    UNCOVERED

## FMT_IMPORT_MARKDOWN_REQUIREMENT: Markdown File Format

Origin: formats, [doc/requirements/FORMATS.md:170](../doc/requirements/FORMATS.md?plain=1#L170)


The artefact is a Markdown file with freely chosen layout.  A Requirement is in
a heading line with requirement ID, a colon, a space and a title, followed by description and other
attributes.



Upwards Tracing:
*   design => [formats, code]
*   requirements => [design, formats, architecture]
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
        Reference: [doc/requirements/FORMATS.md:178](../doc/requirements/FORMATS.md?plain=1#L178)

Downwards Tracing:
*   formats => [code]
    *    UNCOVERED

## FMT_IMPORT_MONO_REQ: Single Requirement Per File

Origin: formats, [doc/requirements/FORMATS.md:287](../doc/requirements/FORMATS.md?plain=1#L287)


the MonoReq parser emits exactly one Requirement with the following
attributes:
*   Id: The stem of the file path (i.e. `README.md`)
*   Title:  The first line containing Word-Characters with all non-word
    characters trimmed of both ends of the line. (Allowing Markdown heading,
    C style comments, ...)
*   Depends: Every Requirement-Id that immediately follows a fat arrow (`=>`).



Comment:
See this projects README for examples.

Upwards Tracing:
*   design => [formats, code]
*   requirements => [design, formats, architecture]
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
        Reference: [doc/requirements/FORMATS.md:300](../doc/requirements/FORMATS.md?plain=1#L300)

Downwards Tracing:
*   formats => [code]
    *   [parse](#parse)
        Reference: [src/parsers/monoreq.rs:28:4](../src/parsers/monoreq.rs?plain=1#L28)

## FMT_IMPORT_RUS_COV_MARK: Rust Coverage Marks

Origin: formats, [doc/requirements/FORMATS.md:307](../doc/requirements/FORMATS.md?plain=1#L307)


Parse `requirement_covered!(REQ_ID)` and `requirement_covered!(REQ_ID,"TITLE")` as Coverage Links.
The requirement ID is derived form the surrounding items Path



Upwards Tracing:
*   design => [formats, code]
*   requirements => [design, formats, architecture]
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
        Reference: [doc/requirements/FORMATS.md:313](../doc/requirements/FORMATS.md?plain=1#L313)

Downwards Tracing:
*   formats => [code]
    *   [Parser < '_ >::parse_macro](#parser--_-parse_macro)
        Reference: [src/parsers/rust.rs:70:8](../src/parsers/rust.rs?plain=1#L70)

## FMT_MD_ATTRIBUTES: Attributes

Origin: formats, [doc/requirements/FORMATS.md:211](../doc/requirements/FORMATS.md?plain=1#L211)


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



Comment:
`Covers: Some, IDs,`

or

    `Covers:
    *   Some
    *   IDs

Upwards Tracing:
*   design => [formats, code]
*   requirements => [design, formats, architecture]
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
        Reference: [doc/requirements/FORMATS.md:244](../doc/requirements/FORMATS.md?plain=1#L244)

Downwards Tracing:
*   formats => [code]
    *    UNCOVERED

## FMT_MD_DESC: Description

Origin: formats, [doc/requirements/FORMATS.md:188](../doc/requirements/FORMATS.md?plain=1#L188)


The paragraphs following the start of the requirement make up the description of
the requirement.

*   The Start of another Requirement.
*   The start of an Attribute Paragraph
*   A Heading the same level or less. This ends the Requirement.




Upwards Tracing:
*   design => [formats, code]
*   requirements => [design, formats, architecture]
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
        Reference: [doc/requirements/FORMATS.md:200](../doc/requirements/FORMATS.md?plain=1#L200)

Downwards Tracing:
*   formats => [code]
    *    UNCOVERED

## FMT_MD_DESC_HEADINGS: Heading Level in Description is adjusted

Origin: formats, [doc/requirements/FORMATS.md:202](../doc/requirements/FORMATS.md?plain=1#L202)


Headings with a lower level than the starting one, that do not start a nested
requirement are added to the description. Their heading level is adjusted by
removing as many leading `#` as the requirement had



Upwards Tracing:
*   design => [formats, code]
*   requirements => [design, formats, architecture]
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
        Reference: [doc/requirements/FORMATS.md:209](../doc/requirements/FORMATS.md?plain=1#L209)

Downwards Tracing:
*   formats => [code]
    *    UNCOVERED

## FMT_MD_OPT_PREFIX: List of Prefixes

Origin: formats, [doc/requirements/FORMATS.md:267](../doc/requirements/FORMATS.md?plain=1#L267)


A List of strings can be passed, which is used to prevent the parser from
creating unintended requirements from headlines which accidentally have the
right form.

normal headings, if the identifier of the would be requirement does not start
with one of the list of prefixes. If the list is empty, no prefix matching is
performed and all matching lines lead to a requirement.



Upwards Tracing:
*   design => [formats, code]
*   requirements => [design, formats, architecture]
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
        Reference: [doc/requirements/FORMATS.md:279](../doc/requirements/FORMATS.md?plain=1#L279)

Downwards Tracing:
*   formats => [code]
    *    UNCOVERED

## FMT_MD_START: Requirement Start

Origin: formats, [doc/requirements/FORMATS.md:181](../doc/requirements/FORMATS.md?plain=1#L181)


A Requirement starts with a `#` heading of any level that has the form `ID:
TITLE`.



Upwards Tracing:
*   design => [formats, code]
*   requirements => [design, formats, architecture]
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
        Reference: [doc/requirements/FORMATS.md:187](../doc/requirements/FORMATS.md?plain=1#L187)

Downwards Tracing:
*   formats => [code]
    *    UNCOVERED

## FMT_UNICODE_NORMALIZE: Normalize Unicode during read

Origin: formats, [doc/requirements/FORMATS.md:64](../doc/requirements/FORMATS.md?plain=1#L64)


All input strings are unicode normalizes as
[NFC](https://www.unicode.org/reports/tr15/#Normalization_Forms_Table).
This means that
*   All output derived from input will be NFC normalized
*   Identifier Matching can be done on the byte level



Comment:
See [Rust RFC 2457](https://rust-lang.github.io/rfcs/2457-non-ascii-idents.html) on the topic.

This means two requirement ids are equal if their NFC forms are equal.

Upwards Tracing:
*   design => [formats, code]
*   requirements => [design, formats, architecture]
    *   [REQ_UNICODE_SAFE](#req_unicode_safe-sane-handling-of-unicode "Sane Handling of unicode")
        Reference: [doc/requirements/FORMATS.md:73](../doc/requirements/FORMATS.md?plain=1#L73)

Downwards Tracing:
*   formats => [code]
    *    UNCOVERED
## readme

## README: Requirement Tracing

Origin: readme, [README.md](../README.md)

Upwards Tracing:
    *   Derived

Downwards Tracing:
*   readme => [requirements]
    *   [REQ_TRACE](#req_trace-determine-which-requirements-cover-which "Determine which requirements cover which")
        Reference: [README.md:10](../README.md?plain=1#L10)
    *   [REQ_UP](#req_up-upward-coverage "Upward Coverage")
        Reference: [README.md:11](../README.md?plain=1#L11)
    *   [REQ_DOWN](#req_down-downward-coverage "Downward Coverage")
        Reference: [README.md:12](../README.md?plain=1#L12)
    *   [REQ_EXTENSIBLE](#req_extensible-extensible-parsing "Extensible Parsing")
        Reference: [README.md:15](../README.md?plain=1#L15)
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
        Reference: [README.md:16](../README.md?plain=1#L16)
    *   [REQ_HUMAN_READABLE](#req_human_readable-human-readable-output "Human Readable Output")
        Reference: [README.md:20](../README.md?plain=1#L20)
    *   [REQ_MACHINE_READABLE](#req_machine_readable-machine-readable-output "Machine Readable Output")
        Reference: [README.md:21](../README.md?plain=1#L21)
    *   [REQ_FORMATS](#req_formats-well-defined-formats "Well defined Formats")
        Reference: [README.md:22](../README.md?plain=1#L22)
    *   [REQ_FAST](#req_fast-fast "Fast")
        Reference: [README.md:26](../README.md?plain=1#L26)
    *   [REQ_CACHE_FRIENDLY](#req_cache_friendly-work-well-with-build-systems-that-cache "Work well with build systems that cache")
        Reference: [README.md:27](../README.md?plain=1#L27)
    *   [REQ_VAL_TITLE](#req_val_title-check-matching-title "Check matching title")
        Reference: [README.md:28](../README.md?plain=1#L28)
    *   [REQ_CONFIG](#req_config-simple-configuration-in-one-file "Simple Configuration in One File")
        Reference: [README.md:31](../README.md?plain=1#L31)
    *   [REQ_MACHINE_FRIENDLY](#req_machine_friendly-easy-to-include-in-automated-work-flows "Easy to include in automated work flows")
        Reference: [README.md:32](../README.md?plain=1#L32)
    *   [REQ_LATE_ERROR](#req_late_error-collect-errors-but-continue-processing "Collect Errors but continue processing")
        Reference: [README.md:36](../README.md?plain=1#L36)
## requirements

## REQ_CACHE_FRIENDLY: Work well with build systems that cache

Origin: requirements, [doc/requirements/REQUIREMENTS.md:132](../doc/requirements/REQUIREMENTS.md?plain=1#L132)


Report all files which are consumed, so that build systems like make or
ninja can know when an input has changed an Given Data and a requested format, the formatter formats the given data in the
given format.rerun the tool.



Upwards Tracing:
*   readme => [requirements]
    *   [README](#readme-requirement-tracing "Requirement Tracing")
        Reference: [README.md:27](../README.md?plain=1#L27)

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [FMT_EXPORT_JSON](#fmt_export_json-json-for-exporting-results "JSON for Exporting Results")
        Reference: [doc/requirements/FORMATS.md:120](../doc/requirements/FORMATS.md?plain=1#L120)
    *   [FMT_IMPORT_JSON](#fmt_import_json-json-for-importing-requirements "JSON for Importing Requirements")
        Reference: [doc/requirements/FORMATS.md:160](../doc/requirements/FORMATS.md?plain=1#L160)

## REQ_CONFIG: Simple Configuration in One File

Origin: requirements, [doc/requirements/REQUIREMENTS.md:123](../doc/requirements/REQUIREMENTS.md?plain=1#L123)


All Configuration is stored in a single file using a common Format that is
editable for humans and machine readable.



Upwards Tracing:
*   readme => [requirements]
    *   [README](#readme-requirement-tracing "Requirement Tracing")
        Reference: [README.md:31](../README.md?plain=1#L31)

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [DSG_CTRL_CONFIG](#dsg_ctrl_config-single-config-file "Single Config File")
        Reference: [doc/requirements/DESIGN.md:63](../doc/requirements/DESIGN.md?plain=1#L63)
    *   [DSG_JOBS](#dsg_jobs-jobs-encode-requested-behavior "Jobs encode requested behavior")
        Reference: [doc/requirements/DESIGN.md:51](../doc/requirements/DESIGN.md?plain=1#L51)
    *   [FMT_CONFIG_TOML](#fmt_config_toml-use-a-single-toml-file-as-configuration "Use a Single TOML File as Configuration")
        Reference: [doc/requirements/FORMATS.md:92](../doc/requirements/FORMATS.md?plain=1#L92)

## REQ_DOWN: Downward Coverage

Origin: requirements, [doc/requirements/REQUIREMENTS.md:67](../doc/requirements/REQUIREMENTS.md?plain=1#L67)


A Requirement is covered by a lower one by including the id of the lower one in
its Dependencies attribute.



Upwards Tracing:
*   readme => [requirements]
    *   [README](#readme-requirement-tracing "Requirement Tracing")
        Reference: [README.md:12](../README.md?plain=1#L12)

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [DSG_REQ_FIELDS](#dsg_req_fields-requirement-fields "Requirement Fields")
        Reference: [doc/requirements/DESIGN.md:338](../doc/requirements/DESIGN.md?plain=1#L338)
    *   [DSG_TRACE_DOWNWARDS](#dsg_trace_downwards-trace-downwards-using-depends-attribute "Trace downwards using `depends` attribute")
        Reference: [doc/requirements/DESIGN.md:206](../doc/requirements/DESIGN.md?plain=1#L206)
    *   [DSG_TRACE_REFERENCE_EXIST](#dsg_trace_reference_exist-coverage-links-must-exist "Coverage Links must exist")
        Reference: [doc/requirements/DESIGN.md:270](../doc/requirements/DESIGN.md?plain=1#L270)

## REQ_EXTENSIBLE: Extensible Parsing

Origin: requirements, [doc/requirements/REQUIREMENTS.md:28](../doc/requirements/REQUIREMENTS.md?plain=1#L28)


If internal parsers are not able to work on an Artefact, external tools can be
incorporated.



Upwards Tracing:
*   readme => [requirements]
    *   [README](#readme-requirement-tracing "Requirement Tracing")
        Reference: [README.md:15](../README.md?plain=1#L15)

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [FMT_IMPORT_JSON](#fmt_import_json-json-for-importing-requirements "JSON for Importing Requirements")
        Reference: [doc/requirements/FORMATS.md:161](../doc/requirements/FORMATS.md?plain=1#L161)
    *   [ARCH_PARSER](#arch_parser-parser "Parser")
        Reference: [doc/requirements/ARCHITECTURE.md:52](../doc/requirements/ARCHITECTURE.md?plain=1#L52)

## REQ_FAST: Fast

Origin: requirements, [doc/requirements/REQUIREMENTS.md:128](../doc/requirements/REQUIREMENTS.md?plain=1#L128)


Show results quickly, especially if only a small query is given.



Upwards Tracing:
*   readme => [requirements]
    *   [README](#readme-requirement-tracing "Requirement Tracing")
        Reference: [README.md:26](../README.md?plain=1#L26)

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [ARCH_CLI](#arch_cli-command-line-interface "Command Line Interface")
        Reference: [doc/requirements/ARCHITECTURE.md:32](../doc/requirements/ARCHITECTURE.md?plain=1#L32)

## REQ_FORMATS: Well defined Formats

Origin: requirements, [doc/requirements/REQUIREMENTS.md:49](../doc/requirements/REQUIREMENTS.md?plain=1#L49)


To work with external programs as parsers or to process the output, the formats used must be well
defined.



Upwards Tracing:
*   readme => [requirements]
    *   [README](#readme-requirement-tracing "Requirement Tracing")
        Reference: [README.md:16](../README.md?plain=1#L16)
    *   [README](#readme-requirement-tracing "Requirement Tracing")
        Reference: [README.md:22](../README.md?plain=1#L22)

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [FMT_EXPORT_MARKDOWN](#fmt_export_markdown-export-to-markdown "Export to Markdown")
        Reference: [doc/requirements/FORMATS.md:128](../doc/requirements/FORMATS.md?plain=1#L128)
    *   [FMT_ID_v2](#fmt_id_v2-requirement-identifier "Requirement Identifier")
        Reference: [doc/requirements/FORMATS.md:46](../doc/requirements/FORMATS.md?plain=1#L46)
    *   [FMT_IMPORT_JSON](#fmt_import_json-json-for-importing-requirements "JSON for Importing Requirements")
        Reference: [doc/requirements/FORMATS.md:162](../doc/requirements/FORMATS.md?plain=1#L162)
    *   [FMT_IMPORT_MARKDOWN_REQUIREMENT](#fmt_import_markdown_requirement-markdown-file-format "Markdown File Format")
        Reference: [doc/requirements/FORMATS.md:178](../doc/requirements/FORMATS.md?plain=1#L178)
    *   [FMT_IMPORT_MONO_REQ](#fmt_import_mono_req-single-requirement-per-file "Single Requirement Per File")
        Reference: [doc/requirements/FORMATS.md:300](../doc/requirements/FORMATS.md?plain=1#L300)
    *   [FMT_IMPORT_RUS_COV_MARK](#fmt_import_rus_cov_mark-rust-coverage-marks "Rust Coverage Marks")
        Reference: [doc/requirements/FORMATS.md:313](../doc/requirements/FORMATS.md?plain=1#L313)
    *   [FMT_MD_ATTRIBUTES](#fmt_md_attributes-attributes "Attributes")
        Reference: [doc/requirements/FORMATS.md:244](../doc/requirements/FORMATS.md?plain=1#L244)
    *   [FMT_MD_DESC](#fmt_md_desc-description "Description")
        Reference: [doc/requirements/FORMATS.md:200](../doc/requirements/FORMATS.md?plain=1#L200)
    *   [FMT_MD_DESC_HEADINGS](#fmt_md_desc_headings-heading-level-in-description-is-adjusted "Heading Level in Description is adjusted")
        Reference: [doc/requirements/FORMATS.md:209](../doc/requirements/FORMATS.md?plain=1#L209)
    *   [FMT_MD_OPT_PREFIX](#fmt_md_opt_prefix-list-of-prefixes "List of Prefixes")
        Reference: [doc/requirements/FORMATS.md:279](../doc/requirements/FORMATS.md?plain=1#L279)
    *   [FMT_MD_START](#fmt_md_start-requirement-start "Requirement Start")
        Reference: [doc/requirements/FORMATS.md:187](../doc/requirements/FORMATS.md?plain=1#L187)

## REQ_HUMAN_READABLE: Human Readable Output

Origin: requirements, [doc/requirements/REQUIREMENTS.md:45](../doc/requirements/REQUIREMENTS.md?plain=1#L45)


Information can be returned in a format that can easily be read by humans



Upwards Tracing:
*   readme => [requirements]
    *   [README](#readme-requirement-tracing "Requirement Tracing")
        Reference: [README.md:20](../README.md?plain=1#L20)

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [FMT_EXPORT_MARKDOWN](#fmt_export_markdown-export-to-markdown "Export to Markdown")
        Reference: [doc/requirements/FORMATS.md:129](../doc/requirements/FORMATS.md?plain=1#L129)
    *   [ARCH_FORMATTER](#arch_formatter-format-output-in-requested-format "Format output in requested Format")
        Reference: [doc/requirements/ARCHITECTURE.md:70](../doc/requirements/ARCHITECTURE.md?plain=1#L70)

## REQ_IDENTIFIEABLE: Show versions of input artefacts in output

Origin: requirements, [doc/requirements/REQUIREMENTS.md:33](../doc/requirements/REQUIREMENTS.md?plain=1#L33)


When reading the output, each input must be clearly identifiable.
For example by its:
*   git describe
*   hash
*   file modification time



Upwards Tracing:
*   readme => [requirements]
    *   Derived

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *    UNCOVERED

## REQ_INSTALL: Easy to install

Origin: requirements, [doc/requirements/REQUIREMENTS.md:23](../doc/requirements/REQUIREMENTS.md?plain=1#L23)


The tool should be distributed as an executable without depending on
libraries, files, etc.



Upwards Tracing:
*   readme => [requirements]
    *   Derived

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [ARCH_CLI](#arch_cli-command-line-interface "Command Line Interface")
        Reference: [doc/requirements/ARCHITECTURE.md:31](../doc/requirements/ARCHITECTURE.md?plain=1#L31)

## REQ_LATE_ERROR: Collect Errors but continue processing

Origin: requirements, [doc/requirements/REQUIREMENTS.md:77](../doc/requirements/REQUIREMENTS.md?plain=1#L77)


When errors are encountered in parsing, tracing or outputting, processing
continues as long as possible and then all errors are reported.



Upwards Tracing:
*   readme => [requirements]
    *   [README](#readme-requirement-tracing "Requirement Tracing")
        Reference: [README.md:36](../README.md?plain=1#L36)

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [DSG_ART_PARSE_COLLECT_ERRORS](#dsg_art_parse_collect_errors-collect-errors-but-keep-parsing "Collect errors but keep parsing")
        Reference: [doc/requirements/DESIGN.md:359](../doc/requirements/DESIGN.md?plain=1#L359)

## REQ_MACHINE_FRIENDLY: Easy to include in automated work flows

Origin: requirements, [doc/requirements/REQUIREMENTS.md:19](../doc/requirements/REQUIREMENTS.md?plain=1#L19)


For ease of integration into other tools, all functionality must be available via a CLI.



Upwards Tracing:
*   readme => [requirements]
    *   [README](#readme-requirement-tracing "Requirement Tracing")
        Reference: [README.md:32](../README.md?plain=1#L32)

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [DSG_CLI](#dsg_cli-command-line-interface "Command Line Interface")
        Reference: [doc/requirements/DESIGN.md:15](../doc/requirements/DESIGN.md?plain=1#L15)
    *   [DSG_CLI_RETURN_CODE](#dsg_cli_return_code-set-return-code-to-indicate-success "Set return Code to indicate success")
        Reference: [doc/requirements/DESIGN.md:27](../doc/requirements/DESIGN.md?plain=1#L27)
    *   [ARCH_CLI](#arch_cli-command-line-interface "Command Line Interface")
        Reference: [doc/requirements/ARCHITECTURE.md:30](../doc/requirements/ARCHITECTURE.md?plain=1#L30)

## REQ_MACHINE_READABLE: Machine Readable Output

Origin: requirements, [doc/requirements/REQUIREMENTS.md:41](../doc/requirements/REQUIREMENTS.md?plain=1#L41)


Information can be returned in a format that can easily be read by other tools



Upwards Tracing:
*   readme => [requirements]
    *   [README](#readme-requirement-tracing "Requirement Tracing")
        Reference: [README.md:21](../README.md?plain=1#L21)

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [FMT_EXPORT_JSON](#fmt_export_json-json-for-exporting-results "JSON for Exporting Results")
        Reference: [doc/requirements/FORMATS.md:120](../doc/requirements/FORMATS.md?plain=1#L120)
    *   [ARCH_FORMATTER](#arch_formatter-format-output-in-requested-format "Format output in requested Format")
        Reference: [doc/requirements/ARCHITECTURE.md:71](../doc/requirements/ARCHITECTURE.md?plain=1#L71)

## REQ_MATCH_ID: Match by ID

Origin: requirements, [doc/requirements/REQUIREMENTS.md:82](../doc/requirements/REQUIREMENTS.md?plain=1#L82)


A Requirements covers another by its ID.



Upwards Tracing:
*   readme => [requirements]
    *   Derived

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [DSG_TRACE_DOWNWARDS](#dsg_trace_downwards-trace-downwards-using-depends-attribute "Trace downwards using `depends` attribute")
        Reference: [doc/requirements/DESIGN.md:205](../doc/requirements/DESIGN.md?plain=1#L205)
    *   [DSG_TRACE_UPWARDS](#dsg_trace_upwards-trace-upwards-using-covers-attribute "Trace upwards using `covers` attribute")
        Reference: [doc/requirements/DESIGN.md:195](../doc/requirements/DESIGN.md?plain=1#L195)

## REQ_PARSER_ERROR: Useful Parser Errors

Origin: requirements, [doc/requirements/REQUIREMENTS.md:72](../doc/requirements/REQUIREMENTS.md?plain=1#L72)


Parser Errors give the precise location and type of the problem, for example filename with
line number of the artefact.



Upwards Tracing:
*   readme => [requirements]
    *   Derived

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [DSG_ART_PARSE_COLLECT_ERRORS](#dsg_art_parse_collect_errors-collect-errors-but-keep-parsing "Collect errors but keep parsing")
        Reference: [doc/requirements/DESIGN.md:360](../doc/requirements/DESIGN.md?plain=1#L360)

## REQ_TRACE: Determine which requirements cover which

Origin: requirements, [doc/requirements/REQUIREMENTS.md:5](../doc/requirements/REQUIREMENTS.md?plain=1#L5)


Compute tracing for each Requirement, whether it is covered, uncovered, covers
another requirement or is derived.



Upwards Tracing:
*   readme => [requirements]
    *   [README](#readme-requirement-tracing "Requirement Tracing")
        Reference: [README.md:10](../README.md?plain=1#L10)

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [DSG_TRACE_CHECK_TITLE](#dsg_trace_check_title-when-tracing-upwards-or-downwards-match-title "When tracing upwards or downwards match title")
        Reference: [doc/requirements/DESIGN.md:255](../doc/requirements/DESIGN.md?plain=1#L255)
    *   [DSG_TRACE_DERIVED](#dsg_trace_derived-record-requirements-that-do-not-cover-anything "Record requirements that do not cover anything")
        Reference: [doc/requirements/DESIGN.md:217](../doc/requirements/DESIGN.md?plain=1#L217)
    *   [DSG_TRACE_DOWNWARDS](#dsg_trace_downwards-trace-downwards-using-depends-attribute "Trace downwards using `depends` attribute")
        Reference: [doc/requirements/DESIGN.md:204](../doc/requirements/DESIGN.md?plain=1#L204)
    *   [DSG_TRACE_REFERENCE_EXIST](#dsg_trace_reference_exist-coverage-links-must-exist "Coverage Links must exist")
        Reference: [doc/requirements/DESIGN.md:269](../doc/requirements/DESIGN.md?plain=1#L269)
    *   [DSG_TRACE_UNCOVERED](#dsg_trace_uncovered-record-requirements-that-are-not-completely-covered "Record requirements that are not completely covered")
        Reference: [doc/requirements/DESIGN.md:233](../doc/requirements/DESIGN.md?plain=1#L233)
    *   [DSG_TRACE_UPWARDS](#dsg_trace_upwards-trace-upwards-using-covers-attribute "Trace upwards using `covers` attribute")
        Reference: [doc/requirements/DESIGN.md:194](../doc/requirements/DESIGN.md?plain=1#L194)
    *   [ARCH_GRAPH](#arch_graph-graph "Graph")
        Reference: [doc/requirements/ARCHITECTURE.md:118](../doc/requirements/ARCHITECTURE.md?plain=1#L118)
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")
        Reference: [doc/requirements/ARCHITECTURE.md:60](../doc/requirements/ARCHITECTURE.md?plain=1#L60)
    *   [ARCH_TRACED_GRAPH](#arch_traced_graph-tracing-information-of-grpah "Tracing Information of Grpah")
        Reference: [doc/requirements/ARCHITECTURE.md:126](../doc/requirements/ARCHITECTURE.md?plain=1#L126)

## REQ_UNICODE_SAFE: Sane Handling of unicode

Origin: requirements, [doc/requirements/REQUIREMENTS.md:54](../doc/requirements/REQUIREMENTS.md?plain=1#L54)


Some Characters can be represented by multiple different sequences of Unicode
Code Points. Also Unicode Encodings like UTF-8 can encode the same Codepoint
as different bytes.




Upwards Tracing:
*   readme => [requirements]
    *   Derived

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [FMT_FILE_ENCODINGS](#fmt_file_encodings-handle-file-encodings "Handle File Encodings")
        Reference: [doc/requirements/FORMATS.md:61](../doc/requirements/FORMATS.md?plain=1#L61)
    *   [FMT_UNICODE_NORMALIZE](#fmt_unicode_normalize-normalize-unicode-during-read "Normalize Unicode during read")
        Reference: [doc/requirements/FORMATS.md:73](../doc/requirements/FORMATS.md?plain=1#L73)

## REQ_UNIQUE_ID_v2: Requirements have a unique Identifier

Origin: requirements, [doc/requirements/REQUIREMENTS.md:10](../doc/requirements/REQUIREMENTS.md?plain=1#L10)


Each requirement must be identifiable by a short, unique string.
All unicode symbols typically used as identifiers must be possible,
though parsers may restrict this



History:
*   v2: Unicode

Upwards Tracing:
*   readme => [requirements]
    *   Derived

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [DSG_CTRL_DETECT_DUPLICATE_REQS](#dsg_ctrl_detect_duplicate_reqs-detect-duplicates "Detect duplicates")
        Reference: [doc/requirements/DESIGN.md:81](../doc/requirements/DESIGN.md?plain=1#L81)
    *   [DSG_TRACE_DETECT_DUPLICATE](#dsg_trace_detect_duplicate-detect-duplicate-requirements-in-different-artefacts "Detect duplicate Requirements in different Artefacts")
        Reference: [doc/requirements/DESIGN.md:166](../doc/requirements/DESIGN.md?plain=1#L166)

## REQ_UP: Upward Coverage

Origin: requirements, [doc/requirements/REQUIREMENTS.md:62](../doc/requirements/REQUIREMENTS.md?plain=1#L62)


A Requirement covers a higher one by including the id of the higer one in its
Coverage attribute.



Upwards Tracing:
*   readme => [requirements]
    *   [README](#readme-requirement-tracing "Requirement Tracing")
        Reference: [README.md:11](../README.md?plain=1#L11)

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [DSG_REQ_FIELDS](#dsg_req_fields-requirement-fields "Requirement Fields")
        Reference: [doc/requirements/DESIGN.md:337](../doc/requirements/DESIGN.md?plain=1#L337)
    *   [DSG_TRACE_REFERENCE_EXIST](#dsg_trace_reference_exist-coverage-links-must-exist "Coverage Links must exist")
        Reference: [doc/requirements/DESIGN.md:271](../doc/requirements/DESIGN.md?plain=1#L271)
    *   [DSG_TRACE_UPWARDS](#dsg_trace_upwards-trace-upwards-using-covers-attribute "Trace upwards using `covers` attribute")
        Reference: [doc/requirements/DESIGN.md:196](../doc/requirements/DESIGN.md?plain=1#L196)

## REQ_VAL_COVERAGE: Validate Coverage

Origin: requirements, [doc/requirements/REQUIREMENTS.md:117](../doc/requirements/REQUIREMENTS.md?plain=1#L117)


An error is reported for a Coverage claim for which no Requirement exists in the
relevant artefacts.




Upwards Tracing:
*   readme => [requirements]
    *   Derived

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [DSG_TRACE_REFERENCE_EXIST](#dsg_trace_reference_exist-coverage-links-must-exist "Coverage Links must exist")
        Reference: [doc/requirements/DESIGN.md:273](../doc/requirements/DESIGN.md?plain=1#L273)

## REQ_VAL_TITLE: Check matching title

Origin: requirements, [doc/requirements/REQUIREMENTS.md:86](../doc/requirements/REQUIREMENTS.md?plain=1#L86)


A Coverage link that is established by requirement ID can be verified by
comparing the requirement's title.



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

Upwards Tracing:
*   readme => [requirements]
    *   [README](#readme-requirement-tracing "Requirement Tracing")
        Reference: [README.md:28](../README.md?plain=1#L28)

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [DSG_TRACE_CHECK_TITLE](#dsg_trace_check_title-when-tracing-upwards-or-downwards-match-title "When tracing upwards or downwards match title")
        Reference: [doc/requirements/DESIGN.md:256](../doc/requirements/DESIGN.md?plain=1#L256)

## UC_CHECK: Check for correct Tracing

Origin: requirements, [doc/requirements/REQUIREMENTS.md:156](../doc/requirements/REQUIREMENTS.md?plain=1#L156)


Like `UC_TRACE` but the only output of interest is whether there were tracing errors or not,
for use in CI/CD Pipelines.


Upwards Tracing:
*   readme => [requirements]
    *   Derived

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [DSG_CLI_RETURN_CODE](#dsg_cli_return_code-set-return-code-to-indicate-success "Set return Code to indicate success")
        Reference: [doc/requirements/DESIGN.md:28](../doc/requirements/DESIGN.md?plain=1#L28)
    *   [DSG_CTRL_TRACE](#dsg_ctrl_trace-trace-requirements "Trace Requirements")
        Reference: [doc/requirements/DESIGN.md:102](../doc/requirements/DESIGN.md?plain=1#L102)

## UC_PARSE: Parse Artefacts

Origin: requirements, [doc/requirements/REQUIREMENTS.md:140](../doc/requirements/REQUIREMENTS.md?plain=1#L140)


A Set of artefacts are parsed, reporting all requirements and errors.



Parameters:
*   Artefacts to Parse

Upwards Tracing:
*   readme => [requirements]
    *   Derived

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [DSG_CTRL_PARSE](#dsg_ctrl_parse-parse-all-artefacts "Parse all Artefacts")
        Reference: [doc/requirements/DESIGN.md:74](../doc/requirements/DESIGN.md?plain=1#L74)
    *   [ARCH_PARSER](#arch_parser-parser "Parser")
        Reference: [doc/requirements/ARCHITECTURE.md:53](../doc/requirements/ARCHITECTURE.md?plain=1#L53)

## UC_TMX: Create Traceability Matrix

Origin: requirements, [doc/requirements/REQUIREMENTS.md:147](../doc/requirements/REQUIREMENTS.md?plain=1#L147)


All requirements are matched up and down the Tracing Graph. The results are
stored in a file and bad tracing is reported.



Parameters:
*   Tracing Report Format
*   Tracing Report File

Upwards Tracing:
*   readme => [requirements]
    *   Derived

Downwards Tracing:
*   requirements => [design, formats, architecture]
    *   [DSG_CTRL_TRACE](#dsg_ctrl_trace-trace-requirements "Trace Requirements")
        Reference: [doc/requirements/DESIGN.md:101](../doc/requirements/DESIGN.md?plain=1#L101)
    *   [ARCH_FORMATTER](#arch_formatter-format-output-in-requested-format "Format output in requested Format")
        Reference: [doc/requirements/ARCHITECTURE.md:69](../doc/requirements/ARCHITECTURE.md?plain=1#L69)
    *   [ARCH_TRACE](#arch_trace-tracer "Tracer")
        Reference: [doc/requirements/ARCHITECTURE.md:61](../doc/requirements/ARCHITECTURE.md?plain=1#L61)
