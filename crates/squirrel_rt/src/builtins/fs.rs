/// Imports
use crate::{
    builtins::utils,
    refs::{EnvRef, MutRef, Ref},
    rt::{
        env::Environment,
        value::{Callable, Native, Value},
    },
};
use camino::Utf8PathBuf;
use squirrel_common::{bail, bug, io::IOError};
use squirrel_lex::token::Span;
use std::{
    cell::RefCell,
    fs::{self, File},
    rc::Rc,
};

/// Helper: validates path
fn validate_path<F, V>(span: &Span, path: Value, f: F) -> V
where
    F: FnOnce(Utf8PathBuf) -> V,
{
    match path {
        Value::String(path) => f(Utf8PathBuf::from(path)),
        other => utils::error(span, &format!("`{other}` is not a valid path")),
    }
}

/// Helper: validates path argument by index
fn validate_path_arg<F, V>(span: &Span, values: &[Value], index: usize, f: F) -> V
where
    F: FnOnce(Utf8PathBuf) -> V,
{
    validate_path(span, values.get(index).cloned().unwrap(), f)
}

/// Helper: validates one path argument
fn validate_one_path_arg<F, V>(span: &Span, values: &[Value], f: F) -> V
where
    F: FnOnce(Utf8PathBuf) -> V,
{
    validate_path_arg(span, values, 0, |path| f(path))
}

/// Helper: validates two path arguments
fn validate_two_path_args<F, V>(span: &Span, values: &[Value], f: F) -> V
where
    F: FnOnce(Utf8PathBuf, Utf8PathBuf) -> V,
{
    validate_path_arg(span, values, 0, |from| {
        validate_path_arg(span, values, 1, |to| f(from, to))
    })
}

/// Is path exists?
fn is_exists() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("exists"))
                } else {
                    Value::Bool(path.exists())
                }
            })
        }),
    })
}

/// Is path a directory?
fn is_dir() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("is_dir"))
                } else {
                    Value::Bool(path.is_dir())
                }
            })
        }),
    })
}

/// Is path a file?
fn is_file() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("is_file"))
                } else {
                    Value::Bool(path.is_file())
                }
            })
        }),
    })
}

/// Returns file name
fn file_name() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                path.file_name()
                    .map(|it| Value::String(it.to_string()))
                    .unwrap_or(Value::Null)
            })
        }),
    })
}

/// Returns file stem
fn file_stem() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                path.file_stem()
                    .map(|it| Value::String(it.to_string()))
                    .unwrap_or(Value::Null)
            })
        }),
    })
}

/// Make directory
fn mk_dir() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("mk_dir"))
                } else {
                    match fs::create_dir(path) {
                        Ok(_) => Value::Null,
                        Err(err) => {
                            utils::error(span, &format!("failed to make directory: `{err}`"))
                        }
                    }
                }
            })
        }),
    })
}

/// Make directory and it's parents
fn mk_dir_all() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("mk_dir_all"))
                } else {
                    match fs::create_dir_all(path) {
                        Ok(_) => Value::Null,
                        Err(err) => {
                            utils::error(span, &format!("failed to make directory: `{err}`"))
                        }
                    }
                }
            })
        }),
    })
}

/// Make file
fn mk_file() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("mk_file"))
                } else {
                    match File::create(path) {
                        Ok(_) => Value::Null,
                        Err(err) => utils::error(span, &format!("failed to create file: `{err}`")),
                    }
                }
            })
        }),
    })
}

/// Remove empty directory
fn rm_dir() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("rm_dir"))
                } else {
                    match fs::remove_dir(path) {
                        Ok(_) => Value::Null,
                        Err(err) => {
                            utils::error(span, &format!("failed to remove directory: `{err}`"))
                        }
                    }
                }
            })
        }),
    })
}

/// Remove directory and it's contents
fn rm_dir_all() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("rm_dir_all"))
                } else {
                    match fs::remove_dir_all(path) {
                        Ok(_) => Value::Null,
                        Err(err) => {
                            utils::error(span, &format!("failed to remove directory: `{err}`"))
                        }
                    }
                }
            })
        }),
    })
}

/// Remove file
fn rm_file() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("rm_file"))
                } else {
                    match fs::remove_file(path) {
                        Ok(_) => Value::Null,
                        Err(err) => utils::error(span, &format!("failed to remove file: `{err}`")),
                    }
                }
            })
        }),
    })
}

/// Files list
fn read_dir() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|rt, span, values| {
            validate_one_path_arg(span, &values, |path| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("read_dir"))
                } else {
                    let contents = match fs::read_dir(path) {
                        Ok(entries) => entries
                            .map(|entry| match entry {
                                Ok(path) => Value::String(format!("{:?}", path.path())),
                                Err(err) => {
                                    utils::error(span, &format!("failed to read entry: `{err}`"))
                                }
                            })
                            .collect::<Vec<Value>>(),
                        Err(err) => {
                            utils::error(span, &format!("failed to read directory: `{err}`"))
                        }
                    };

                    let list_builtin = rt
                        .builtins
                        .env
                        .borrow()
                        .lookup("List")
                        .unwrap_or_else(|| utils::error(span, "list builtin is not found"));

                    match list_builtin {
                        Value::Class(list_ty) => match rt.call_class(span, Vec::new(), list_ty) {
                            Ok(Value::Instance(list)) => {
                                list.borrow_mut().fields.insert(
                                    "$internal".to_string(),
                                    Value::Any(MutRef::new(RefCell::new(contents))),
                                );
                                Value::Instance(list)
                            }
                            _ => bug!("invalid list instantiation"),
                        },
                        _ => utils::error(span, "list builtin is not a class"),
                    }
                }
            })
        }),
    })
}

/// Copy file
fn copy() -> Ref<Native> {
    Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            validate_two_path_args(span, &values, |from, to| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("copy"))
                } else {
                    match fs::copy(from, to) {
                        Ok(_) => Value::Null,
                        Err(err) => utils::error(span, &format!("failed to copy file: `{err}`")),
                    }
                }
            })
        }),
    })
}

/// Rename file
fn rename() -> Ref<Native> {
    Ref::new(Native {
        arity: 2,
        function: Box::new(|_, span, values| {
            validate_two_path_args(span, &values, |from, to| {
                if cfg!(target_family = "wasm") {
                    bail!(IOError::NotSupported("rename"))
                } else {
                    match fs::rename(from, to) {
                        Ok(_) => Value::Null,
                        Err(err) => utils::error(span, &format!("failed to rename file: `{err}`")),
                    }
                }
            })
        }),
    })
}

/// Read file text
fn read() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|rt, span, values| {
            validate_one_path_arg(span, &values, |path| Value::String(rt.io.read(&path)))
        }),
    })
}

/// Write text to file
fn write() -> Ref<Native> {
    Ref::new(Native {
        arity: 2,
        function: Box::new(|rt, span, values| {
            validate_one_path_arg(span, &values, |path| {
                match values.get(1).cloned().unwrap() {
                    Value::String(text) => {
                        rt.io.write(&path, text);
                        Value::Null
                    }
                    other => utils::error(span, &format!("`{other}` is not valid content")),
                }
            })
        }),
    })
}

/// Provides `fs` module env
pub fn provide_env() -> EnvRef {
    let mut env = Environment::default();

    env.force_define("read", Value::Callable(Callable::Native(read())));
    env.force_define("write", Value::Callable(Callable::Native(write())));
    env.force_define("is_exists", Value::Callable(Callable::Native(is_exists())));
    env.force_define("is_dir", Value::Callable(Callable::Native(is_dir())));
    env.force_define("is_file", Value::Callable(Callable::Native(is_file())));
    env.force_define("file_name", Value::Callable(Callable::Native(file_name())));
    env.force_define("file_stem", Value::Callable(Callable::Native(file_stem())));
    env.force_define("mk_file", Value::Callable(Callable::Native(mk_file())));
    env.force_define("mk_dir", Value::Callable(Callable::Native(mk_dir())));
    env.force_define(
        "mk_dir_all",
        Value::Callable(Callable::Native(mk_dir_all())),
    );
    env.force_define("rm_file", Value::Callable(Callable::Native(rm_file())));
    env.force_define("rm_dir", Value::Callable(Callable::Native(rm_dir())));
    env.force_define(
        "rm_dir_all",
        Value::Callable(Callable::Native(rm_dir_all())),
    );
    env.force_define("read_dir", Value::Callable(Callable::Native(read_dir())));
    env.force_define("copy", Value::Callable(Callable::Native(copy())));
    env.force_define("rename", Value::Callable(Callable::Native(rename())));

    Rc::new(RefCell::new(env))
}
