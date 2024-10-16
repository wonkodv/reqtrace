use std::path::PathBuf;

use crate::errors::Error;

pub fn glob_paths(paths: &Vec<String>) -> Result<Vec<PathBuf>, Error> {
    let mut result = Vec::new();

    for path in paths {
        let glob = glob::glob(path);
        let glob = glob.map_err(|e| Error::Config(format!("can not glob {path:?}: {e:?}")))?;
        for entry in glob {
            let path = entry.map_err(|e| Error::Io(e.path().into(), e.to_string()))?;
            result.push(path);
        }
    }

    Ok(result)
}

pub mod lazy {
    use std::cell::UnsafeCell;

    use std::mem;

    #[derive(Debug)]
    enum LazyState<T, F, D> {
        NotInit(F, D),
        UnderConstruction,
        Init(T),
    }

    type Init<T, D> = fn(D) -> T;
    type State<T, D> = LazyState<T, Init<T, D>, D>;

    /// A Lazy Cell, that produces T from D on first use
    #[derive(Debug)]
    pub struct Lazy<T, D> {
        state: UnsafeCell<State<T, D>>,
    }

    impl<T, D> Lazy<T, D> {
        pub fn new(f: fn(D) -> T, d: D) -> Self {
            Self {
                state: UnsafeCell::new(LazyState::NotInit(f, d)),
            }
        }

        pub fn get(&self) -> &T {
            let state: *mut LazyState<T, fn(D) -> T, D> = self.state.get();
            let state: &mut LazyState<T, fn(D) -> T, D> = {
                // SAFETY: this function is the only that can hand out references into state, and
                // it does so only when state == LazyState::Init in which case state is no longer
                // mutated.
                // Due to the UnsafeCell, Lazy is !Sync, so there can be no data races
                #[allow(unsafe_code)]
                unsafe {
                    &mut *state
                }
            };

            // if t was already produced,  return a reference to it
            if let LazyState::Init(t) = state {
                return t;
            }

            let old = mem::replace(state, LazyState::UnderConstruction);

            let LazyState::NotInit(func, data) = old else {
                unreachable!("state was not Init and not NotInit");
            };
            let new = func(data);
            *state = LazyState::Init(new);
            let LazyState::Init(t) = state else {
                unreachable!("state was not Init, even though we just set it so");
            };
            return t;
        }
    }
}

#[cfg(test)]
mod test {

    use std::sync::atomic::{AtomicU8, Ordering};

    use super::*;

    static SENTINEL: AtomicU8 = AtomicU8::new(0);

    fn produce(input: Vec<u8>) -> String {
        SENTINEL.fetch_add(1, Ordering::Relaxed);
        String::from_utf8(input).unwrap()
    }

    #[test]
    fn test_lazy() {
        SENTINEL.store(0, Ordering::Relaxed);

        let l: lazy::Lazy<String, Vec<u8>> = lazy::Lazy::new(produce, vec![65, 66, 67]);

        assert!(SENTINEL.load(Ordering::Relaxed) == 0);

        let s: &String = &l.get();

        assert!(SENTINEL.load(Ordering::Relaxed) == 1);

        assert!(s.as_str() == "ABC");

        let s: &String = &l.get();

        assert!(SENTINEL.load(Ordering::Relaxed) == 1);

        assert!(s.as_str() == "ABC");
    }
}
