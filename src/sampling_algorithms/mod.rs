use crate::sets::{B, Zn};

pub struct SamplingAlgorithms;
impl SamplingAlgorithms {
    pub fn sample_ntt<const Q: u64>(b: Vec<u8>) -> Zn<Q, 256> {
        assert_eq!(b.len(), 34);
        todo!();
    }
    pub fn sample_poly_cbd<const Q: u64, const ETA: usize>(b: Vec<u8>) -> Zn<Q, 256> {
        assert_eq!(b.len(), 64*ETA);
        todo!();
    }
}
