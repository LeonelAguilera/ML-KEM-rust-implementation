use std::{iter::Sum, ops::{Add, BitXor, Index, IndexMut, Mul, Sub}};

const GAMMA_VALS: [i16; 128] = [17, -17, 2761, -2761, 583, -583, 2649, -2649, 1637, -1637, 723, -723, 2288, -2288, 1100, -1100, 1409, -1409, 2662, -2662, 3281, -3281, 233, -233, 756, -756, 2156, -2156, 3015, -3015, 3050, -3050, 1703, -1703, 1651, -1651, 2789, -2789, 1789, -1789, 1847, -1847, 952, -952, 1461, -1461, 2687, -2687, 939, -939, 2308, -2308, 2437, -2437, 2388, -2388, 733, -733, 2337, -2337, 268, -268, 641, -641, 1584, -1584, 2298, -2298, 2037, -2037, 3220, -3220, 375, -375, 2549, -2549, 2090, -2090, 1645, -1645, 1063, -1063, 319, -319, 2773, -2773, 757, -757, 2099, -2099, 561, -561, 2466, -2466, 2594, -2594, 2804, -2804, 1092, -1092, 403, -403, 1026, -1026, 1143, -1143, 2150, -2150, 2775, -2775, 886, -886, 1722, -1722, 1212, -1212, 1874, -1874, 1029, -1029, 2110, -2110, 2935, -2935, 885, -885, 2154, -2154];

#[derive(Clone, Copy)]
pub struct B<const N: usize>([u8; N]);

impl<const N: usize> B<N> {
    pub fn new_populated(val: u8) -> Self {
        return B([val; N]);
    }
    pub fn new_random() -> Self {
        let mut b = [0; N];
        rand::fill(&mut b);
        return B(b);
    }
    pub fn to_vec(&self) -> Vec<u8> {
        return self.0.to_vec();
    }
    pub fn append<const N2: usize>(&self, &rhs: &B<N2>) -> B<{N + N2}> {
        let mut whole = B::<{N + N2}>::new_empty();
        let (one, two) = whole.0.split_at_mut(self.0.len());
        one.copy_from_slice(&self.0);
        two.copy_from_slice(&rhs.0);

        return whole;
    }
    pub fn len(&self) -> usize {
        return self.0.len();
    }
    pub fn pad<const M: usize>(self, padding: u8) -> B<M> {
        assert!(M <= N);
        let mut out = B::<M>::new_empty();
        for i in 0..M {
            out[i] = if i < N {self[i]} else {padding};
        }
        return out;
    }
}

impl<const N: usize> NewEmpty for B<N> {
    fn new_empty() -> Self {
        return Self([0; N]);
    }
}

impl<const N: usize> Index<usize> for B<N> {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl<const N: usize> IndexMut<usize> for B<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}

impl<const N: usize, const M: i64> From<Zn<M, N>> for B<N> {
    fn from(value: Zn<M, N>) -> Self {
        let mut b = Self::new_empty();
        for i in 0..N {
            b[i] = value[i].0 as u8;
        }
        return b;
    }
}

impl<const N: usize> BitXor for B<N> {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut out = Self::new_empty();
        for i in 0..out.len() {
            out[i] = self[i] ^ rhs[i];
        }
        return out;
    }
}

pub struct BK<const N: usize, const K: usize>([B<N>; K]);

impl<const N: usize, const K: usize> BK<N, K> {
    pub fn flatten(&self) -> B<{K * N}> {
        let mut b = B::new_empty();
        for i in 0..K {
            for j in 0..N {
                b[(i*N) + K] = self[i][j];
            }
        }
        return b;
    }
}

impl<const N: usize, const K: usize> NewEmpty for BK<N, K> {
    fn new_empty() -> Self {
        return BK([B::new_empty(); K])
    }
}

impl<const N: usize, const K: usize> Index<usize> for BK<N, K> {
    type Output = B<N>;
    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl<const N: usize, const K: usize> IndexMut<usize> for BK<N, K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.0[index];
    }
}

impl<const NK: usize, const N: usize> From<B<NK>> for BK<N, {NK/N}> {
    fn from(value: B<NK>) -> Self {
        assert_eq!(NK % N, 0, "Tried to turn a B<NK> into a BK<N, K> where N*K is not equal to NK");
        let mut out = BK::new_empty();
        let k = NK/N;

        for i in 0..k {
            for j in 0..N {
                out[i][j] = value[(i * N) + j];
            }
        }

        return out;
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
    pub fn rem_euclid(&self, rhs: i16) -> Self {
        return Self::new(self.0.rem_euclid(rhs) as i64);
    }
    pub fn unwrap(&self) -> i16 {
        return self.0;
    }

}

impl<const M: i64> Mul for Z<M> {
    type Output = Z<M>;
    fn mul(self, rhs: Self) -> Self::Output {
        return Self::new((self.0 as i64) * (rhs.0 as i64));
    }
}

impl<const M: i64> Add for Z<M> {
    type Output = Z<M>;
    fn add(self, rhs: Self) -> Self::Output {
        return Self::new((self.0 as i64) + (rhs.0 as i64));
    }
}

impl<const M: i64> Sub for Z<M> {
    type Output = Z<M>;
    fn sub(self, rhs: Self) -> Self::Output {
        return Self::new((self.0 as i64) - (rhs.0 as i64));
    }
}

#[derive(Clone, Copy)]
pub struct Zn<const M: i64, const N: usize>([Z<M>; N]);

impl<const M: i64, const N: usize> Zn<M, N> {
    pub fn to_vec(&self) -> Vec<Z<M>> {
        return self.0.to_vec();
    }
    pub fn iter(&self) -> std::vec::IntoIter<Z<M>> {
        return self.to_vec().into_iter();
    }
}

impl<const M: i64, const N: usize> NewEmpty for Zn<M, N> {
    fn new_empty() -> Self {
        return Self([Z::<M>::new(0); N]);
    }
}

impl<const M: i64, const N: usize> Mul for Zn<M, N> {
    type Output = Zn<M, N>;
    fn mul(self, rhs: Self) -> Self::Output {

        let mut h = Zn::<M, N>::new_empty();
        for i in 0..(N / 2) {
            assert!(i < GAMMA_VALS.len(), "Tried to multiply two Zn values with N greater than 128");
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
    pub fn to_vec(&self) -> Vec<Zn<M, N>> {
        return self.0.to_vec();
    }
    pub fn iter(&self) -> std::vec::IntoIter<Zn<M, N>>{
        return self.to_vec().into_iter();
    }
    pub fn map<S, F>(&self, f: F) -> S
    where 
        S: NewEmpty + IndexMut<usize>,
        <S as Index<usize>>::Output: Sized,
        F: Fn(Zn<M, N>) -> <S as Index<usize>>::Output,
    {
        let mut out = S::new_empty();
        for i in 0..K {
            out[i] = f(self[i]);
        }
        return out;
    }
}

impl<const M: i64, const N: usize, const K: usize> NewEmpty for Znk<M, N, K> {
    fn new_empty() -> Self {
        return Self([Zn::<M, N>::new_empty(); K]);
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

impl<const M: i64, const N: usize, const K: usize> NewEmpty for Znkxk<M, N, K> {
    fn new_empty() -> Self {
        return Self([Znk::<M, N, K>::new_empty(); K]);
    }
}

pub trait NewEmpty {
    fn new_empty() -> Self;
}


#[cfg(test)]
mod tests {
    use crate::sets::{B, BK, NewEmpty, Z, Zn, Znk};

    #[test]
    fn test_b_index() {
        let mut test_element = B::<5>::new_random();
        test_element[1] = 2;
        test_element[2] = 3;
        test_element[3] = 4;
        test_element[4] = test_element[0];
        assert_eq!(test_element[0], test_element[4]);
        assert_eq!(test_element[1], 2);
        assert_eq!(test_element[2], 3);
        assert_eq!(test_element[3], 4);
    }

    #[test]
    fn test_bk_from_b() {
        let test_b = B::<12>::new_random();
        let test_bk: BK<4, 3> = BK::from(test_b.clone());
        assert_eq!(test_b[0],  test_bk[0][0]);
        assert_eq!(test_b[1],  test_bk[0][1]);
        assert_eq!(test_b[2],  test_bk[0][2]);
        assert_eq!(test_b[3],  test_bk[0][3]);
        assert_eq!(test_b[4],  test_bk[1][0]);
        assert_eq!(test_b[5],  test_bk[1][1]);
        assert_eq!(test_b[6],  test_bk[1][2]);
        assert_eq!(test_b[7],  test_bk[1][3]);
        assert_eq!(test_b[8],  test_bk[2][0]);
        assert_eq!(test_b[9],  test_bk[2][1]);
        assert_eq!(test_b[10], test_bk[2][2]);
        assert_eq!(test_b[11], test_bk[2][3]);
    }

    #[test]
    fn test_z_new() {
        let a = Z::<24>::new(0);
        let b = Z::<24>::new(12);
        let c = Z::<24>::new(23);
        let d = Z::<24>::new(24);
        let e = Z::<24>::new(25);
        let f = Z::<24>::new(48);
        let g = Z::<24>::new(49);

        assert_eq!(a.0, 0);
        assert_eq!(b.0, 12);
        assert_eq!(c.0, 23);
        assert_eq!(d.0, 0);
        assert_eq!(e.0, 1);
        assert_eq!(f.0, 0);
        assert_eq!(g.0, 1);
    }

    #[test]
    fn test_z_add() {
        let a = Z::<24>::new(13);
        let b = Z::<24>::new(15);
        let c = a + b;
        assert_eq!(c.0, 4);
    }

    #[test]
    fn test_z_mul() {
        let a = Z::<24>::new(13);
        let b = Z::<24>::new(15);
        let c = a * b;
        assert_eq!(c.0, 3);
    }

    #[test]
    fn test_zn_index() {
        let mut test_element = Zn::<24, 5>::new_empty();
        test_element[0] = Z::new(1);
        test_element[1] = Z::new(2);
        test_element[2] = Z::new(3);
        test_element[3] = Z::new(4);
        test_element[4] = test_element[0];
        assert_eq!(test_element[0].0, test_element[4].0);
        assert_eq!(test_element[1].0, 2);
        assert_eq!(test_element[2].0, 3);
        assert_eq!(test_element[3].0, 4);
    }

    #[test]
    fn test_zn_add() {
        let mut a = Zn::<24, 5>::new_empty();
        let mut b = Zn::<24, 5>::new_empty();

        for i in 0..5 {
            a[i] = Z::new(3 * i as i64);
            b[i] = Z::new(7 * i as i64);
        }

        let c = a + b;
        assert_eq!(c[0].0, 0);
        assert_eq!(c[1].0, 10);
        assert_eq!(c[2].0, 20);
        assert_eq!(c[3].0, 6);
        assert_eq!(c[4].0, 16);
    }

    #[test]
    fn test_znk_sum() {
        let mut a = Znk::<24, 5, 10>::new_empty();
        for i in 0..5 {
            for j in 0..10 {
                a[j][i] = Z::new(2 * (i + j + 1) as i64);
            }
        }
        let b: Zn<24, 5> = a.iter().sum();

        assert_eq!(b[0].0, 14);
        assert_eq!(b[1].0, 10);
        assert_eq!(b[2].0, 6);
        assert_eq!(b[3].0, 2);
        assert_eq!(b[4].0, 22);
    }
}
