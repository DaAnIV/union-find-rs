// Copyright 2016 union-find-rs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use dashmap::DashMap;

use crate::{Union, UnionFind, UnionResult};
use std::iter::FromIterator;

/// Union-Find implementation with quick union operation.
#[derive(Debug)]
pub struct QuickUnionUf<V> {
    link_parent: DashMap<usize, usize>,
    payload: Vec<Option<V>>,
}

impl<V: Clone> Clone for QuickUnionUf<V> {
    #[inline]
    fn clone(&self) -> QuickUnionUf<V> {
        QuickUnionUf {
            link_parent: self.link_parent.clone(),
            payload: self.payload.clone(),
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &QuickUnionUf<V>) {
        self.link_parent.clone_from(&other.link_parent);
        self.payload.clone_from(&other.payload);
    }
}

impl<V: Union> UnionFind<V> for QuickUnionUf<V> {
    #[inline]
    fn size(&self) -> usize {
        self.payload.len()
    }

    #[inline]
    fn insert(&mut self, data: V) -> usize {
        let key = self.payload.len();
        let _ = self.link_parent.insert(key, key);
        self.payload.push(Some(data));
        key
    }

    #[inline]
    fn union(&mut self, key0: usize, key1: usize) -> bool {
        let k0 = self.find(key0);
        let k1 = self.find(key1);
        if k0 == k1 {
            return false;
        }

        // Temporary replace with dummy to move out the elements of the vector.
        let v0 = self.payload[k0].take().unwrap();
        let v1 = self.payload[k1].take().unwrap();

        let (parent, child, val) = match Union::union(v0, v1) {
            UnionResult::Left(val) => (k0, k1, val),
            UnionResult::Right(val) => (k1, k0, val),
        };
        self.payload[parent] = Some(val);
        let _ = self.link_parent.insert(child, parent);

        true
    }

    #[inline]
    fn find(&self, key: usize) -> usize {
        let mut k = key;
        let mut p = *self.link_parent.get(&k).unwrap();
        while p != k {
            let pp = *self.link_parent.get(&p).unwrap();
            let _ = self.link_parent.insert(k, pp);
            k = p;
            p = pp;
        }
        k
    }

    #[inline]
    fn get(&self, key: usize) -> &V {
        let root_key = self.find(key);
        self.payload[root_key].as_ref().unwrap()
    }

    #[inline]
    fn get_mut(&mut self, key: usize) -> &mut V {
        let root_key = self.find(key);
        self.payload[root_key].as_mut().unwrap()
    }
}

impl<A: Union> FromIterator<A> for QuickUnionUf<A> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = A>>(iterator: T) -> QuickUnionUf<A> {
        let mut uf = QuickUnionUf {
            link_parent: Default::default(),
            payload: vec![],
        };
        uf.extend(iterator);
        uf
    }
}

impl<A> Extend<A> for QuickUnionUf<A> {
    #[inline]
    fn extend<T>(&mut self, iterable: T)
    where
        T: IntoIterator<Item = A>,
    {
        let len = self.payload.len();
        let payload = iterable.into_iter().map(Some);
        self.payload.extend(payload);

        let new_len = self.payload.len();
        self.link_parent.extend((len..new_len).map(|x| (x, x)));
    }
}
