/// Imports
use crate::{
    builtins::utils,
    refs::{RealmRef, Ref},
    rt::{
        realm::Realm,
        value::{Callable, Native, Value},
    },
};
use rand::RngExt;
use std::{
    cell::RefCell,
    f64::consts::{
        E, FRAC_1_PI, FRAC_1_SQRT_2, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, LN_2,
        LN_10, LOG2_10, LOG2_E, LOG10_2, LOG10_E, PI, SQRT_2, TAU,
    },
    rc::Rc,
};

/// Math sin
fn sin() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => Value::Float(f64::sin(*int as f64)),
            Value::Float(float) => Value::Float(f64::sin(*float)),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math sinh
fn sinh() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => Value::Float(f64::sinh(*int as f64)),
            Value::Float(float) => Value::Float(f64::sinh(*float)),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math cos
fn cos() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => Value::Float(f64::cos(*int as f64)),
            Value::Float(float) => Value::Float(f64::cos(*float)),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math cosh
fn cosh() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => Value::Float(f64::cosh(*int as f64)),
            Value::Float(float) => Value::Float(f64::cosh(*float)),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math asin
fn asin() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => Value::Float(f64::asin(*int as f64)),
            Value::Float(float) => Value::Float(f64::asin(*float)),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math asinh
fn asinh() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => Value::Float(f64::asinh(*int as f64)),
            Value::Float(float) => Value::Float(f64::asinh(*float)),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math acos
fn acos() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => Value::Float(f64::acos(*int as f64)),
            Value::Float(float) => Value::Float(f64::acos(*float)),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math acosh
fn acosh() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => Value::Float(f64::acosh(*int as f64)),
            Value::Float(float) => Value::Float(f64::acosh(*float)),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math atan
fn atg() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => Value::Float(f64::atan(*int as f64)),
            Value::Float(float) => Value::Float(f64::atan(*float)),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math atan 2
fn atg2() -> Ref<Native> {
    Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            match (values.first().unwrap(), values.get(1).unwrap()) {
                (Value::Int(x), Value::Int(y)) => Value::Float(f64::atan2(*y as f64, *x as f64)),
                (Value::Int(x), Value::Float(y)) => Value::Float(f64::atan2(*y, *x as f64)),
                (Value::Float(x), Value::Int(y)) => Value::Float(f64::atan2(*y as f64, *x)),
                (Value::Float(x), Value::Float(y)) => Value::Float(f64::atan2(*y, *x)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }),
    })
}

/// Math tg
fn tg() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => Value::Float(f64::tan(*int as f64)),
            Value::Float(float) => Value::Float(f64::tan(*float)),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math tgh
fn tgh() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => Value::Float(f64::tanh(*int as f64)),
            Value::Float(float) => Value::Float(f64::tanh(*float)),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

// Math ctg
fn ctg() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => Value::Float(1.0 / f64::tan(*int as f64)),
            Value::Float(float) => Value::Float(1.0 / f64::tan(*float)),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

// Math ctgh
fn ctgh() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => Value::Float(1.0 / f64::tanh(*int as f64)),
            Value::Float(float) => Value::Float(1.0 / f64::tanh(*float)),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math sqrt
fn sqrt() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => Value::Float(f64::sqrt(*int as f64)),
            Value::Float(float) => Value::Float(f64::sqrt(*float)),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math cbrt
fn cbrt() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => Value::Float(f64::cbrt(*int as f64)),
            Value::Float(float) => Value::Float(f64::cbrt(*float)),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math log
fn log() -> Ref<Native> {
    Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            let a = match values.first().unwrap() {
                Value::Int(i) => *i as f64,
                Value::Float(f) => *f,
                _ => utils::error(span, "argument is expected to be a number"),
            };
            let b = match values.get(1).unwrap() {
                Value::Int(i) => *i as f64,
                Value::Float(f) => *f,
                _ => utils::error(span, "argument is expected to be a number"),
            };
            Value::Float(a.log(b))
        }),
    })
}

/// Math min
fn min() -> Ref<Native> {
    Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            // Int min
            Value::Int(a) => match values.get(1).unwrap() {
                Value::Int(b) => Value::Int(*a.min(b)),
                Value::Float(b) => Value::Float((*a as f64).min(*b)),
                _ => utils::error(span, "argument is expected to be a number"),
            },
            // Float min
            Value::Float(a) => match values.get(1).unwrap() {
                Value::Int(b) => Value::Float(a.min(*b as f64)),
                Value::Float(b) => Value::Float(a.min(*b)),
                _ => utils::error(span, "argument is expected to be a number"),
            },
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math max
fn max() -> Ref<Native> {
    Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            // Int max
            Value::Int(a) => match values.get(1).unwrap() {
                Value::Int(b) => Value::Int(*a.max(b)),
                Value::Float(b) => Value::Float((*a as f64).max(*b)),
                _ => utils::error(span, "argument is expected to be a number"),
            },
            // Float max
            Value::Float(a) => match values.get(1).unwrap() {
                Value::Int(b) => Value::Float(a.max(*b as f64)),
                Value::Float(b) => Value::Float(a.max(*b)),
                _ => utils::error(span, "argument is expected to be a number"),
            },
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math clamp
fn clamp() -> Ref<Native> {
    Ref::new(Native {
        arity: 3,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            // Int clamp
            Value::Int(x) => match (values.get(1).unwrap(), values.get(2).unwrap()) {
                (Value::Int(a), Value::Int(b)) => {
                    if a > b {
                        utils::error(span, "clamp: min must be <= max")
                    }
                    Value::Int(*x.clamp(a, b))
                }
                (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => {
                    let (min, max) = (*a as f64, *b);
                    if min > max {
                        utils::error(span, "clamp: min must be <= max")
                    }
                    Value::Float((*x as f64).clamp(min, max))
                }
                (Value::Float(a), Value::Float(b)) => {
                    if a > b {
                        utils::error(span, "clamp: min must be <= max")
                    }
                    Value::Float((*x as f64).clamp(*a, *b))
                }
                _ => utils::error(span, "argument is expected to be a number"),
            },
            // Float clamp
            Value::Float(x) => match (values.get(1).unwrap(), values.get(2).unwrap()) {
                (Value::Int(a), Value::Int(b)) => {
                    if a > b {
                        utils::error(span, "clamp: min must be <= max")
                    }
                    Value::Float(x.clamp(*a as f64, *b as f64))
                }
                (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => {
                    let (min, max) = (*a as f64, *b);
                    if min > max {
                        utils::error(span, "clamp: min must be <= max")
                    }
                    Value::Float(x.clamp(min, max))
                }
                (Value::Float(a), Value::Float(b)) => {
                    if a > b {
                        utils::error(span, "clamp: min must be <= max")
                    }
                    Value::Float(x.clamp(*a, *b))
                }
                _ => utils::error(span, "argument is expected to be a number"),
            },
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math log2
fn log2() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => Value::Float(f64::log2(*int as f64)),
            Value::Float(float) => Value::Float(f64::log2(*float)),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math log10
fn log10() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => Value::Float(f64::log10(*int as f64)),
            Value::Float(float) => Value::Float(f64::log10(*float)),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math exp
fn exp() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => Value::Float(f64::exp(*int as f64)),
            Value::Float(float) => Value::Float(f64::exp(*float)),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math abs
fn abs() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Int(int) => match int.checked_abs() {
                Some(result) => Value::Int(result),
                None => utils::error(span, "int overflow in abs"),
            },
            Value::Float(float) => Value::Float(float.abs()),
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math floor
fn floor() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Float(float) => Value::Float(float.floor()),
            _ => utils::error(span, "argument is expected to be a float"),
        }),
    })
}

/// Math ceil
fn ceil() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Float(float) => Value::Float(float.ceil()),
            _ => utils::error(span, "argument is expected to be a float"),
        }),
    })
}

/// Math trunc
fn trunc() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Float(float) => Value::Float(float.trunc()),
            _ => utils::error(span, "argument is expected to be a float"),
        }),
    })
}

/// Math round
fn round() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            Value::Float(float) => Value::Float(float.round()),
            _ => utils::error(span, "argument is expected to be a float"),
        }),
    })
}

/// Math pow
fn pow() -> Ref<Native> {
    Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| match values.first().unwrap() {
            // Int pow
            Value::Int(a) => match values.get(1).unwrap() {
                // Int exp
                Value::Int(b) => {
                    use std::convert::TryInto;

                    // Positive exponent
                    if *b >= 0 {
                        // Safe cast
                        let b_u32: u32 = (*b).try_into().unwrap_or_else(|_| {
                            utils::error(span, &format!("exponent {} is too large", b))
                        });

                        match a.checked_pow(b_u32) {
                            Some(result) => Value::Int(result),
                            None => utils::error(span, "int overflow in exp"),
                        }
                    }
                    // Negative exponent
                    else {
                        // Safe cast
                        let b_i32: i32 = (*b).try_into().unwrap_or_else(|_| {
                            utils::error(span, &format!("exponent {} is too small", b))
                        });

                        Value::Float((*a as f64).powi(b_i32))
                    }
                }
                // Float exp
                Value::Float(b) => Value::Float((*a as f64).powf(*b)),
                // Otherwise, raising error
                _ => utils::error(span, "argument is expected to be a number"),
            },
            // Float pow
            Value::Float(a) => match values.get(1).unwrap() {
                // Int exp
                Value::Int(b) => Value::Float(a.powi(*b as i32)),
                // Float exp
                Value::Float(b) => Value::Float(a.powf(*b)),
                // Otherwise, raising error
                _ => utils::error(span, "argument is expected to be a number"),
            },
            _ => utils::error(span, "argument is expected to be a number"),
        }),
    })
}

/// Math hypot
fn hypot() -> Ref<Native> {
    Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            match (values.first().unwrap(), values.get(1).unwrap()) {
                (Value::Int(x), Value::Int(y)) => Value::Float(f64::hypot(*x as f64, *y as f64)),
                (Value::Int(x), Value::Float(y)) => Value::Float(f64::hypot(*x as f64, *y)),
                (Value::Float(x), Value::Int(y)) => Value::Float(f64::hypot(*x, *y as f64)),
                (Value::Float(x), Value::Float(y)) => Value::Float(f64::hypot(*x, *y)),
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }),
    })
}

/// Random numner
fn rnd() -> Ref<Native> {
    Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            match (values.first().unwrap(), values.get(1).unwrap()) {
                (Value::Int(x), Value::Int(y)) => Value::Int(rand::rng().random_range(*x..*y)),
                (Value::Int(x), Value::Float(y)) => {
                    Value::Float(rand::rng().random_range((*x as f64)..*y))
                }
                (Value::Float(x), Value::Int(y)) => {
                    Value::Float(rand::rng().random_range(*x..(*y as f64)))
                }
                (Value::Float(x), Value::Float(y)) => {
                    Value::Float(rand::rng().random_range(*x..*y))
                }
                _ => utils::error(span, "argument is expected to be a number"),
            }
        }),
    })
}

/// Provides `math` module env
pub fn provide_env() -> RealmRef {
    let mut env = Realm::default();

    env.define("log", Value::Callable(Callable::Native(log())));
    env.define("log2", Value::Callable(Callable::Native(log2())));
    env.define("log10", Value::Callable(Callable::Native(log10())));
    env.define("exp", Value::Callable(Callable::Native(exp())));
    env.define("abs", Value::Callable(Callable::Native(abs())));
    env.define("floor", Value::Callable(Callable::Native(floor())));
    env.define("ceil", Value::Callable(Callable::Native(ceil())));
    env.define("trunc", Value::Callable(Callable::Native(trunc())));
    env.define("round", Value::Callable(Callable::Native(round())));
    env.define("sin", Value::Callable(Callable::Native(sin())));
    env.define("cos", Value::Callable(Callable::Native(cos())));
    env.define("asin", Value::Callable(Callable::Native(asin())));
    env.define("acos", Value::Callable(Callable::Native(acos())));
    env.define("sinh", Value::Callable(Callable::Native(sinh())));
    env.define("cosh", Value::Callable(Callable::Native(cosh())));
    env.define("asinh", Value::Callable(Callable::Native(asinh())));
    env.define("acosh", Value::Callable(Callable::Native(acosh())));
    env.define("tg", Value::Callable(Callable::Native(tg())));
    env.define("tgh", Value::Callable(Callable::Native(tgh())));
    env.define("ctg", Value::Callable(Callable::Native(ctg())));
    env.define("ctgh", Value::Callable(Callable::Native(ctgh())));
    env.define("atg", Value::Callable(Callable::Native(atg())));
    env.define("atg2", Value::Callable(Callable::Native(atg2())));
    env.define("sqrt", Value::Callable(Callable::Native(sqrt())));
    env.define("cbrt", Value::Callable(Callable::Native(cbrt())));
    env.define("pow", Value::Callable(Callable::Native(pow())));
    env.define("hypot", Value::Callable(Callable::Native(hypot())));
    env.define("min", Value::Callable(Callable::Native(min())));
    env.define("max", Value::Callable(Callable::Native(max())));
    env.define("clamp", Value::Callable(Callable::Native(clamp())));
    env.define("rnd", Value::Callable(Callable::Native(rnd())));
    env.define("pi", Value::Float(PI));
    env.define("tau", Value::Float(TAU));
    env.define("frac_pi_2", Value::Float(FRAC_PI_2));
    env.define("frac_pi_3", Value::Float(FRAC_PI_3));
    env.define("frac_pi_4", Value::Float(FRAC_PI_4));
    env.define("frac_pi_6", Value::Float(FRAC_PI_6));
    env.define("frac_pi_8", Value::Float(FRAC_PI_8));
    env.define("frac_1_pi", Value::Float(FRAC_1_PI));
    env.define("frac_1_sqrt_2", Value::Float(FRAC_1_SQRT_2));
    env.define("sqrt_2", Value::Float(SQRT_2));
    env.define("ln2", Value::Float(LN_2));
    env.define("ln10", Value::Float(LN_10));
    env.define("log10_e", Value::Float(LOG10_E));
    env.define("log10_2", Value::Float(LOG10_2));
    env.define("log2_e", Value::Float(LOG2_E));
    env.define("log2_10", Value::Float(LOG2_10));
    env.define("e", Value::Float(E));

    Rc::new(RefCell::new(env))
}
