//https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.203.pdf
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

#![allow(dead_code)]
#![allow(unused)]

mod ml_kem;
mod sets;
mod k_pke;
mod hash_functions;
mod sampling_algorithms;
mod general_algorithms;

use ml_kem::MlKemTypes;

use crate::{hash_functions::HashFunctions, sets::B};

fn main() {
    let to_be_shaed = B::from([0]);
    let shaed = HashFunctions::h(&to_be_shaed);
    println!("{:?}", shaed);
    let crypto = MlKemTypes::MlKem512.new();
    let (encryption_key, decryption_key) = crypto.key_gen();
    println!("{:?}", encryption_key);
    println!("{:?}", decryption_key);
}

