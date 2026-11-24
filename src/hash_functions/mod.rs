mod keccak;

use std::ops::Rem;

use num_bigint::{BigInt, Sign};

use crate::{hash_functions::keccak::keccak, sets::{B, Len, Z, Zn}};

pub struct HashFunctions {}
impl HashFunctions {
    pub fn shake256(s: Vec<u8>, output_length_in_bits: usize) -> Vec<u8>{
        todo!();
    }
    //SHA3-256
    pub fn h<const N: usize>(s: &B<N>) -> B<32>
    where 
        [(); {(1 + ((N + 2) / (1600 - 512))) * (1600 - 512)}]:,
    [(); {(1 + ((N + 2) / (1600 - 512))) * (1600 - 512) / (1600 - 512) }]:,
    [(); {N + 2}]:,
    {
        return keccak::<512, 256, {N + 2}>(&s.append(&B::from([0,1])));
    }
    //SHAKE256
    pub fn j(s: Vec<u8>) -> B<32> {
        todo!();
    }
    //SHA3-512
    pub fn g(s: Vec<u8>) -> (B<32>, B<32>) {
        todo!();
    }
    pub fn prf<const ETA: usize>(s: &B<32>, b: u8,) -> Vec<u8> {
        return Self::shake256([s.to_vec(), [b].to_vec()].concat(), 8 * 64 * ETA);
    }
    pub fn ntt<const Q: i64>(f: &Zn<Q, 256>) -> Zn<Q, 256> {
        let mut f_hat = f.clone();
        let mut i = 1;
        let mut len = 128;
        while len >= 2 {
            for start in (0..256).step_by(2 * len) {
                let zeta = HashFunctions::compute_ntt_zeta::<Q>(i);
                i += 1;
                for j in start..(start + len) {
                    let t = zeta * f_hat[j + len];
                    f_hat[j + len] = f_hat[j] - t;
                    f_hat[j] = f_hat[j] + t;
                }
            }
            len /= 2;
        }
        return f_hat;
    }
    pub fn ntt_inv<const Q: i64>(f_hat: &Zn<Q, 256>) -> Zn<Q, 256> {
        let mut f = f_hat.clone();
        let mut i = 127;
        let mut len = 2;
        while len <= 128 {
            for start in (0..256).step_by(2 * len) {
                let zeta = HashFunctions::compute_ntt_zeta::<Q>(i);
                i -= 1;
                for j in start..(start + len) {
                    let t = f[j];
                    f[j] = t + f[j + len];
                    f[j + len] = zeta * (f[j + len] - t);
                }
            }
            len *= 2;
        }
        for k in 0..(f.len()) {
            f[k] = f[k] * Z::<Q>::new(3303);
        }
        return f;
    }

    fn bit_reverse(num: u8) -> u8 {
        let mut binary = [0; 7];
        let mut num_internal = num;

        for i in (0..7).rev() {
            if num_internal >= 2u8.pow(i as u32) {
                binary[i] = 1;
                num_internal -= 2u8.pow(i as u32);
            }
        }
        for i in 0..7 {
            num_internal += (2u8.pow(i as u32)) * binary[6 - i];
        }
        return num_internal;
    }

    fn compute_ntt_zeta<const Q: i64>(i: u8) -> Z<Q> {
        let exponent = HashFunctions::bit_reverse(i);
        let base = BigInt::new(Sign::Plus, vec![17]);

        let num = base.pow(exponent as u32);
        let zeta: BigInt = num.rem(3329);
        let zeta = zeta.to_u64_digits().1[0];

        return Z::<Q>::new(zeta as i64);
    }
}
