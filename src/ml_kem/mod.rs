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

impl MlKemDyn {
    pub fn key_gen(&self) -> (Vec<u8>, Vec<u8>) {
        match self {
            MlKemDyn::MlKem512(ml_kem) => ml_kem.key_gen(),
            MlKemDyn::MlKem768(ml_kem) => ml_kem.key_gen(),
            MlKemDyn::MlKem1024(ml_kem) => ml_kem.key_gen(),
        }
    }
}

pub struct MlKem<const N: u64, const Q: i64, const K: usize, const ETA1: usize, const ETA2: u64, const DU: u64, const DV: u64>;

impl<const N: u64, const Q: i64, const K: usize, const ETA1: usize, const ETA2: u64, const DU: u64, const DV: u64> MlKem<N, Q, K, ETA1, ETA2, DU, DV> {
    // Page 35
    pub fn key_gen(&self) -> (Vec<u8>, Vec<u8>)
    where 
        [(); (K*384) + 32]:,
        [(); (K*768) + 96]:
    {
        let d = B::<32>::new_random();
        let z = B::<32>::new_random();
        
        let (ek, dk) = self.key_gen_internal(d, z);
        return (ek.to_vec(), dk.to_vec());
    }
    
    // Page 32
    fn key_gen_internal(&self, d: B<32>, z: B<32>) -> (B<{(K*384) + 32}>, B<{(K*768) + 96}>) {
        let (ek_pke, dk_pke) = KPke::<K>::key_gen::<Q, ETA1>(d);
        let ek = ek_pke;
        let dk = Self::get_dk(dk_pke, ek, HashFunctions::h(&ek), z);

        return (ek, dk);
    }

    // Page 32 - line 3
    fn get_dk(dk_pke: B<{K*384}>, ek: B<{(K*384) + 32}>, h: B<32>, z: B<32>) -> B<{(K*768) + 96}> {
        let mut whole = B::new_empty();
        let mut index_offset = 0;
        for i in 0..(K*384) {
            whole[i + index_offset] = dk_pke[i];
        }
        index_offset += K*384;

        for i in 0..((K*384) + 32) {
            whole[i + index_offset] = ek[i];
        }
        index_offset += (K*384) + 32;

        for i in 0..32 {
            whole[i + index_offset] = h[i];
        }
        index_offset += 32;

        for i in 0..32 {
            whole[i + index_offset] = z[i];
        }

        return whole;
    }
}

