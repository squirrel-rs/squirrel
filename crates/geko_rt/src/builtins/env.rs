/// Imports
use crate::refs::MutRef;
use crate::{
    builtins::utils::error,
    refs::{RealmRef, Ref},
    rt::{
        realm::Realm,
        value::{Callable, Native, Value},
    },
};
use geko_common::bug;
use std::{cell::RefCell, rc::Rc};

/// Set var definition
pub fn set_var() -> Ref<Native> {
    Ref::new(Native {
        arity: 2,
        function: Box::new(|_, _, values| {
            let key = values.first().map(|v| v.to_string()).unwrap_or_default();
            if !key.is_empty() {
                // Safety: setting variable is safe because of single-threaded runtime
                unsafe {
                    std::env::set_var(
                        key,
                        values.get(1).map(|v| v.to_string()).unwrap_or_default(),
                    )
                };
            }
            Value::Null
        }),
    })
}

/// Get var definition
pub fn get_var() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| {
            match std::env::var(values.first().map(|v| v.to_string()).unwrap_or_default()) {
                Ok(val) => Value::String(val),
                Err(_) => Value::Null,
            }
        }),
    })
}

/// Unset definition
pub fn unset() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| unsafe {
            std::env::remove_var(values.first().unwrap().to_string());
            Value::Null
        }),
    })
}

/// Var definition
pub fn var() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            match std::env::var_os(values.first().map(|v| v.to_string()).unwrap_or_default()) {
                Some(val) => Value::String(val.to_string_lossy().into_owned()),
                None => error(span, "os variable is not set"),
            }
        }),
    })
}

/// Current workind directory definition
pub fn cwd() -> Ref<Native> {
    Ref::new(Native {
        arity: 0,
        function: Box::new(|_, span, _| match std::env::current_dir() {
            Ok(path) => Value::String(path.to_string_lossy().into_owned()),
            Err(_) => error(span, "failed to get current work directory"),
        }),
    })
}

/// Home directory definition
pub fn home() -> Ref<Native> {
    Ref::new(Native {
        arity: 0,
        function: Box::new(|_, span, _| match std::env::home_dir() {
            Some(path) => Value::String(path.to_string_lossy().into_owned()),
            None => error(span, "could not determine home directory"),
        }),
    })
}

/// Command line arguments
pub fn args() -> Ref<Native> {
    Ref::new(Native {
        arity: 0,
        function: Box::new(|rt, span, _| {
            // Retrieving list class
            let list_class = rt
                .builtins
                .env
                .borrow()
                .lookup("List")
                .unwrap_or_else(|| error(span, "list builtin is not found"));

            // Instantiating list instance
            match list_class {
                Value::Class(list_ty) => match rt.call_class(span, Vec::new(), list_ty) {
                    Ok(val) => match val {
                        // Setting up internal vector
                        Value::Instance(list) => {
                            list.borrow_mut().fields.insert(
                                "$internal".to_string(),
                                Value::Any(MutRef::new(RefCell::new(
                                    std::env::args().map(Value::String).collect::<Vec<Value>>(),
                                ))),
                            );
                            Value::Instance(list)
                        }
                        _ => bug!("`call_class` returned non-instance value"),
                    },
                    Err(_) => bug!("control flow leak"),
                },
                _ => error(span, "list builtin is not a class"),
            }
        }),
    })
}

/// Provides `env` module env
pub fn provide_env() -> RealmRef {
    let mut env = Realm::default();

    env.define("set_var", Value::Callable(Callable::Native(set_var())));
    env.define("get_var", Value::Callable(Callable::Native(get_var())));
    env.define("unset", Value::Callable(Callable::Native(unset())));
    env.define("var", Value::Callable(Callable::Native(var())));
    env.define("cwd", Value::Callable(Callable::Native(cwd())));
    env.define("home", Value::Callable(Callable::Native(home())));
    env.define("args", Value::Callable(Callable::Native(args())));
    env.define("arch", Value::String(std::env::consts::ARCH.to_string()));
    env.define("os", Value::String(std::env::consts::OS.to_string()));
    env.define(
        "family",
        Value::String(std::env::consts::FAMILY.to_string()),
    );
    env.define(
        "dll",
        Value::String(std::env::consts::DLL_EXTENSION.to_string()),
    );
    env.define(
        "exe",
        Value::String(std::env::consts::EXE_EXTENSION.to_string()),
    );

    Rc::new(RefCell::new(env))
}
