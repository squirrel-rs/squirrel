/// Imports
use crate::{
    atom::{AssignOp, Function, TraitFunction},
    expr::Expression,
};
use geko_lex::token::Span;

/// Usage
#[derive(Debug, Clone)]
pub enum UsageKind {
    // As `name`
    As(String),
    // For `items`
    For(Vec<String>),
    // For every item
    All,
    // Just import
    Just,
}

/// Statement
#[derive(Debug, Clone)]
pub enum Statement {
    // While statement
    While {
        span: Span,
        condition: Expression,
        block: Block,
    },
    // If statement
    If {
        span: Span,
        condition: Expression,
        then: Block,
        else_: Option<Box<Statement>>,
    },
    // For statement
    For {
        span: Span,
        var: String,
        iterable: Expression,
        block: Block,
    },
    // Class declaration
    Class {
        span: Span,
        name_span: Span,
        name: String,
        methods: Vec<Function>,
    },
    // Enum declaration
    Enum {
        span: Span,
        name: String,
        variants: Vec<String>,
    },
    // Trait declaration
    Trait {
        span: Span,
        name: String,
        functions: Vec<TraitFunction>,
    },
    // Function declaration
    Function(Function),
    // Assignment declaration
    Assign {
        span: Span,
        name: String,
        op: AssignOp,
        value: Expression,
    },
    // Field assignment declaration
    Set {
        span: Span,
        container: Expression,
        name: String,
        op: AssignOp,
        value: Expression,
    },
    // Return statement
    Return {
        span: Span,
        expr: Option<Expression>,
    },
    // Continue statement
    Continue(Span),
    // Break statement
    Break(Span),
    // Expr
    Expr(Expression),
    // Block
    Block(Box<Block>),
    // Use statement
    Use {
        span: Span,
        path: String,
        kind: UsageKind,
    },
    // Bail statement
    Bail {
        span: Span,
        message: Expression,
    },
}

/// Represents block
#[derive(Debug, Clone)]
pub struct Block {
    pub span: Span,
    pub statements: Vec<Statement>,
}
