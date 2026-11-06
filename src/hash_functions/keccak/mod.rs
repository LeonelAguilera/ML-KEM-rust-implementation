use crate::sets::{B, BK, Len, NewEmpty};

//Page 20
//keccak<512, 256, N_LEN>(n) -> B<D>
pub fn keccak<const C: usize, const D: usize, const N_LEN: usize>(n: &B<N_LEN>) -> B<{D/8}>
where 
    [(); {1600 - C}]:,
    [(); { (1 + (N_LEN / (1600 - C))) * (1600 - C) }]:,
    [(); { (1 + (N_LEN / (1600 - C))) * (1600 - C) / (1600 - C) }]:,
{
    return sponge::<D, {1600 - C}, 1600, N_LEN>(keccak_p::<1600, 24>, n).bits_to_bytes();
}

//Page 19
fn sponge<const D: usize, const R: usize, const BFW: usize, const N_LEN: usize>(f: fn(B<BFW>)->B<BFW>, n: &B<N_LEN>) -> B<D>
where 
    [(); {(1 + (N_LEN / R)) * R}]:,
    [(); {(1 + (N_LEN / R)) * R / R}]:,
{
    let p: B<{(1 + (N_LEN / R)) * R}> = n.pad::<B<{(1 + (N_LEN / R)) * R}>>(|index| {
        if (index == N_LEN) || (index == ((R * (1 + (N_LEN / R))) - 1)) {
            1
        }
        else {
            0
        }
    });
    let n = p.len() / R;

    let p0n: BK<R, {((1 + (N_LEN / R))) * R / R}> = BK::<R, {((1 + (N_LEN / R))) * R / R}>::from(p);
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

//Page 19
fn pad10x1<const X: usize, const M: usize>() -> B<{(M+2) % X}> {
    let mut output_array = B::new_empty();
    output_array[0] = 1;
    output_array[((M + 2) % X) - 1] = 1;
    return output_array;
}

//Page 17
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

//Page 16
fn rnd<const W: usize>(a: [[[u8; 5]; 5]; W], ir: usize) -> [[[u8; 5]; 5]; W] {
    return iota(xi(pi(rho(theta(a)))), ir);
}

//Page 16
fn iota<const W: usize>(a: [[[u8; 5]; 5]; W], ir: usize) -> [[[u8; 5]; 5]; W] {
    let l = usize::ilog2(W) as usize;
    let mut ap = [[[0; 5]; 5]; W];

    for x in 0..5 {
        for y in 0..5 {
            for z in 0..W {
                ap[z][y][x] = a[z][y][x];
            }
        }
    }

    let mut rc = B::<W>::new_empty();

    for j in 0..l {
        rc[(1 << j) - 1] = rc_func(j + (7*ir));
    }

    for z in 0..W {
        ap[z][0][0] = ap[z][0][0] ^ rc[z];
    }

    return ap;
}

//Page 16
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

//Page 15
fn xi<const W: usize>(a: [[[u8; 5]; 5]; W]) -> [[[u8; 5]; 5]; W] {
    let mut ap = [[[0; 5]; 5]; W];

    for x in 0..5 {
        for y in 0..5 {
            for z in 0..W {
                ap[z][y][x] = a[z][y][x] ^ ((a[z][y][(x + 1).rem_euclid(5)] ^ 1) & a[z][y][(x+2).rem_euclid(5)]);
            }
        }
    }

    return ap;
}

//Page 14
fn pi<const W: usize>(a: [[[u8; 5]; 5]; W]) -> [[[u8; 5]; 5]; W] {
    let mut ap = [[[0; 5]; 5]; W];

    for x in 0..5 {
        for y in 0..5 {
            for z in 0..W {
                ap[z][y][x] = a[z][y][(x + (3 * y)) % 5];
            }
        }
    }

    return ap;
}

//Page 12
fn rho<const W: usize>(a: [[[u8; 5]; 5]; W]) -> [[[u8; 5]; 5]; W] {
    let mut ap = [[[0; 5]; 5]; W];

    let (mut x, mut y) = (1, 0);

    for t in 0..=23 {
        for z in 0..W {
            ap[z][y][x] = a[(z.wrapping_sub((t + 1) * (t + 2) / 2)) % W][y][x];
            (x, y) = (y, ((2 * x) + (3 * y)) % 5);
        }
    }

    return ap;
}

//Page 11
fn theta<const W: usize>(a: [[[u8; 5]; 5]; W]) -> [[[u8; 5]; 5]; W] {
    let mut c = [[0; 5]; W];
    let mut d = [[0; 5]; W];
    let mut ap = [[[0; 5]; 5]; W];

    for x in 0..5 {
        for z in 0..W {
            c[z][x] = a[z][0][x] ^ a[z][1][x] ^ a[z][2][x] ^ a[z][3][x] ^ a[z][4][x];
        }
    }

    for x in 0..5 {
        for z in 0..W {
            d[z][x] = c[z][(x + 4).rem_euclid(5)] ^ c[(z + 4).rem_euclid(5)][(x + 1).rem_euclid(5)];
        }
    }

    for x in 0..5 {
        for y in 0..5 {
            for z in 0..W {
                ap[z][y][x] = a[z][y][x] ^ d[z][x];
            }
        }
    }

    return ap;
}

//Page 9
fn string_to_state_array<const BFW: usize>(s: B<BFW>) -> [[[u8; 5]; 5]; {BFW / 25}] {
    let w = BFW / 25;
    println!("bfw: {BFW} w: {w}");
    let mut a: [[[u8; 5]; 5]; {BFW / 25}] = [[[0; 5]; 5]; {BFW / 25}];
    for x in 0..5 {
        for y in 0..5 {
            for z in 0..w {
                a[z][y][x] = s[(w * ((5 * y) + x)) + z];
            }
        }
    }

    return a;
}

//Page 10
fn state_array_to_string<const BFW: usize>(a: [[[u8; 5]; 5]; {BFW / 25}]) -> B<BFW> {
    let w = BFW / 25;
    let mut s: B<BFW> = B::new_empty();
    for x in 0..5 {
        for y in 0..5 {
            for z in 0..w {
                s[(w * ((5 * y) + x)) + z] = a[z][y][x];
            }
        }
    }

    return s;
}
