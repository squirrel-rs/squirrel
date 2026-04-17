/// Imports
use crate::rt::realm::Realm;
use std::{cell::RefCell, rc::Rc};

/// Ref types
pub type MutRef<T> = Rc<RefCell<T>>;
pub type Ref<T> = Rc<T>;
pub type RealmRef = Rc<RefCell<Realm>>;
