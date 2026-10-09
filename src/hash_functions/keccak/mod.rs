use crate::sets::{B, BK, NewEmpty};

fn keccak<const C: usize, const D: usize>(n: Vec<u8>) -> B<D> {
    todo!();
}

fn sponge<const D: usize, const R: usize, const BFW: usize, const PAD_LEN: usize, const N_LEN: usize>(f: fn(B<BFW>)->B<BFW>, pad: B<PAD_LEN>, n: B<N_LEN>) -> B<D>
where 
    [(); {N_LEN + PAD_LEN}]:,
    [(); {(N_LEN + PAD_LEN) / R}]:,
    [(); {BFW - R}]:,
{
    let p: B<{N_LEN + PAD_LEN}> = n.append(&pad);
    let n = p.len() / R;
    let p0n: BK<R, {(N_LEN + PAD_LEN) / R}> = BK::from(p);
    let mut s = B::<BFW>::new_empty();

    for i in 0..n {
        s = f(s ^ p0n[i].pad(0));
    }

    todo!();
}

fn pad10x1<const X: usize, const M: usize>() -> B<{(M+2) % X}> {
    let mut output_array = B::new_empty();
    output_array[0] = 1;
    output_array[((M + 2) % X) - 1] = 1;
    return output_array;
}
