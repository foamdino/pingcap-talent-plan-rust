#![deny(missing_docs)]
// Copyright ⓒ 2019 Kev Jackson.
// Licensed under the MIT license
// (see LICENSE or <http://opensource.org/licenses/MIT>) All files in the project carrying such
// notice may not be copied, modified, or distributed except according to those terms.

//! `kvs` is a library for a distributed key-val store (in the same sphere as consul/cockroach/etcd)

use std::collections::BTreeMap;

/// Defines the KvStore struct - right now it's minimal
#[derive(Default)]
pub struct KvStore {
    store: BTreeMap<String, String>,
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

    /// Sets a value for a given key
    ///
    /// # Example
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// ```
    pub fn set(&mut self, k: String, v: String) {
        self.store.insert(k, v);
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
    pub fn get(&self, k: String) -> Option<String> {
        match self.store.get(&k) {
            Some(val) => Some(val.clone()),
            None => None,
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
    pub fn remove(&mut self, k: String) {
        self.store.remove(&k);
    }
}
