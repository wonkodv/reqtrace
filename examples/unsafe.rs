fn test_unsafe() {
    // If this file is parsed as a `Rust Unsafe` Artefact, the following line
    // leads to Requirement with the ID `UNSAFE_EXAMPLE`
    let x = unsafe /* UNSAFE_EXAMPLE */ { 42 };
}
