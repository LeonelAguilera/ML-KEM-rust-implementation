use std::fs;

use crate::{hash_functions::HashFunctions, sets::{Len, NewEmpty, Zn}};

pub fn create_ntt_testbench() {
    let mut ntt_file = String::new();

    let f = Zn::<3329, 256>::new_random();
    let f_hat = HashFunctions::ntt(&f);

    for i in 0..(f.len() - 1) {
        ntt_file += &format!("{}, ", f[i]);
    }
    ntt_file += &format!("{}; ", f[f.len() - 1]);

    for i in 0..(f_hat.len() - 1) {
        ntt_file += &format!("{}, ", f_hat[i]);
    }
    ntt_file += &format!("{}\n", f_hat[f_hat.len() - 1]);

    let _ = fs::write("ntt.txt", ntt_file);
}
