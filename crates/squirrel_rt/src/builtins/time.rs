/// Imports
use crate::{
    builtins::utils,
    interpreter::Interpreter,
    refs::{EnvRef, MutRef, Ref},
    rt::{
        env::Environment,
        value::{Callable, Class, Method, Native, Value},
    },
};
use chrono::{DateTime, Datelike, Duration, Local, NaiveDateTime, Timelike, Utc};
use squirrel_common::bug;
use squirrel_lex::token::Span;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// Helper: creates fresh `Time` with `NaiveTimeDelta`
fn fresh_time(rt: &mut Interpreter, span: &Span, time: NaiveDateTime) -> Value {
    // Searching `Time` class
    let time_class = match rt.builtins.modules.get("time") {
        // Safety: borrow is temporal for the end of function
        Some(module) => match module.borrow().env.borrow().lookup("Time") {
            Some(Value::Class(ty)) => ty,
            _ => utils::error(span, "corrupted module"),
        },
        None => utils::error(span, "corrupted module"),
    };

    // Creating `Time` instance
    match rt.call_class(
        span,
        vec![Value::Any(MutRef::new(RefCell::new(time)))],
        time_class,
    ) {
        Ok(val) => val,
        Err(_) => bug!("control flow leak"),
    }
}

/// Helper: validates time
fn validate_time<F, V>(span: &Span, value: Value, f: F) -> V
where
    F: FnOnce(NaiveDateTime) -> V,
{
    match value {
        Value::Instance(instance) => {
            // Safety: borrow is temporal for this line
            let internal = instance
                .borrow_mut()
                .fields
                .get("$internal")
                .cloned()
                .unwrap();

            match internal {
                // Safety: borrow is temporal and short
                Value::Any(time) => match time.borrow_mut().downcast_mut::<NaiveDateTime>() {
                    Some(time) => f(time.clone()),
                    _ => utils::error(span, "corrupted time"),
                },
                _ => {
                    utils::error(span, "corrupted time");
                }
            }
        }
        _ => unreachable!(),
    }
}

/// Helper: validates time argument
fn validate_time_arg<F, V>(span: &Span, values: &[Value], idx: usize, f: F) -> V
where
    F: FnOnce(NaiveDateTime) -> V,
{
    validate_time(span, values.get(idx).cloned().unwrap(), f)
}

/// Helper: validates one time argument
fn validate_one_time_arg<F, V>(span: &Span, values: &[Value], f: F) -> V
where
    F: FnOnce(NaiveDateTime) -> V,
{
    validate_time(span, values.get(0).cloned().unwrap(), f)
}

/// Helper: validates two time arguments
fn validate_two_time_args<F, V>(span: &Span, values: &[Value], f: F) -> V
where
    F: FnOnce(NaiveDateTime, NaiveDateTime) -> V,
{
    validate_time_arg(span, values, 0, |from| {
        validate_time_arg(span, values, 1, |to| f(from, to))
    })
}

/// `Time` init method
fn time_init_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, _, values| {
            let list = values.first().cloned().unwrap();
            match list {
                Value::Instance(instance) => {
                    // Setting `$internal` field
                    instance
                        .borrow_mut()
                        .fields
                        .insert("$internal".to_string(), values.get(1).cloned().unwrap());

                    Value::Null
                }
                _ => unreachable!(),
            }
        }),
    }))
}

/// `Time` year method
fn time_year_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_time_arg(span, &values, |time| Value::Int(time.year() as i64))
        }),
    }))
}

/// `Time` month method
fn time_month_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_time_arg(span, &values, |time| Value::Int(time.month0() as i64))
        }),
    }))
}

/// `Time` week method
fn time_week_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_time_arg(span, &values, |time| {
                Value::Int(time.iso_week().week() as i64)
            })
        }),
    }))
}

/// `Time` ordinal method
fn time_ordinal_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_time_arg(span, &values, |time| Value::Int(time.ordinal0() as i64))
        }),
    }))
}

/// `Time` day method
fn time_day_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_time_arg(span, &values, |time| Value::Int(time.day0() as i64))
        }),
    }))
}

/// `Time` weekday method
fn time_weekday_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_time_arg(span, &values, |time| {
                Value::String(time.weekday().to_string())
            })
        }),
    }))
}

/// `Time` hour method
fn time_hour_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_time_arg(span, &values, |time| Value::Int(time.hour() as i64))
        }),
    }))
}

/// `Time` minute method
fn time_minute_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_time_arg(span, &values, |time| Value::Int(time.minute() as i64))
        }),
    }))
}

/// `Time` second method
fn time_second_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_time_arg(span, &values, |time| Value::Int(time.second() as i64))
        }),
    }))
}

/// `Time` in seconds method
fn time_in_seconds_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_time_arg(span, &values, |time| {
                Value::Int(time.and_utc().timestamp() as i64)
            })
        }),
    }))
}

/// `Time` in millis method
fn time_in_millis_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_time_arg(span, &values, |time| {
                Value::Int(time.and_utc().timestamp_millis() as i64)
            })
        }),
    }))
}

/// `Time` add weeks method
fn time_add_weeks_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(weeks) => fresh_time(rt, span, time + Duration::weeks(weeks)),
                    _ => utils::error(span, "weeks expected to be an int"),
                }
            })
        }),
    }))
}

/// `Time` add days method
fn time_add_days_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(days) => fresh_time(rt, span, time + Duration::days(days)),
                    _ => utils::error(span, "days expected to be an int"),
                }
            })
        }),
    }))
}

/// `Time` add hours method
fn time_add_hours_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(hours) => fresh_time(rt, span, time + Duration::hours(hours)),
                    _ => utils::error(span, "hours expected to be an int"),
                }
            })
        }),
    }))
}

/// `Time` add minutes method
fn time_add_minutes_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(minutes) => fresh_time(rt, span, time + Duration::minutes(minutes)),
                    _ => utils::error(span, "minutes expected to be an int"),
                }
            })
        }),
    }))
}

/// `Time` add seconds method
fn time_add_seconds_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(secs) => fresh_time(rt, span, time + Duration::seconds(secs)),
                    _ => utils::error(span, "seconds expected to be an int"),
                }
            })
        }),
    }))
}

/// `Time` add millis method
fn time_add_millis_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(millis) => {
                        fresh_time(rt, span, time + Duration::milliseconds(millis))
                    }
                    _ => utils::error(span, "millis expected to be an int"),
                }
            })
        }),
    }))
}

/// `Time` add nanos method
fn time_add_nanos_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(nanos) => fresh_time(rt, span, time + Duration::nanoseconds(nanos)),
                    _ => utils::error(span, "nanos expected to be an int"),
                }
            })
        }),
    }))
}

/// `Time` add micros method
fn time_add_micros_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(nanos) => fresh_time(rt, span, time + Duration::microseconds(nanos)),
                    _ => utils::error(span, "nanos expected to be an int"),
                }
            })
        }),
    }))
}

/// `Time` sub weeks method
fn time_sub_weeks_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(weeks) => fresh_time(rt, span, time - Duration::weeks(weeks)),
                    _ => utils::error(span, "weeks expected to be an int"),
                }
            })
        }),
    }))
}

/// `Time` sub days method
fn time_sub_days_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(days) => fresh_time(rt, span, time - Duration::days(days)),
                    _ => utils::error(span, "days expected to be an int"),
                }
            })
        }),
    }))
}

/// `Time` sub hours method
fn time_sub_hours_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(hours) => fresh_time(rt, span, time - Duration::hours(hours)),
                    _ => utils::error(span, "hours expected to be an int"),
                }
            })
        }),
    }))
}

/// `Time` sub minutes method
fn time_sub_minutes_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(minutes) => fresh_time(rt, span, time - Duration::minutes(minutes)),
                    _ => utils::error(span, "minutes expected to be an int"),
                }
            })
        }),
    }))
}

/// `Time` sub seconds method
fn time_sub_seconds_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(secs) => fresh_time(rt, span, time - Duration::seconds(secs)),
                    _ => utils::error(span, "seconds expected to be an int"),
                }
            })
        }),
    }))
}

/// `Time` sub millis method
fn time_sub_millis_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(millis) => {
                        fresh_time(rt, span, time - Duration::milliseconds(millis))
                    }
                    _ => utils::error(span, "millis expected to be an int"),
                }
            })
        }),
    }))
}

/// `Time` sub nanos method
fn time_sub_nanos_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(nanos) => fresh_time(rt, span, time - Duration::nanoseconds(nanos)),
                    _ => utils::error(span, "nanos expected to be an int"),
                }
            })
        }),
    }))
}

/// `Time` sub micros method
fn time_sub_micros_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(nanos) => fresh_time(rt, span, time - Duration::microseconds(nanos)),
                    _ => utils::error(span, "nanos expected to be an int"),
                }
            })
        }),
    }))
}

/// `Time` with year method
fn time_with_year_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(year) => fresh_time(
                        rt,
                        span,
                        match time.with_year(year as i32) {
                            Some(dt) => dt,
                            _ => utils::error(span, "invalid year"),
                        },
                    ),
                    _ => utils::error(span, "year expected to be an int"),
                }
            })
        }),
    }))
}

/// `Time` with month method
fn time_with_month_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(month) if month >= 0 => fresh_time(
                        rt,
                        span,
                        match time.with_month0(month as u32) {
                            Some(dt) => dt,
                            _ => utils::error(span, "invalid month"),
                        },
                    ),
                    _ => utils::error(span, "month expected to be a positive int"),
                }
            })
        }),
    }))
}

/// `Time` with ordinal method
fn time_with_ordinal_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(ordinal) if ordinal >= 0 => fresh_time(
                        rt,
                        span,
                        match time.with_ordinal0(ordinal as u32) {
                            Some(dt) => dt,
                            _ => utils::error(span, "invalid ordinal"),
                        },
                    ),
                    _ => utils::error(span, "ordinal expected to be a positive int"),
                }
            })
        }),
    }))
}

/// `Time` with day method
fn time_with_day_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(day) if day >= 0 => fresh_time(
                        rt,
                        span,
                        match time.with_day(day as u32) {
                            Some(dt) => dt,
                            _ => utils::error(span, "invalid day"),
                        },
                    ),
                    _ => utils::error(span, "day expected to be a positive int"),
                }
            })
        }),
    }))
}

/// `Time` with hour method
fn time_with_hour_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(hour) if hour >= 0 => fresh_time(
                        rt,
                        span,
                        match time.with_hour(hour as u32) {
                            Some(dt) => dt,
                            _ => utils::error(span, "invalid hour"),
                        },
                    ),
                    _ => utils::error(span, "hour expected to be a positive int"),
                }
            })
        }),
    }))
}

/// `Time` with minute method
fn time_with_minute_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(minute) if minute >= 0 => fresh_time(
                        rt,
                        span,
                        match time.with_minute(minute as u32) {
                            Some(dt) => dt,
                            _ => utils::error(span, "invalid minute"),
                        },
                    ),
                    _ => utils::error(span, "minute expected to be a positive int"),
                }
            })
        }),
    }))
}

/// `Time` with second method
fn time_with_second_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::Int(second) if second >= 0 => fresh_time(
                        rt,
                        span,
                        match time.with_second(second as u32) {
                            Some(dt) => dt,
                            _ => utils::error(span, "invalid second"),
                        },
                    ),
                    _ => utils::error(span, "second expected to be a positive int"),
                }
            })
        }),
    }))
}

/// `Time` format method
fn time_format_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            validate_one_time_arg(span, &values, |time| {
                match values.get(1).cloned().unwrap() {
                    Value::String(fmt) => Value::String(time.format(&fmt).to_string()),
                    _ => utils::error(span, "format expected to be a string"),
                }
            })
        }),
    }))
}

/// `Time` gt method
fn time_gt_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            validate_two_time_args(span, &values, |a, b| Value::Bool(a > b))
        }),
    }))
}

/// `Time` ge method
fn time_ge_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            validate_two_time_args(span, &values, |a, b| Value::Bool(a >= b))
        }),
    }))
}

/// `Time` lt method
fn time_lt_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            validate_two_time_args(span, &values, |a, b| Value::Bool(a < b))
        }),
    }))
}

/// `Time` le method
fn time_le_method() -> Method {
    Method::Native(Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            validate_two_time_args(span, &values, |a, b| Value::Bool(a <= b))
        }),
    }))
}

/// Provides `Time` class
fn provide_time_class() -> Ref<Class> {
    Ref::new(Class {
        name: "Time".to_string(),
        methods: HashMap::from([
            // Init method
            ("init".to_string(), time_init_method()),
            // Year method
            ("year".to_string(), time_year_method()),
            // Month method
            ("month".to_string(), time_month_method()),
            // Week method
            ("week".to_string(), time_week_method()),
            // Ordinal method
            ("ordinal".to_string(), time_ordinal_method()),
            // Day method
            ("day".to_string(), time_day_method()),
            // Weekday method
            ("weekday".to_string(), time_weekday_method()),
            // Hour method
            ("hour".to_string(), time_hour_method()),
            // Minute method
            ("minute".to_string(), time_minute_method()),
            // Second method
            ("second".to_string(), time_second_method()),
            // In seconds method
            ("in_seconds".to_string(), time_in_seconds_method()),
            // In millis method
            ("in_millis".to_string(), time_in_millis_method()),
            // Add weeks method
            ("add_weeks".to_string(), time_add_weeks_method()),
            // Add days method
            ("add_days".to_string(), time_add_days_method()),
            // Add hours method
            ("add_hours".to_string(), time_add_hours_method()),
            // Add minutes method
            ("add_minutes".to_string(), time_add_minutes_method()),
            // Add seconds method
            ("add_seconds".to_string(), time_add_seconds_method()),
            // Add millis method
            ("add_millis".to_string(), time_add_millis_method()),
            // Add nanos method
            ("add_nanos".to_string(), time_add_nanos_method()),
            // Add micros method
            ("add_micros".to_string(), time_add_micros_method()),
            // Sub weeks method
            ("sub_weeks".to_string(), time_sub_weeks_method()),
            // Sub days method
            ("sub_days".to_string(), time_sub_days_method()),
            // Sub hours method
            ("sub_hours".to_string(), time_sub_hours_method()),
            // Sub minutes method
            ("sub_minutes".to_string(), time_sub_minutes_method()),
            // Sub seconds method
            ("sub_seconds".to_string(), time_sub_seconds_method()),
            // Sub millis method
            ("sub_millis".to_string(), time_sub_millis_method()),
            // Sub nanos method
            ("sub_nanos".to_string(), time_sub_nanos_method()),
            // Sub micros method
            ("sub_micros".to_string(), time_sub_micros_method()),
            // With year method
            ("with_year".to_string(), time_with_year_method()),
            // With month method
            ("with_month".to_string(), time_with_month_method()),
            // With ordinal method
            ("with_ordinal".to_string(), time_with_ordinal_method()),
            // With day method
            ("with_day".to_string(), time_with_day_method()),
            // With hour method
            ("with_hour".to_string(), time_with_hour_method()),
            // With minute method
            ("with_minute".to_string(), time_with_minute_method()),
            // With second method
            ("with_second".to_string(), time_with_second_method()),
            // Greater than
            ("gt".to_string(), time_gt_method()),
            // Greater or equal
            ("ge".to_string(), time_ge_method()),
            // Less than
            ("lt".to_string(), time_lt_method()),
            // Less or equal
            ("le".to_string(), time_le_method()),
            // Format
            ("format".to_string(), time_format_method()),
        ]),
    })
}

/// Time local
fn local() -> Ref<Native> {
    Ref::new(Native {
        arity: 0,
        function: Box::new(|rt, span, _| fresh_time(rt, span, Local::now().naive_local())),
    })
}

/// Time utc
fn utc() -> Ref<Native> {
    Ref::new(Native {
        arity: 0,
        function: Box::new(|rt, span, _| fresh_time(rt, span, Utc::now().naive_utc())),
    })
}

/// Time from seconds
fn from_seconds() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|rt, span, values| match values.get(0).cloned().unwrap() {
            Value::Int(seconds) => fresh_time(
                rt,
                span,
                match DateTime::from_timestamp_secs(seconds) {
                    Some(dt) => dt.naive_local(),
                    None => utils::error(span, "invalid timestamp"),
                },
            ),
            _ => utils::error(span, "seconds expected to be an int"),
        }),
    })
}

/// Time from millis
fn from_millis() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|rt, span, values| match values.get(0).cloned().unwrap() {
            Value::Int(millis) => fresh_time(
                rt,
                span,
                match DateTime::from_timestamp_millis(millis) {
                    Some(dt) => dt.naive_local(),
                    None => utils::error(span, "invalid timestamp"),
                },
            ),
            _ => utils::error(span, "millis expected to be an int"),
        }),
    })
}

/// Time from nanos
fn from_nanos() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|rt, span, values| match values.get(0).cloned().unwrap() {
            Value::Int(nanos) => fresh_time(
                rt,
                span,
                DateTime::from_timestamp_nanos(nanos).naive_local(),
            ),
            _ => utils::error(span, "nanos expected to be an int"),
        }),
    })
}

/// Provides `is` module env
pub fn provide_env() -> EnvRef {
    let mut env = Environment::default();

    env.force_define("Time", Value::Class(provide_time_class()));
    env.force_define("local", Value::Callable(Callable::Native(local())));
    env.force_define("utc", Value::Callable(Callable::Native(utc())));
    env.force_define(
        "from_seconds",
        Value::Callable(Callable::Native(from_seconds())),
    );
    env.force_define(
        "from_millis",
        Value::Callable(Callable::Native(from_millis())),
    );
    env.force_define(
        "from_nanos",
        Value::Callable(Callable::Native(from_nanos())),
    );

    Rc::new(RefCell::new(env))
}
