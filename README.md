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

## Note to anyone who isn't experienced in rust but wants to understand this code

You will notice in multiple functions a structure that repeats with a shape similar to this:
```Rust
fn my_func<const K: usize>(params: type) -> Something 
where                   // <-- This part
    [(); K * N + 32]:,  // <-- This part
{
    logic.run();
}
```

I recommend you dear reader to just ignore them as they are mostly meaningless,
but, if you must understand what is going on, well...\
This is the consequence of a very unlucky intersection between my needs and
desires, and the current state of rust development. The goal of this
implementation is not for it to simply be a working encapsulator, but to trace
the path of an HDL implementation, where dynamic memory allocations don't exist,
for this reason, I vouched to only use statically sized data-types, including
arrays.\
If I was trying to implement not more than one flavor of ML-KEM this would be
trivial, as in the source material the size of each array is predefined, but,
the goal instead is to write the code once, and then, by the use of generics,
select between ML-KEM512, 768 or 1024.\
To transfer this behavior to rust, generic constants are needed (similar to C
int templates), so now, an array is no longer defined as simple `[u8; 384]`,
but instead `[u8; {384 * K}]`, and this is where the problems begin.\
The current release of the stable rust compiler only supports the trivial case
for the use of generic constants, this being `[u8; K]` (i.e. Only the constant,
no operations performed). For this reason, I was forced to move the toolchain
to the expermental "nightly" release (this is the reason for the inclusion of
the `toolchain.toml` file).\
Now, how does this relate to the `where` keyword and the weird `[(); K...]`
syntax?. Currently, the compiler can only handle trivial operations, which may
seem enough, but it is not hard to see how calculating the final output size
of a type becomes harder once you start concatenating operations, each one with
its own different expresion for the calculation of the final output size.\
For example, let's define:
```Rust
fn function_a<const K: usize>(a: irrelevant) -> [u8; {K * 256}] {}

fn function_b(a: irrelevant) -> [u8; 32] {}

fn join_arrays<const A: usize, const B: usize>(lhs: [u8; A], rhs: [u8; B]) -> [u8; {A + B}] {}

fn disaster<const K: usize>(a: irrelevant, b: irrelevant) -> [u8; {(256 * K) + 32}] {
    let my_var = function_a::<K>::(a);
    let sufix = function_b(b);

    let output_val = join_arrays(my_var, sufix);

    return output_val;
}
```

It is easy for us to see how this is valid code, since `K * 256 + 32 == (256 * K) + 32`.
However, this isn't that easy to see for the compiler which, first of all, has
to determine if the expressions we are using are even computable. On the first
pass, what the compiler sees is that the size of `output_val` is `A + B`, which
looks nothing like `(256 * K) + 32`, meaning, it will error.\
For this reason, as an (in principle) temporary solution, they created that
weird syntax from the begining that just means "Trust me", and is used to tell
the compiler "I promise you will be able to evaluate my expression".\
So, TL;DR: It is a syntax workarround that the rust wizards made up while they
figure out how to make this work.
