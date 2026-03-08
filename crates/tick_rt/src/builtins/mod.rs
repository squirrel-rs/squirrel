/// Modules
mod core;
mod env;
mod is;
mod list;
mod math;
mod mem;
mod mods;
mod process;
mod utils;
mod crypto;

/// Imports
use crate::{
    refs::{EnvRef, MutRef},
    rt::value::Module,
};
use std::collections::HashMap;

/// Built-in definitions
pub(crate) struct Builtins {
    /// Built-in variables and functions (print, println, readln, etc.)
    pub(crate) env: EnvRef,
    /// Built-in modules
    pub(crate) modules: HashMap<String, MutRef<Module>>,
}

/// Provides builtins
pub fn provide_builtins() -> Builtins {
    Builtins {
        env: core::provide_env(),
        modules: mods::provide_modules(),
    }
}
