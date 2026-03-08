/// Imports
use crate::rt::{
    env::Environment,
    value::{Function, Instance, Native, Type, Value},
};
use rust_cc::{Finalize, Trace};

/// Trace implementation for native
unsafe impl Trace for Native {
    fn trace(&self, _: &mut rust_cc::Context<'_>) {}
}
impl Finalize for Native {
    fn finalize(&self) {}
}

/// Trace implementation for native
unsafe impl Trace for Type {
    fn trace(&self, ctx: &mut rust_cc::Context<'_>) {
        for value in self.methods.values() {
            value.trace(ctx);
        }
    }
}
impl Finalize for Type {
    fn finalize(&self) {}
}

/// Trace implementation for function
unsafe impl Trace for Function {
    fn trace(&self, _: &mut rust_cc::Context<'_>) {}
}
impl Finalize for Function {
    fn finalize(&self) {}
}

/// Trace implementation for instance
unsafe impl Trace for Instance {
    fn trace(&self, ctx: &mut rust_cc::Context<'_>) {
        for value in self.fields.values() {
            value.trace(ctx);
        }
    }
}
impl Finalize for Instance {
    fn finalize(&self) {}
}

/// Trace implementation for environment
unsafe impl Trace for Environment {
    fn trace(&self, ctx: &mut rust_cc::Context<'_>) {
        for value in self.variables.values() {
            value.trace(ctx);
        }
        match &self.enclosing {
            Some(env) => env.trace(ctx),
            None => {}
        }
    }
}
impl Finalize for Environment {
    fn finalize(&self) {}
}

/// Trace implementation for value
unsafe impl Trace for Value {
    fn trace(&self, ctx: &mut rust_cc::Context<'_>) {
        match self {
            // No need to trace primitives
            Value::Bool(_) | Value::Int(_) | Value::Float(_) | Value::String(_) | Value::Null => {}
            // Ref-counted objects
            Value::Callable(callable) => callable.trace(ctx),
            Value::Type(cc) => cc.trace(ctx),
            Value::Enum(cc) => cc.trace(ctx),
            Value::Module(cc) => cc.trace(ctx),
            Value::Instance(cc) => cc.trace(ctx),
            // Any can't be properly traced
            Value::Any(_) => {}
        }
    }
}
impl Finalize for Value {
    fn finalize(&self) {}
}
