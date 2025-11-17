#![allow(dead_code)]
use flux_rs::*;

#[opaque]
#[refined_by(len: int)]
pub struct RVec<T> {
    pub inner: Vec<T>,
}

#[macro_export]
macro_rules! rvec {
    () => { RVec::new() };
    ($($e:expr),+$(,)?) => {{
        let mut res = RVec::new();
        $( res.push($e); )*
        res
    }};
    ($elem:expr; $n:expr) => {{
        RVec::from_elem_n($elem, $n)
    }}
}

impl<T> RVec<T> {
    #[trusted]
    #[sig(fn() -> RVec<T>[0])]
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    #[trusted]
    #[sig(fn(self: &strg RVec<T>[@n], T) -> () ensures self: RVec<T>[n+1])]
    pub fn push(&mut self, item: T) {
        self.inner.push(item);
    }

    #[trusted]
    #[sig(fn(&RVec<T>[@n]) -> usize[n])]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[trusted]
    #[sig(fn(&RVec<T>[@n]) -> bool[n == 0])]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[trusted]
    #[sig(fn(&RVec<T>[@n], i: usize{0 <= i && i < n}) -> &T)]
    pub fn get(&self, i: usize) -> &T {
        &self.inner[i]
    }

    #[trusted]
    #[sig(fn(&mut RVec<T>[@n], i: usize{ 0 <= i && i < n}) -> &mut T)]
    pub fn get_mut(&mut self, i: usize) -> &mut T {
        &mut self.inner[i]
    }

    #[trusted]
    #[sig(fn(self: &strg RVec<T>[@n]) -> T
    		requires n > 0
            ensures self: RVec<T>[n-1])]
    pub fn pop(&mut self) -> T {
        self.inner.pop().unwrap()
    }

    #[trusted]
    #[sig(fn(&mut RVec<T>[@n], a: usize{0 <= a && a < n}, b: usize{0 <= b && b < n}) -> ())]
    pub fn swap(&mut self, a: usize, b: usize) {
        self.inner.swap(a, b);
    }

    #[trusted]
    #[sig(fn(T, n: usize) -> RVec<T>[n])]
    pub fn from_elem_n(elem: T, n: usize) -> Self
    where
        T: Copy,
    {
        let mut vec = Self::new();
        let mut i = 0;
        while i < n {
            vec.push(elem);
            i += 1;
        }
        vec
    }

    #[trusted]
    #[sig(fn(&RVec<T>[@n]) -> RVec<T>[n])]
    pub fn clone(&self) -> Self
    where
        T: Clone,
    {
        Self {
            inner: self.inner.clone(),
        }
    }

    #[trusted]
    pub fn from_vec(inner: Vec<T>) -> RVec<T> {
        RVec { inner }
    }

    #[trusted]
    pub fn to_vec(self) -> Vec<T> {
        self.inner
    }

    #[trusted]
    pub fn as_ptr(self) -> *const T {
        self.inner.as_ptr()
    }

    #[trusted]
    #[sig(fn(self: &strg RVec<T>[@n], other: &[T][@m]) -> () ensures self: RVec<T>[n + m])]
    pub fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone,
    {
        self.inner.extend_from_slice(other)
    }

    #[trusted]
    #[sig(fn(self: &strg RVec<T>[@cur], len:usize{len <= cur}) ensures self: RVec<T>[len])]
    pub fn set_len(&mut self, len: usize) {
        unsafe { self.inner.set_len(len) }
    }
}

#[opaque]
pub struct RVecIter<T> {
    vec: RVec<T>,
    curr: usize,
}

impl<T> IntoIterator for RVec<T> {
    type Item = T;
    type IntoIter = RVecIter<T>;

    // TODO: cannot get variant of opaque struct
    #[trusted]
    #[sig(fn(RVec<T>) -> RVecIter<T>)]
    fn into_iter(self) -> RVecIter<T> {
        RVecIter { vec: self, curr: 0 }
    }
}

impl<T> Iterator for RVecIter<T> {
    type Item = T;

    // TODO: cannot get variant of opaque struct
    #[trusted]
    #[sig(fn(&mut RVecIter<T>) -> Option<T>)]
    fn next(&mut self) -> Option<T> {
        self.vec.inner.pop()
    }
}

#[trusted_impl]
impl<T> std::ops::Index<usize> for RVec<T> {
    type Output = T;

    #[sig(fn(&RVec<T>[@n], usize{v : 0 <= v && v < n}) -> &T)]
    fn index(&self, index: usize) -> &T {
        self.get(index)
    }
}

#[trusted_impl]
impl<T> std::ops::IndexMut<usize> for RVec<T> {
    #[sig(fn(&mut RVec<T>[@n], usize{v : 0 <= v && v < n}) -> &mut T)]
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index)
    }
}

#[opaque]
#[refined_by(base:int, addr:int, len: int)]
#[invariant(fits_in_lin_mem_base(base, addr, len))]
pub struct BSlice<'a> {
    pub inner: &'a mut [u8],
}

impl BSlice<'_> {
    #[trusted]
    pub fn from_raw<'a>(inner: &'a mut [u8]) -> BSlice<'a> {
        BSlice { inner }
    }
}
