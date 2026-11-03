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
        s = f(s ^ p0n[i].pad(|_|0));
    }

    let mut z = B::new_empty();

    for counter in 0..(D/R) {
        for i in 0..R {
            z[(counter * R) + i] = s[i];
        }
        s = f(s);
    }

    return z;
}

fn pad10x1<const X: usize, const M: usize>() -> B<{(M+2) % X}> {
    let mut output_array = B::new_empty();
    output_array[0] = 1;
    output_array[((M + 2) % X) - 1] = 1;
    return output_array;
}

fn keccak_p<const BFW: usize, const NR: usize>(s: B<BFW>) -> B<BFW>
where
    [(); {BFW / 25}]:,
{
    let w = BFW / 25;
    let l = usize::ilog2(w) as usize;

    let mut a: [[[u8; 5]; 5]; {BFW / 25}] = string_to_state_array(s);

    for ir in (12 + (2 * l) - NR)..(12 + (2*l)) {
        a = rnd(a, ir);
    }

    let sp = state_array_to_string(a);

    return sp;
}

fn rnd<const W: usize>(a: [[[u8; 5]; 5]; W], ir: usize) -> [[[u8; 5]; 5]; W] {
    return iota(xi(pi(rho(theta(a)))), ir);
}

fn iota<const W: usize>(a: [[[u8; 5]; 5]; W], ir: usize) -> [[[u8; 5]; 5]; W] {
    let l = usize::ilog2(W) as usize;
    let mut ap = [[[0; 5]; 5]; W];

    for x in 0..5 {
        for y in 0..5 {
            for z in 0..W {
                ap[x][y][z] = a[x][y][z];
            }
        }
    }

    let mut rc = B::<W>::new_empty();

    for j in 0..l {
        rc[(1 << j) - 1] = rc_func(j + (7*ir));
    }

    for z in 0..W {
        ap[0][0][z] = ap[0][0][z] ^ rc[z];
    }

    return ap;
}

fn rc_func(t: usize) -> u8 {
    let t_mod_255 = t.rem_euclid(255);
    if t_mod_255 == 0 {
        return 1;
    }
    let mut r = B::<9>::new_empty();
    r[0] = 1;

    for i in 1..t_mod_255 {
        for temp in 8..1 {
            r[temp] = r[temp - 1];
        }
        r[0] = 1;

        r[0] = r[0] ^ r[8];
        r[4] = r[4] ^ r[8];
        r[5] = r[5] ^ r[8];
        r[6] = r[6] ^ r[8];
    }
    
    return r[0];
}

fn xi<const W: usize>(a: [[[u8; 5]; 5]; W]) -> [[[u8; 5]; 5]; W] {
    let mut ap = [[[0; 5]; 5]; W];

    for x in 0..5 {
        for y in 0..5 {
            for z in 0..W {
                ap[x][y][z] = a[x][y][z] ^ ((a[(x + 1).rem_euclid(5)][y][z] ^ 1) & a[(x+2).rem_euclid(5)][y][z]);
            }
        }
    }

    return ap;
}

fn pi<const W: usize>(a: [[[u8; 5]; 5]; W]) -> [[[u8; 5]; 5]; W] {
    let mut ap = [[[0; 5]; 5]; W];

    for x in 0..5 {
        for y in 0..5 {
            for z in 0..W {
                ap[x][y][z] = a[(x + (3 * y)) % 5][y][z];
            }
        }
    }

    return ap;
}

fn rho<const W: usize>(a: [[[u8; 5]; 5]; W]) -> [[[u8; 5]; 5]; W] {
    let mut ap = [[[0; 5]; 5]; W];

    let (mut x, mut y) = (1, 0);

    for t in 0..=23 {
        for z in 0..W {
            ap[x][y][z] = a[x][y][(z - ((t + 1) * (t + 2) / 2)) % W];
            (x, y) = (y, ((2 * x) + (3 * y)) % 5);
        }
    }

    return ap;
}

fn theta<const W: usize>(a: [[[u8; 5]; 5]; W]) -> [[[u8; 5]; 5]; W] {
    let mut c = [[0; 5]; W];
    let mut d = [[0; 5]; W];
    let mut ap = [[[0; 5]; 5]; W];

    for x in 0..5 {
        for z in 0..W {
            c[x][z] = a[x][0][z] ^ a[x][1][z] ^ a[x][2][z] ^ a[x][3][z] ^ a[x][4][z];
        }
    }

    for x in 0..5 {
        for z in 0..W {
            d[x][z] = c[(x - 1).rem_euclid(5)][z] ^ c[(x + 1).rem_euclid(5)][(z - 1).rem_euclid(5)];
        }
    }

    for x in 0..5 {
        for y in 0..5 {
            for z in 0..W {
                ap[x][y][z] = a[x][y][z] ^ d[x][z];
            }
        }
    }

    return ap;
}

fn string_to_state_array<const BFW: usize>(s: B<BFW>) -> [[[u8; 5]; 5]; {BFW / 25}] {
    let w = BFW / 25;
    let mut a: [[[u8; 5]; 5]; {BFW / 25}] = [[[0; 5]; 5]; {BFW / 25}];
    for x in 0..5 {
        for y in 0..5 {
            for z in 0..w {
                a[x][y][z] = s[(w * ((5 * y) + x)) + z];
            }
        }
    }

    return a;
}

fn state_array_to_string<const BFW: usize>(a: [[[u8; 5]; 5]; {BFW / 25}]) -> B<BFW> {
    let w = BFW / 25;
    let mut s: B<BFW> = B::new_empty();
    for x in 0..5 {
        for y in 0..5 {
            for z in 0..w {
                s[(w * ((5 * y) + x)) + z] = a[x][y][z];
            }
        }
    }

    return s;
}
