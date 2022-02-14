use std::path::PathBuf;

use crate::errors::Error;

pub fn glob_paths(paths: &Vec<String>) -> Result<Vec<PathBuf>, Error> {
    let mut result = Vec::new();

    for path in paths {
        let glob = glob::glob(path);
        let glob = glob.map_err(|e| Error::ConfigError(format!("can not glob {path:?}: {e:?}")))?;
        for path in glob {
            let path = path.map_err(|e| Error::IoError(e.path().into(), e.into_error()))?;
            result.push(path);
        }
    }

    Ok(result)
}

pub mod lazy {
    use std::cell::UnsafeCell;
    use std::intrinsics::transmute;
    use std::mem;

    #[derive(Debug)]
    enum LazyState<T, F, D> {
        NotInit(F, D),
        UnderConstruction,
        Init(T),
    }

    #[derive(Debug)]
    pub struct Lazy<T, D> {
        state: UnsafeCell<LazyState<T, fn(D) -> T, D>>,
    }

    impl<T, D> Lazy<T, D> {
        pub fn new(f: fn(D) -> T, d: D) -> Self {
            Self {
                state: UnsafeCell::new(LazyState::NotInit(f, d)),
            }
        }
    }

    impl<T, D> std::ops::Deref for Lazy<T, D> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            let state: *mut LazyState<_, _, _> = self.state.get();
            let state: &mut LazyState<T, fn(D) -> T, D> = unsafe {
                // SAFETY: this function is the only that can hand out references into state, and
                // it does so only when state == LazyState::Init in which case state is no longer
                // mutated.
                transmute(state)
            };

            if let LazyState::Init(t) = state {
                return t;
            }

            let old = mem::replace(state, LazyState::UnderConstruction);

            if let LazyState::NotInit(func, data) = old {
                let new = func(data);
                *state = LazyState::Init(new);
                if let LazyState::Init(t) = state {
                    return t;
                }
            }
            unreachable!("state was not Init and not NotInit");
        }
    }
}
