use std::{cell::UnsafeCell, collections::HashSet};

/// Place to put strings so you can point to them from various places.
/// The `&str` live as long as the StringVault.
///
/// # Examples
/// ```rust
/// let sv = StringVault::new();
/// let s:&str = sv.keep("Hello World");
///
/// sv.keep(s); // moves s into keep
///
/// println!("{}", s);
/// ```
///
/// ```compile_fail
/// let sv = StringVault::new();
/// let s:&str = sv.keep("Hello World");
///
/// sv.keep(s);
///
/// std::mem::drop(sv);
///
/// println!("{}", s); // s can not outlive sv
/// ```
///
pub struct StringVault {
    set: UnsafeCell<HashSet<String>>,
}

impl StringVault {
    /// Create a new empty StringVault
    pub fn new() -> Self {
        let set: UnsafeCell<_> = HashSet::new().into();
        Self { set }
    }

    /// Move a string into the StringVault.
    ///
    /// If an equal string already exists, the new one is droped, otherwise
    /// the new one is kept.
    /// returns a `&str` reference to the interned string
    pub fn keep<'a>(&'a self, val: String) -> &'a str {
        let set = self.set.get();
        let set = unsafe /* UNSAFE_StringVault_keep_set */ {
            set.as_mut()
        };
        let set = set.unwrap();
        let interned;
        if let Some(old) = set.get(&val) {
            unsafe /* UNSAFE_StringVault_keep_old */ {
                interned = std::mem::transmute::<&'_ str, &'a str>(old.as_str());
            }
        } else {
            unsafe /* UNSAFE_StringVault_keep_new */ {
                interned = std::mem::transmute::<&'_ str, &'a str>(val.as_str());
            }
            set.insert(val);
        }
        return interned;
    }

    /// Find an equal string or create a clone and keep it.
    ///
    /// if no equal string exists, a new `String` is created and kept.
    ///
    /// Returns a `&str` reference to the interned string
    pub fn keep_cloned<'a, S: AsRef<str>>(&'a self, val: S) -> &'a str {
        let set = self.set.get();
        let set = unsafe /* UNSAFE_StringVault_keep_cloned_set */ {
            set.as_mut()
        };
        let set = set.unwrap();

        let val: &str = val.as_ref();

        let interned;
        if let Some(old) = set.get(val) {
            unsafe /* UNSAFE_StringVault_keep_cloned_old */ {
                interned = std::mem::transmute::<&'_ str, &'a str>(old.as_str());
            }
        } else {
            let val: String = String::from(val);
            unsafe /* UNSAFE_StringVault_keep_cloned_new */ {
                interned = std::mem::transmute::<&'_ str, &'a str>(val.as_str());
            }
            set.insert(val);
        }
        return interned;
    }
}

/// add `as_interned(&sv)` to `String`
trait AsInterned {
    fn as_interned<'a>(self, sv: &'a StringVault) -> &'a str;
}

impl AsInterned for String {
    fn as_interned<'a>(self, sv: &'a StringVault) -> &'a str {
        sv.keep(self)
    }
}

/// add `as_interned(&sv)` to anything that can be referenced as `&str`
trait AsInternedClone {
    fn as_interned<'a>(&self, sv: &'a StringVault) -> &'a str;
}

impl<S: AsRef<str>> AsInternedClone for S {
    fn as_interned<'a>(&self, sv: &'a StringVault) -> &'a str {
        sv.keep_cloned(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_string_vault() {
        let sv = &StringVault::new();

        let s1 = "Hello World".to_string();
        let s2 = s1.clone();

        let p1 = s1.as_ptr() as usize;
        let p2 = s2.as_ptr() as usize;

        let si1 = sv.keep(s1);
        let si2 = sv.keep(s2);

        let pi1 = si1.as_ptr() as usize;
        let pi2 = si2.as_ptr() as usize;

        assert!(p1 != p2);
        assert!(pi1 == pi2);
        assert!(p1 == pi1);
    }

    #[test]
    fn test_str_trait() {
        let sv = StringVault::new();

        let _: &str = "Hello World".as_interned(&sv);
    }

    #[test]
    fn test_string_trait() {
        let sv = StringVault::new();
        let s = String::from("Hello World");

        let _: &str = s.as_interned(&sv);
    }
}
