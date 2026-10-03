use std::{iter::Sum, ops::{Add, Index, IndexMut, Mul}};

const GAMMA_VALS: [i16; 128] = [17, -17, 2761, -2761, 583, -583, 2649, -2649, 1637, -1637, 723, -723, 2288, -2288, 1100, -1100, 1409, -1409, 2662, -2662, 3281, -3281, 233, -233, 756, -756, 2156, -2156, 3015, -3015, 3050, -3050, 1703, -1703, 1651, -1651, 2789, -2789, 1789, -1789, 1847, -1847, 952, -952, 1461, -1461, 2687, -2687, 939, -939, 2308, -2308, 2437, -2437, 2388, -2388, 733, -733, 2337, -2337, 268, -268, 641, -641, 1584, -1584, 2298, -2298, 2037, -2037, 3220, -3220, 375, -375, 2549, -2549, 2090, -2090, 1645, -1645, 1063, -1063, 319, -319, 2773, -2773, 757, -757, 2099, -2099, 561, -561, 2466, -2466, 2594, -2594, 2804, -2804, 1092, -1092, 403, -403, 1026, -1026, 1143, -1143, 2150, -2150, 2775, -2775, 886, -886, 1722, -1722, 1212, -1212, 1874, -1874, 1029, -1029, 2110, -2110, 2935, -2935, 885, -885, 2154, -2154];

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
pub struct Z<const M: i64>(i16);

impl<const M: i64> Z<M> {
    pub fn new(v: i64) -> Self{
        return Self(v.rem_euclid(M) as i16);
    }
    pub fn base_case_multiply(a0: Self, a1: Self, b0: Self, b1: Self, gamma: Self) -> (Self, Self) {
        let c0 = (a0 * b0) + (a1 * b1 * gamma);
        let c1 = (a0 * b1) + (a1 * b0);
        return (c0, c1);
    }
}

impl<const M: i64> Mul for Z<M> {
    type Output = Z<M>;
    fn mul(self, rhs: Self) -> Self::Output {
        return Z::<M>(((self.0 as i64) * (rhs.0 as i64)).rem_euclid(M) as i16);
    }
}

impl<const M: i64> Add for Z<M> {
    type Output = Z<M>;
    fn add(self, rhs: Self) -> Self::Output {
        return Z::<M>(((self.0 as i64) + (rhs.0 as i64)).rem_euclid(M) as i16);
    }
}

#[derive(Clone, Copy)]
pub struct Zn<const M: i64, const N: usize>([Z<M>; N]);

impl<const M: i64, const N: usize> Zn<M, N> {
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

impl<const M: i64, const N: usize> Mul for Zn<M, N> {
    type Output = Zn<M, N>;
    fn mul(self, rhs: Self) -> Self::Output {

        let mut h = Zn::<M, N>::new_empty();
        for i in 0..(N / 2) {
            let gamma = Z::new(GAMMA_VALS[i] as i64);
            (h[2*i], h[2*i + 1]) = Z::base_case_multiply(self.0[2*i], self.0[2*i + 1], rhs.0[2*i], rhs.0[2*i + 1], gamma);
        }
        return h;
    }
}

impl<const M: i64, const N: usize> Index<usize> for Zn<M, N> {
    type Output = Z<M>;
    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl<const M: i64, const N: usize> IndexMut<usize> for Zn<M, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}

impl<const M: i64, const N: usize> Add for Zn<M, N> {
    type Output = Zn<M, N>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut out = Zn::<M, N>::new_empty();
        for i in 0..N {
            out[i] = self[i] + rhs[i];
        }
        return out;
    }
}

impl<const M: i64, const N: usize> Sum for Zn<M, N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::new_empty(), |lhs, rhs| lhs + rhs)
    }
}


#[derive(Clone, Copy)]
pub struct Znk<const M: i64, const N: usize, const K: usize>([Zn<M, N>; K]);

impl<const M: i64, const N: usize, const K: usize> Znk<M, N, K> {
    pub fn new_empty() -> Self {
        return Self([Zn::<M, N>::new_empty(); K]);
    }
    pub fn to_vec(&self) -> Vec<Zn<M, N>> {
        return self.0.to_vec();
    }
    pub fn iter(&self) -> std::vec::IntoIter<Zn<M, N>>{
        return self.to_vec().into_iter();
    }
    pub fn map<F: Fn(Zn<M, N>) -> Zn<M, N>>(&self, f: F) -> Self {
        let mut out = Self::new_empty();
        for i in 0..K {
            out[i] = f(self[i]);
        }
        return out;
    }
}

impl<const M: i64, const N: usize, const K: usize> Index<usize> for Znk<M, N, K> {
    type Output = Zn<M, N>;
    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl<const M: i64, const N: usize, const K: usize> IndexMut<usize> for Znk<M, N, K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}

impl<const M: i64, const N: usize, const K: usize> Add for Znk<M, N, K> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut out = Self::new_empty();
        for i in 0..K {
            out[i] = self[i] + rhs[i];
        }
        return out;
    }
}

pub struct Znkxk<const M: i64, const N: usize, const K: usize>([Znk<M, N, K>; K]);

impl<const M: i64, const N: usize, const K: usize> Znkxk<M, N, K> {
    pub fn new_empty() -> Self {
        return Self([Znk::<M, N, K>::new_empty(); K]);
    }
}

impl<const M: i64, const N: usize, const K: usize> Index<usize> for Znkxk<M, N, K> {
    type Output = Znk<M, N, K>;
    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl<const M: i64, const N: usize, const K: usize> IndexMut<usize> for Znkxk<M, N, K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}

pub trait Circle<T> {
    fn circle(&self, rhs: T) -> T;
}

impl<const M: i64, const N: usize, const K: usize> Circle<Znk<M, N, K>> for Znkxk<M, N, K> {
    fn circle(&self, rhs: Znk<M, N, K>) -> Znk<M, N, K> {
        let mut w = Znk::<M, N, K>::new_empty();
        for i in 0..K {
            w[i] = (0..K).map(|j| self[i][j] * rhs[j]).sum();
        }
        return w;
    }
}
