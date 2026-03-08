/// Imports
use crate::{
    builtins::list,
    refs::{EnvRef, Ref},
    rt::{
        env::Environment,
        value::{Callable, Native, Value},
    },
};
use std::{cell::RefCell, rc::Rc};

/// Print definition
pub fn print() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|rt, _, values| {
            rt.io.output(&values.get(0).unwrap().to_string());
            rt.io.flush();
            Value::Null
        }),
    });
}

/// Println definition
pub fn println() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|rt, _, values| {
            rt.io.output(&format!("{}\n", values.get(0).unwrap()));
            rt.io.flush();
            Value::Null
        }),
    });
}

/// Readln definition
pub fn readln() -> Ref<Native> {
    return Ref::new(Native {
        arity: 0,
        function: Box::new(|rt, _, _| Value::String(rt.io.input())),
    });
}

/// Provides env
pub fn provide_env() -> EnvRef {
    let mut env = Environment::default();

    env.force_define("print", Value::Callable(Callable::Native(print())));
    env.force_define("println", Value::Callable(Callable::Native(println())));
    env.force_define("readln", Value::Callable(Callable::Native(readln())));
    env.force_define("List", Value::Type(list::provide_type()));

    Rc::new(RefCell::new(env))
}
