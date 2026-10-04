//https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.203.pdf
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

mod ml_kem;
mod sets;
mod k_pke;
mod hash_functions;
mod sampling_algorithms;

use ml_kem::MlKemTypes;

fn main() {
    let crypto = MlKemTypes::MlKem512.new();
    let (encryption_key, decryption_key) = crypto.key_gen();
    println!("{:?}", encryption_key);
    println!("{:?}", decryption_key);
}

