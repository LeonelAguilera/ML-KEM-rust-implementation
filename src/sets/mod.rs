use std::ops::{Index, IndexMut};

#[derive(Clone, Copy)]
pub struct B<const N: usize>([u8; N]);

impl<const N: usize> B<N> {
    pub fn new_empty() -> Self {
        return B([0; N]);
    }
    pub fn new_random() -> Self {
        let mut b = [0; N];
        rand::fill(&mut b);
        return B(b);
    }
    pub fn to_vec(&self) -> Vec<u8> {
        return self.0.to_vec();
    }
}

#[derive(Copy, Clone)]
pub struct Z<const M: u64>(u16);

impl<const M: u64> Z<M> {
    pub fn new(v: u64) -> Self{
        return Self(v.rem_euclid(M) as u16);
    }
}

#[derive(Clone, Copy)]
pub struct Zn<const M: u64, const N: usize>([Z<M>; N]);

impl<const M: u64, const N: usize> Zn<M, N> {
    pub fn new_empty() -> Self {
        return Self([Z::<M>::new(0); N]);
    }
    pub fn to_vec(&self) -> Vec<Z<M>> {
        return self.0.to_vec();
    }
    pub fn iter(&self) -> std::vec::IntoIter<Z<M>> {
        return self.to_vec().into_iter();
    }
}

#[derive(Clone, Copy)]
pub struct Znk<const M: u64, const N: usize, const K: usize>([Zn<M, N>; K]);

impl<const M: u64, const N: usize, const K: usize> Znk<M, N, K> {
    pub fn new_empty() -> Self {
        return Self([Zn::<M, N>::new_empty(); K]);
    }
    pub fn to_vec(&self) -> Vec<Zn<M, N>> {
        return self.0.to_vec();
    }
    pub fn iter(&self) -> std::vec::IntoIter<Zn<M, N>>{
        return self.to_vec().into_iter();
    }
    // pub fn to_vec(&self) -> Vec<Vec<Z<M>>> {
    //     return self.0.to_vec();
    // }
}

impl<const M: u64, const N: usize, const K: usize> Index<usize> for Znk<M, N, K> {
    type Output = Zn<M, N>;
    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl<const M: u64, const N: usize, const K: usize> IndexMut<usize> for Znk<M, N, K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}

pub struct Znkxk<const M: u64, const N: usize, const K: usize>([Znk<M, N, K>; K]);

impl<const M: u64, const N: usize, const K: usize> Znkxk<M, N, K> {
    pub fn new_empty() -> Self {
        return Self([Znk::<M, N, K>::new_empty(); K]);
    }
    // pub fn to_vec(&self) -> Vec<Vec<Z<M>>> {
    //     return self.0.to_vec();
    // }
}

impl<const M: u64, const N: usize, const K: usize> Index<usize> for Znkxk<M, N, K> {
    type Output = Znk<M, N, K>;
    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl<const M: u64, const N: usize, const K: usize> IndexMut<usize> for Znkxk<M, N, K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}
