

# Tracing Errors

*   Duplicate Requirement: REQ_1
    first seen in [tests/data/requirement_twice_in_2_artefacts/upper.md:5](../tests/data/requirement_twice_in_2_artefacts/upper.md?plain=1#L5)
    then again in [tests/data/requirement_twice_in_2_artefacts/lower.md:8](../tests/data/requirement_twice_in_2_artefacts/lower.md?plain=1#L8)


# Derived Requirements


## Lower Artefact

*   [REQ_1](#req_1-a-requirement "A Requirement")


# Requirements

## Lower Artefact

## COV_1: A Coverage

Origin: Lower Artefact, [tests/data/requirement_twice_in_2_artefacts/lower.md:5](../tests/data/requirement_twice_in_2_artefacts/lower.md?plain=1#L5)

Upwards Tracing:
*   Upper Artefact => [Lower Artefact]
    *   [REQ_1](#req_1-a-requirement "A Requirement")
        Reference: [tests/data/requirement_twice_in_2_artefacts/lower.md:6](../tests/data/requirement_twice_in_2_artefacts/lower.md?plain=1#L6)

Downwards Tracing:

## REQ_1: A Requirement

Origin: Upper Artefact, [tests/data/requirement_twice_in_2_artefacts/upper.md:5](../tests/data/requirement_twice_in_2_artefacts/upper.md?plain=1#L5)

Upwards Tracing:
*   Upper Artefact => [Lower Artefact]
    *   Derived

Downwards Tracing:
*   Upper Artefact => [Lower Artefact]
    *   [COV_1](#cov_1-a-coverage "A Coverage")
        Reference: [tests/data/requirement_twice_in_2_artefacts/lower.md:6](../tests/data/requirement_twice_in_2_artefacts/lower.md?plain=1#L6)
## Upper Artefact

## REQ_1: A Requirement

Origin: Upper Artefact, [tests/data/requirement_twice_in_2_artefacts/upper.md:5](../tests/data/requirement_twice_in_2_artefacts/upper.md?plain=1#L5)

Upwards Tracing:
*   Upper Artefact => [Lower Artefact]
    *   Derived

Downwards Tracing:
*   Upper Artefact => [Lower Artefact]
    *   [COV_1](#cov_1-a-coverage "A Coverage")
        Reference: [tests/data/requirement_twice_in_2_artefacts/lower.md:6](../tests/data/requirement_twice_in_2_artefacts/lower.md?plain=1#L6)
