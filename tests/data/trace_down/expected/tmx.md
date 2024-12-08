

# Requirements

## Lower

### REQ_LOWER: The Lower Requirement

Origin: Lower, [tests/data/trace_down/lower.md:2](../tests/data/trace_down/lower.md?plain=1#L2)

Upwards Tracing:
*   Upper => [Lower]
    *   [REQ_UPPER](#req_upper-the-upper-requirement "The Upper Requirement")
        Reference: [tests/data/trace_down/upper.md:2](../tests/data/trace_down/upper.md?plain=1#L2)

Downwards Tracing:
## Upper

### REQ_UPPER: The Upper Requirement

Origin: Upper, [tests/data/trace_down/upper.md:1](../tests/data/trace_down/upper.md?plain=1#L1)

Upwards Tracing:
    *   Derived

Downwards Tracing:
*   Upper => [Lower]
    *   [REQ_LOWER](#req_lower-the-lower-requirement "The Lower Requirement")
        Reference: [tests/data/trace_down/upper.md:2](../tests/data/trace_down/upper.md?plain=1#L2)
