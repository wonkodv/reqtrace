# Unsafe



## SAFE_STRING_VAULT_SET: String Vault Mutability

`StringVault.keep` and `StringVault.keep_cloned` take an immutable reference and access its `set` attribute mutably.

Needed so that there can be many `&str` which borrow `StringVault`.

Safe because the set is behind an `UnsafeCell`, marking it as `!Sync` so that
only one thread can reference a `StringVault` at a time.

Covers:
*   UNSAFE_StringVault_keep_set
*   UNSAFE_StringVault_keep_cloned_set

## SAFE_STRING_VAULT_LIFETIME: String Vault Return Value Life time

`StringVault.keep` and `StringVault.keep_cloned` `transmute` the lifetime of
a `&str` from the local function to that of the `&StringVault`.

Needed so that `&str` references can be returned

Safe because the `String` which manages the memory is kept living as long as the
StringVault.

Covers:
*   UNSAFE_StringVault_keep_old
*   UNSAFE_StringVault_keep_new
*   UNSAFE_StringVault_keep_cloned_old
*   UNSAFE_StringVault_keep_cloned_new
