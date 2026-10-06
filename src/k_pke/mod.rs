use crate::{hash_functions::HashFunctions, sampling_algorithms::SamplingAlgorithms, sets::{B, Circle, Znk, Znkxk}};

pub struct KPke<const K: usize>;

impl<const K: usize> KPke<K> {
    // Page 29
    pub fn key_gen<const Q: i64, const ETA: usize>(d: B<32>) -> (B<{(K*384) + 32}>, B<{K*384}>) {
        let (p, sigma) = HashFunctions::g([d.to_vec(), [K as u8].to_vec()].concat());
        let mut n = 0;

        let mut a = Znkxk::<Q, 256, K>::new_empty();
        for i in 0..K {
            for j in 0..K {
                a[i][j] = SamplingAlgorithms::sample_ntt(p, j as u8, i as u8);
            }
        }

        let mut s = Znk::<Q, 256, K>::new_empty();
        for i in 0..K {
            s[i] = SamplingAlgorithms::sample_poly_cbd::<Q, ETA>(HashFunctions::prf::<ETA>(&sigma, n));
            n = n + 1;
        }

        let mut e = Znk::<Q, 256, K>::new_empty();
        for i in 0..K {
            e[i] = SamplingAlgorithms::sample_poly_cbd::<Q, ETA>(HashFunctions::prf::<ETA>(&sigma, n));
            n = n + 1;
        }

        let s = s.map(|s_n| HashFunctions::ntt::<Q>(&s_n));
        let e = e.map(|e_n| HashFunctions::ntt::<Q>(&e_n));
        let t = a.circle(s) + e;

        todo!();
    }
}
