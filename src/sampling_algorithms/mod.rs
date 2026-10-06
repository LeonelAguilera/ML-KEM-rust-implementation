use crate::sets::{B, Zn};

pub struct SamplingAlgorithms;
impl SamplingAlgorithms {
    pub fn sample_ntt<const Q: i64>(b: B<32>, i: u8, j: u8) -> Zn<Q, 256> {
        let byte_array = b.append(&B::<1>::new_populated(i)).append(&B::<1>::new_populated(j));
        todo!();
    }
    pub fn sample_poly_cbd<const Q: i64, const ETA: usize>(b: Vec<u8>) -> Zn<Q, 256> {
        assert_eq!(b.len(), 64*ETA);
        todo!();
    }
}
