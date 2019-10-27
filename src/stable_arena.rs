use core::iter::FusedIterator;
use core::ops;

use crate::arena::{Arena, Index};
use core::marker::PhantomData;

// WIP! Docs
// WIP! Feature parity
// WIP! Tests
// WIP! SmallVec

/// WIP!
#[derive(Clone, Debug)]
pub struct StableArena<T> {
    pub(self) arena: Arena<LinkedEntry<T>>,
    first: Option<Index>,
    last: Option<Index>,
}

#[derive(Clone, Debug)]
pub struct LinkedEntry<T> {
    value: T,
    prev: Option<Index>,
    next: Option<Index>,
}

impl<T> StableArena<T> {
    /// WIP!
    pub fn new() -> StableArena<T> {
        StableArena {
            arena: Arena::new(),
            first: None,
            last: None,
        }
    }

    /// WIP!
    pub fn with_capacity(n: usize) -> StableArena<T> {
        StableArena {
            arena: Arena::with_capacity(n),
            first: None,
            last: None,
        }
    }

    /// WIP!
    pub fn clear(&mut self) {
        self.arena.clear();
        self.first = None;
        self.last = None;
    }

    /// WIP!
    #[inline]
    pub fn insert(&mut self, value: T) -> Index {
        let old_last = self.last;
        let entry = LinkedEntry {
            value,
            prev: old_last,
            next: None,
        };
        let new_last = self.arena.insert(entry);

        if let Some(l) = old_last {
            match self.arena.get_mut(l) {
                Some(LinkedEntry { ref mut next, .. }) => *next = Some(new_last),
                None => unreachable!(),
            }
        }

        self.last = Some(new_last);

        if self.first.is_none() {
            self.first = Some(new_last);
        }

        new_last
    }

    /// WIP!
    pub fn remove(&mut self, i: Index) -> Option<T> {
        match self.arena.remove(i) {
            Some(entry) => {
                if let Some(p) = entry.prev {
                    match self.arena.get_mut(p) {
                        Some(prev) => prev.next = entry.next,
                        None => unreachable!(),
                    }
                }
                if let Some(n) = entry.next {
                    match self.arena.get_mut(n) {
                        Some(next) => next.prev = entry.prev,
                        _ => unreachable!(),
                    }
                }
                if Some(i) == self.last {
                    self.last = entry.prev;
                }
                if Some(i) == self.first {
                    self.first = entry.next;
                }

                Some(entry.value)
            },
            None => None
        }
    }

    /// WIP!
    pub fn contains(&self, i: Index) -> bool {
        self.arena.contains(i)
    }

    /// WIP!
    pub fn get(&self, i: Index) -> Option<&T> {
        self.arena.get(i).map(|e| &e.value)
    }

    /// WIP!
    pub fn get_mut(&mut self, i: Index) -> Option<&mut T> {
        self.arena.get_mut(i).map(|e| &mut e.value)
    }

    /// WIP!
    pub fn len(&self) -> usize {
        self.arena.len()
    }

    /// WIP!
    pub fn is_empty(&self) -> bool {
        self.arena.is_empty()
    }

    /// WIP!
    pub fn capacity(&self) -> usize {
        self.arena.capacity()
    }

    /// WIP!
    pub fn reserve(&mut self, additional_capacity: usize) {
        self.arena.reserve(additional_capacity);
    }

    /// WIP!
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            arena: &self.arena,
            next: self.first,
            len: self.arena.len(),
        }
    }

    /// WIP!
    pub fn iter_foo(&mut self) -> IterFoo<T> {
        IterFoo {
            next: self.first,
            _phantom_t: PhantomData
        }
    }
}

#[derive(Clone, Debug)]
pub struct Iter<'a, T> {
    arena: &'a Arena<LinkedEntry<T>>,
    next: Option<Index>,
    len: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (Index, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Some(index) => {
                let current = &self.arena[index];

                self.next = current.next;
                self.len -= 1;

                Some((index, &current.value))
            },
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<'a, T> FusedIterator for Iter<'a, T> {}

// WIP! better name
#[derive(Debug)]
pub struct IterFoo<T> {
    next: Option<Index>,
    _phantom_t: PhantomData<T>,
}

impl<T> IterFoo<T> {
    pub fn next(&mut self, arena: &StableArena<T>) -> Option<Index> {
        match self.next {
            Some(index) => {
                self.next = arena.arena[index].next;
                Some(index)
            },
            None => None,
        }
    }
}

impl<T> ops::Index<Index> for StableArena<T> {
    type Output = T;

    fn index(&self, index: Index) -> &Self::Output {
        self.get(index).expect("No element at index")
    }
}

impl<T> ops::IndexMut<Index> for StableArena<T> {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        self.get_mut(index).expect("No element at index")
    }
}
