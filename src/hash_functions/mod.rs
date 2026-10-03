use crate::sets::{B, Zn};

pub struct HashFunctions {}
impl HashFunctions {
    pub fn shake256(s: Vec<u8>, output_length_in_bits: usize) -> Vec<u8>{
        todo!();
    }
    //SHA3-256
    pub fn h(s: Vec<u8>) -> B<32> {
        todo!();
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
        todo!();
    }
}
