#![deny(missing_docs)]
// Copyright ⓒ 2019 Kev Jackson.
// Licensed under the MIT license
// (see LICENSE or <http://opensource.org/licenses/MIT>) All files in the project carrying such
// notice may not be copied, modified, or distributed except according to those terms.

//! `kvs` is a library for a distributed key-val store (in the same sphere as consul/cockroach/etcd)

use std::collections::BTreeMap;
use std::path::Path;
use std::error;
use std::result;
use std::fmt;

/// Defines the KvStore struct - right now it's minimal
#[derive(Default)]
pub struct KvStore {
    store: BTreeMap<String, String>,
}

/// type alias for Result
pub type Result<T> = result::Result<T, KvsError>;

/// Defines a custom error type
#[derive(Debug)]
pub struct KvsError;

impl fmt::Display for KvsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error while interacting with kvs")
    }
}

impl error::Error for KvsError {
    fn cause(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl KvStore {
    /// Returns a KvStore with an initially empty BTreeMap for storage
    ///
    /// # Example
    ///
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// ```
    pub fn new() -> KvStore {
        KvStore {
            store: BTreeMap::new(),
        }
    }

    /// Returns a previously persisted KvStore opened from a file
    pub fn open(_path: &Path) -> Result<KvStore> {
        Ok(KvStore {
            store: BTreeMap::new(),
        })
    }

    /// Sets a value for a given key
    ///
    /// # Example
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// ```
    pub fn set(&mut self, k: String, v: String) -> Result<()> {
        self.store.insert(k, v);
        Ok(())
    }

    /// Gets a value for a given key
    ///
    /// # Example
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// let v1 = store.get("key1".to_owned());
    /// assert_eq!("value1".to_owned(), v1.unwrap());
    /// ```
    pub fn get(&self, k: String) -> Result<Option<String>> {
        match self.store.get(&k) {
            Some(val) => Ok(Some(val.clone())),
            None => Err(KvsError),
        }
    }

    /// Removes a value for a given key
    ///
    /// # Example
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// let v1 = store.get("key1".to_owned());
    /// assert_eq!("value1".to_owned(), v1.unwrap());
    /// store.remove("key1".to_owned());
    /// ```
    pub fn remove(&mut self, k: String) -> Result<String> {
        self.store.remove(&k);
        Ok("removed".to_owned())
    }
}
