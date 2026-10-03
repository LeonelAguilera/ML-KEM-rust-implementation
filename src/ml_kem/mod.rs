use crate::{hash_functions::HashFunctions, k_pke::KPke, sets::B};

pub enum MlKemTypes{
    MlKem512,
    MlKem768,
    MlKem1024,
}

impl MlKemTypes {
    pub fn new(&self) -> MlKemDyn {
        match self {
            MlKemTypes::MlKem512 => MlKemDyn::MlKem512(MlKem),
            MlKemTypes::MlKem768 => MlKemDyn::MlKem512(MlKem),
            MlKemTypes::MlKem1024 => MlKemDyn::MlKem512(MlKem),
        }
    }
}

pub enum MlKemDyn {
    MlKem512(MlKem<256, 3329, 2, 3, 2, 10, 4>),
    MlKem768(MlKem<256, 3329, 3, 2, 2, 10, 4>),
    MlKem1024(MlKem<256, 3329, 4, 2, 2, 11, 5>),
}

pub struct MlKem<const N: u64, const Q: u64, const K: usize, const ETA1: usize, const ETA2: u64, const DU: u64, const DV: u64>;

impl<const N: u64, const Q: u64, const K: usize, const ETA1: usize, const ETA2: u64, const DU: u64, const DV: u64> MlKem<N, Q, K, ETA1, ETA2, DU, DV> {
    pub fn key_gen(&self) -> (Vec<u8>, Vec<u8>) {
        let d = B::<32>::new_random();
        let z = B::<32>::new_random();
        
        let (ek, dk) = self.key_gen_internal(d, z);
        assert_eq!(ek.len(), K*384 + 32);
        assert_eq!(dk.len(), K*768 + 96);
        return (ek, dk);
    }

    fn key_gen_internal(&self, d: B<32>, z: B<32>) -> (Vec<u8>, Vec<u8>) {
        let (ek_pke, dk_pke) = KPke::<K>::key_gen::<Q, ETA1>(d);
        let ek = ek_pke;
        let dk = [dk_pke, ek.clone(), HashFunctions::h(ek.clone()).to_vec(), z.to_vec()].concat();

        return (ek, dk);
    }
}
