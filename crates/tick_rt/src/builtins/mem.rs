/// Imports
use crate::{
    refs::{EnvRef, Ref},
    rt::env::Environment,
    rt::value::{Callable, Native, Value},
};
use std::{cell::RefCell, rc::Rc};
use sysinfo::System;

/// Total memory
fn total() -> Ref<Native> {
    return Ref::new(Native {
        arity: 0,
        function: Box::new(|_, _, _| {
            let mut sys = System::new();
            sys.refresh_memory();
            Value::Int(sys.total_memory() as i64)
        }),
    });
}

/// Memory usage
fn used() -> Ref<Native> {
    return Ref::new(Native {
        arity: 0,
        function: Box::new(|_, _, _| {
            let mut sys = System::new();
            sys.refresh_memory();
            Value::Int(sys.used_memory() as i64)
        }),
    });
}

/// Free memory
fn free() -> Ref<Native> {
    return Ref::new(Native {
        arity: 0,
        function: Box::new(|_, _, _| {
            let mut sys = System::new();
            sys.refresh_memory();
            Value::Int(sys.free_memory() as i64)
        }),
    });
}

/// Free swapp
fn total_swap() -> Ref<Native> {
    return Ref::new(Native {
        arity: 0,
        function: Box::new(|_, _, _| {
            let mut sys = System::new();
            sys.refresh_memory();
            Value::Int(sys.total_swap() as i64)
        }),
    });
}

/// Swap usage
fn used_swap() -> Ref<Native> {
    return Ref::new(Native {
        arity: 0,
        function: Box::new(|_, _, _| {
            let mut sys = System::new();
            sys.refresh_memory();
            Value::Int(sys.used_swap() as i64)
        }),
    });
}

/// Free swap
fn free_swap() -> Ref<Native> {
    return Ref::new(Native {
        arity: 0,
        function: Box::new(|_, _, _| {
            let mut sys = System::new();
            sys.refresh_memory();
            Value::Int(sys.free_swap() as i64)
        }),
    });
}

/// Size of
fn size_of() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| {
            Value::Int(std::mem::size_of_val(&values.get(0).cloned().unwrap()) as i64)
        }),
    });
}

/// Align of
fn align_of() -> Ref<Native> {
    return Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| {
            Value::Int(std::mem::align_of_val(&values.get(0).cloned().unwrap()) as i64)
        }),
    });
}

/// Provides `mem` module env
pub fn provide_env() -> EnvRef {
    let mut env = Environment::default();

    env.force_define("total", Value::Callable(Callable::Native(total())));
    env.force_define("free", Value::Callable(Callable::Native(free())));
    env.force_define("used", Value::Callable(Callable::Native(used())));
    env.force_define(
        "total_swap",
        Value::Callable(Callable::Native(total_swap())),
    );
    env.force_define("used_swap", Value::Callable(Callable::Native(used_swap())));
    env.force_define("free_swap", Value::Callable(Callable::Native(free_swap())));
    env.force_define("size_of", Value::Callable(Callable::Native(size_of())));
    env.force_define("align_of", Value::Callable(Callable::Native(align_of())));

    Rc::new(RefCell::new(env))
}
