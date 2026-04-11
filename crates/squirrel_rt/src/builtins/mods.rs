/// Imports
use crate::{
    builtins::{convert, crypto, env, fs, is, math, mem, process, time},
    refs::MutRef,
    rt::value::Module,
};
use std::{cell::RefCell, collections::HashMap};

/// Provides modules
pub fn provide_modules() -> HashMap<String, MutRef<Module>> {
    HashMap::from([
        (
            "math".to_string(),
            MutRef::new(RefCell::new(Module {
                env: math::provide_env(),
            })),
        ),
        (
            "is".to_string(),
            MutRef::new(RefCell::new(Module {
                env: is::provide_env(),
            })),
        ),
        (
            "env".to_string(),
            MutRef::new(RefCell::new(Module {
                env: env::provide_env(),
            })),
        ),
        (
            "process".to_string(),
            MutRef::new(RefCell::new(Module {
                env: process::provide_env(),
            })),
        ),
        (
            "mem".to_string(),
            MutRef::new(RefCell::new(Module {
                env: mem::provide_env(),
            })),
        ),
        (
            "crypto".to_string(),
            MutRef::new(RefCell::new(Module {
                env: crypto::provide_env(),
            })),
        ),
        (
            "fs".to_string(),
            MutRef::new(RefCell::new(Module {
                env: fs::provide_env(),
            })),
        ),
        (
            "convert".to_string(),
            MutRef::new(RefCell::new(Module {
                env: convert::provide_env(),
            })),
        ),
        (
            "time".to_string(),
            MutRef::new(RefCell::new(Module {
                env: time::provide_env(),
            })),
        ),
    ])
}
