fn main() {
    // If this file is parsed as a `Rust Unsafe` Artefact, the following line
    // leads to Requirement with the ID `UNSAFE_EXAMPLE`
    #[allow(unused_unsafe)]
    let _x = unsafe /* UNSAFE_EXAMPLE */ { 42 };
}
