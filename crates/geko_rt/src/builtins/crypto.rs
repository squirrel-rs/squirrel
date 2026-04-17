/// Imports
use crate::{
    builtins::utils,
    refs::{RealmRef, Ref},
    rt::{
        realm::Realm,
        value::{Callable, Native, Value},
    },
};
use base64::{Engine, prelude::BASE64_STANDARD};
use md5::{Digest, Md5};
use sha1::{Sha1};
use sha2::{Sha224, Sha256, Sha384, Sha512};
use std::{cell::RefCell, rc::Rc};

/// Base64 encode
fn b64() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| {
            Value::String(BASE64_STANDARD.encode(values.first().cloned().unwrap().to_string()))
        }),
    })
}

/// Base64 decode
fn de_b64() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, span, values| {
            match BASE64_STANDARD.decode(values.first().cloned().unwrap().to_string()) {
                Ok(bytes) => Value::String(String::from_utf8_lossy(&bytes).to_string()),
                Err(err) => utils::error(span, &format!("failed to decode `base64` string: {err}")),
            }
        }),
    })
}

/// Sha1 encode
fn sha1() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| {
            Value::String(hex::encode(Sha1::digest(
                values.first().cloned().unwrap().to_string(),
            )))
        }),
    })
}

/// Sha256 encode
fn sha256() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| {
            Value::String(hex::encode(Sha256::digest(
                values.first().cloned().unwrap().to_string(),
            )))
        }),
    })
}

/// Sha224 encode
fn sha224() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| {
            Value::String(hex::encode(Sha224::digest(
                values.first().cloned().unwrap().to_string(),
            )))
        }),
    })
}

/// Sha512 encode
fn sha512() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| {
            Value::String(hex::encode(Sha512::digest(
                values.first().cloned().unwrap().to_string(),
            )))
        }),
    })
}

/// Sha384 encode
fn sha384() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| {
            Value::String(hex::encode(Sha384::digest(
                values.first().cloned().unwrap().to_string(),
            )))
        }),
    })
}

/// Md5 encode
fn md5() -> Ref<Native> {
    Ref::new(Native {
        arity: 1,
        function: Box::new(|_, _, values| {
            Value::String(hex::encode(Md5::digest(
                values.first().cloned().unwrap().to_string(),
            )))
        }),
    })
}

/// Provides `is` module realm
pub fn provide_env() -> RealmRef {
    let mut realm = Realm::default();

    realm.define("b64", Value::Callable(Callable::Native(b64())));
    realm.define("de_b64", Value::Callable(Callable::Native(de_b64())));
    realm.define("sha1", Value::Callable(Callable::Native(sha1())));
    realm.define("sha256", Value::Callable(Callable::Native(sha256())));
    realm.define("sha224", Value::Callable(Callable::Native(sha224())));
    realm.define("sha512", Value::Callable(Callable::Native(sha512())));
    realm.define("sha384", Value::Callable(Callable::Native(sha384())));
    realm.define("sha384", Value::Callable(Callable::Native(sha384())));
    realm.define("md5", Value::Callable(Callable::Native(md5())));

    Rc::new(RefCell::new(realm))
}
