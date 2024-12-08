

# Input Errors


## Lower Artefact

*   Format Error: Expected an Attribute line like `Comment:`
    in [tests/data/continue_parsing_after_errors/lower.md:11](../tests/data/continue_parsing_after_errors/lower.md?plain=1#L11)
*   Format Error: Expected a Reference like `* REQ_ID: Title`
    in [tests/data/continue_parsing_after_errors/lower.md:16](../tests/data/continue_parsing_after_errors/lower.md?plain=1#L16)

## Upper Artefact

*   Format Error: Expected a Reference like `* REQ_ID: Title`
    in [tests/data/continue_parsing_after_errors/upper.md:9](../tests/data/continue_parsing_after_errors/upper.md?plain=1#L9)
*   Duplicate Attribute: Bar when parsing REQ_2
    [tests/data/continue_parsing_after_errors/upper.md:15](../tests/data/continue_parsing_after_errors/upper.md?plain=1#L15)
*   Duplicate Attribute: Bar when parsing REQ_2
    [tests/data/continue_parsing_after_errors/upper.md:17](../tests/data/continue_parsing_after_errors/upper.md?plain=1#L17)


# Requirements

## Lower Artefact

### COV_1: A Coverage

Origin: Lower Artefact, [tests/data/continue_parsing_after_errors/lower.md:5](../tests/data/continue_parsing_after_errors/lower.md?plain=1#L5)

Upwards Tracing:
*   Upper Artefact => [Lower Artefact]
    *   [REQ_1](#req_1-a-requirement "A Requirement")
        Reference: [tests/data/continue_parsing_after_errors/upper.md:8](../tests/data/continue_parsing_after_errors/upper.md?plain=1#L8)
    *   [REQ_1](#req_1-a-requirement "A Requirement")
        Reference: [tests/data/continue_parsing_after_errors/lower.md:8](../tests/data/continue_parsing_after_errors/lower.md?plain=1#L8)
    *   [REQ_2](#req_2-another-requirement "Another Requirement")
        Reference: [tests/data/continue_parsing_after_errors/lower.md:9](../tests/data/continue_parsing_after_errors/lower.md?plain=1#L9)

Downwards Tracing:
## Upper Artefact

### REQ_1: A Requirement

Origin: Upper Artefact, [tests/data/continue_parsing_after_errors/upper.md:5](../tests/data/continue_parsing_after_errors/upper.md?plain=1#L5)

Upwards Tracing:
    *   Derived

Downwards Tracing:
*   Upper Artefact => [Lower Artefact]
    *   [COV_1](#cov_1-a-coverage "A Coverage")
        Reference: [tests/data/continue_parsing_after_errors/upper.md:8](../tests/data/continue_parsing_after_errors/upper.md?plain=1#L8)
    *   [COV_1](#cov_1-a-coverage "A Coverage")
        Reference: [tests/data/continue_parsing_after_errors/lower.md:8](../tests/data/continue_parsing_after_errors/lower.md?plain=1#L8)

### REQ_2: Another Requirement

Origin: Upper Artefact, [tests/data/continue_parsing_after_errors/upper.md:11](../tests/data/continue_parsing_after_errors/upper.md?plain=1#L11)

Upwards Tracing:
    *   Derived

Downwards Tracing:
*   Upper Artefact => [Lower Artefact]
    *   [COV_1](#cov_1-a-coverage "A Coverage")
        Reference: [tests/data/continue_parsing_after_errors/lower.md:9](../tests/data/continue_parsing_after_errors/lower.md?plain=1#L9)
