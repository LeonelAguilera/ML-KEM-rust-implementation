# ML-KEM Rust Implementation  
> _A learning‑first implementation of the **Kyber‑ML KEM** reference model, ready to be ported to VHDL for an in‑line VPN gateway._

---

This repository contains a **stand-alone Rust crate** that implements the ML-KEM key encapsulation mechanism, based on the standard published in [https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.203.pdf](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.203.pdf).
The goal is two-fold:

1. **Learn** the inner workings of post-quantum cryptography by implementing it in a high-level, type-safe language.
2. Provide a **ready-to-port** foundation that can be (manually) translated into VHDL/Verilog for an FPGA-based VPN gateway that encrypts traffic in-line for the "Digital Design Project" subject at LiU.

---

## API

The user exposed interface is based on the `MlKemTypes` enum, which allows the user to choose between ML-KEM512, ML-KEM768 and ML-KEM1024.\
The provided methods are:
- `key_gen() -> (Vec<u8>, Vec<u8>)`: Generates a pair of random keys. Note: since this is a test implementation, the default random generator from the `rand` crate was used, which is not criptographically secure.
- `encaps()`: TBD
- `decaps()`: TBD

