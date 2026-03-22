use std::cell::RefCell;

/// Imports
use camino::Utf8PathBuf;
use squirrel_common::{
    bail,
    io::{IO, IOError},
};

/// Test IO implementation
pub struct TestIO {
    // Buffer used for output handling
    pub buffer: RefCell<String>,
}

/// Implementation of Test IO
impl IO for TestIO {
    /// Input implementation
    fn input(&self) -> String {
        bail!(IOError::NotSupported("input"));
    }

    /// Output implementation
    fn output(&self, text: &str) {
        self.buffer.borrow_mut().push_str(text);
    }

    /// Read implementation
    fn read(&self, _: &Utf8PathBuf) -> String {
        bail!(IOError::NotSupported("read"));
    }

    /// Resolve implementation
    fn resolve(&self, _: &str) -> Option<Utf8PathBuf> {
        None
    }

    /// Flushes stream
    fn flush(&self) {
        // unnecessary
    }
}
