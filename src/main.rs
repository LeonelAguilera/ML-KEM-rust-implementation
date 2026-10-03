//https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.203.pdf

mod ml_kem;
mod sets;
mod k_pke;
mod hash_functions;
mod sampling_algorithms;

use ml_kem::MlKemTypes;

fn main() {
    let crypto = MlKemTypes::MlKem512.new();
}

