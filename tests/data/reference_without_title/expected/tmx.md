

# Tracing Errors

*   [REQ_NUM_1](#req_num_1-a-requirement "A Requirement") covered without title
    tests/data/reference_without_title/lower_numeric.md:8
*   [COV_NUM_2](#cov_num_2-another-coverage "Another Coverage") covered without title
    tests/data/reference_without_title/upper_numeric.md:10


# Requirements

## Lower Artefact using numeric IDs

### COV_NUM_1: A Coverage

Origin: Lower Artefact using numeric IDs, [tests/data/reference_without_title/lower_numeric.md:5](../tests/data/reference_without_title/lower_numeric.md?plain=1#L5)

Upwards Tracing:
*   Upper Artefact using numeric IDs => [Lower Artefact using numeric IDs]
    *   [REQ_NUM_1](#req_num_1-a-requirement "A Requirement")
        Reference: [tests/data/reference_without_title/lower_numeric.md:8](../tests/data/reference_without_title/lower_numeric.md?plain=1#L8)

Downwards Tracing:

### COV_NUM_2: Another Coverage

Origin: Lower Artefact using numeric IDs, [tests/data/reference_without_title/lower_numeric.md:10](../tests/data/reference_without_title/lower_numeric.md?plain=1#L10)

Upwards Tracing:
*   Upper Artefact using numeric IDs => [Lower Artefact using numeric IDs]
    *   [REQ_NUM_2](#req_num_2-another-requirement "Another Requirement")
        Reference: [tests/data/reference_without_title/upper_numeric.md:10](../tests/data/reference_without_title/upper_numeric.md?plain=1#L10)

Downwards Tracing:
## Lower Artefact using readable IDs

### COV_WITH_ANOTHER_GOOD_NAME: Another Coverage

Origin: Lower Artefact using readable IDs, [tests/data/reference_without_title/lower_readable.md:9](../tests/data/reference_without_title/lower_readable.md?plain=1#L9)

Upwards Tracing:
*   Upper Artefact using readable IDs => [Lower Artefact using readable IDs]
    *   [REQ_WITH_ANOTHER_GOOD_NAME](#req_with_another_good_name-another-requirement "Another Requirement")
        Reference: [tests/data/reference_without_title/upper_readable.md:11](../tests/data/reference_without_title/upper_readable.md?plain=1#L11)

Downwards Tracing:

### COV_WITH_GOOD_NAME: A Coverage

Origin: Lower Artefact using readable IDs, [tests/data/reference_without_title/lower_readable.md:5](../tests/data/reference_without_title/lower_readable.md?plain=1#L5)

Upwards Tracing:
*   Upper Artefact using readable IDs => [Lower Artefact using readable IDs]
    *   [REQ_WITH_GOOD_NAME](#req_with_good_name-a-requirement "A Requirement")
        Reference: [tests/data/reference_without_title/lower_readable.md:6](../tests/data/reference_without_title/lower_readable.md?plain=1#L6)

Downwards Tracing:
## Upper Artefact using numeric IDs

### REQ_NUM_1: A Requirement

Origin: Upper Artefact using numeric IDs, [tests/data/reference_without_title/upper_numeric.md:5](../tests/data/reference_without_title/upper_numeric.md?plain=1#L5)

Upwards Tracing:
    *   Derived

Downwards Tracing:
*   Upper Artefact using numeric IDs => [Lower Artefact using numeric IDs]
    *   [COV_NUM_1](#cov_num_1-a-coverage "A Coverage")
        Reference: [tests/data/reference_without_title/lower_numeric.md:8](../tests/data/reference_without_title/lower_numeric.md?plain=1#L8)

### REQ_NUM_2: Another Requirement

Origin: Upper Artefact using numeric IDs, [tests/data/reference_without_title/upper_numeric.md:7](../tests/data/reference_without_title/upper_numeric.md?plain=1#L7)

Upwards Tracing:
    *   Derived

Downwards Tracing:
*   Upper Artefact using numeric IDs => [Lower Artefact using numeric IDs]
    *   [COV_NUM_2](#cov_num_2-another-coverage "Another Coverage")
        Reference: [tests/data/reference_without_title/upper_numeric.md:10](../tests/data/reference_without_title/upper_numeric.md?plain=1#L10)
## Upper Artefact using readable IDs

### REQ_WITH_ANOTHER_GOOD_NAME: Another Requirement

Origin: Upper Artefact using readable IDs, [tests/data/reference_without_title/upper_readable.md:9](../tests/data/reference_without_title/upper_readable.md?plain=1#L9)

Upwards Tracing:
    *   Derived

Downwards Tracing:
*   Upper Artefact using readable IDs => [Lower Artefact using readable IDs]
    *   [COV_WITH_ANOTHER_GOOD_NAME](#cov_with_another_good_name-another-coverage "Another Coverage")
        Reference: [tests/data/reference_without_title/upper_readable.md:11](../tests/data/reference_without_title/upper_readable.md?plain=1#L11)

### REQ_WITH_GOOD_NAME: A Requirement

Origin: Upper Artefact using readable IDs, [tests/data/reference_without_title/upper_readable.md:7](../tests/data/reference_without_title/upper_readable.md?plain=1#L7)

Upwards Tracing:
    *   Derived

Downwards Tracing:
*   Upper Artefact using readable IDs => [Lower Artefact using readable IDs]
    *   [COV_WITH_GOOD_NAME](#cov_with_good_name-a-coverage "A Coverage")
        Reference: [tests/data/reference_without_title/lower_readable.md:6](../tests/data/reference_without_title/lower_readable.md?plain=1#L6)
