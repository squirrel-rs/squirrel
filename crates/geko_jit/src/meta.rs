/// Imports
use cranelift::prelude;

/// Represents type used during code generation
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Typ {
    Int,
    Float,
    Bool,
}

/// Represents variable used during code generation
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Variable {
    pub variable: prelude::Variable,
    pub typ: Typ,
}

/// Variable implementation
impl Variable {
    /// Creates new signature
    pub fn new(variable: prelude::Variable, typ: Typ) -> Self {
        Self { variable, typ }
    }
}

/// Represents function parameter
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Parameter {
    pub name: String,
    pub typ: Typ,
}

/// Represents function signature
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Signature {
    pub name: String,
    pub params: Vec<Parameter>,
    pub ret: Option<Typ>,
}

/// Signature implementation
impl Signature {
    /// Creates new signature
    pub fn new(name: &str, params: Vec<Parameter>, ret: Option<Typ>) -> Self {
        Self {
            name: name.to_string(),
            params,
            ret,
        }
    }
}

// Represents jit value used for args and return type
// pub union Value {
// u8: u8,
// i64: i64,
// f64: f64,
// }
