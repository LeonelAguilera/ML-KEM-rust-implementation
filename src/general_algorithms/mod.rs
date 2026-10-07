use crate::sets::{B, NewEmpty, Zn};

// Page: 22
pub fn byte_encode<const D: usize, const M: i64>(f: &Zn<M, 256>) -> B<{D * 256 / 8}>
where 
    [(); {D * 256}]:,
{
    assert!(D <= 12, "Function `byte_encode` called with wrong D parameter: D > 12");
    if D < 12 {assert_eq!(M as u16, 2_u16.pow(D as u32), "M value incorrect for D selection")}

    let mut f = f.clone();
    let mut b = Zn::<M, {D * 256}>::new_empty();

    for i in 0..256 {
        let a = f[i];
        for j in 0..D {
            b[(i * D) + j] = a.rem_euclid(2);
            f[i] = (a - b[(i * D) + j]);
        }
    }

    let b = B::<{D * 256}>::from(b);
    let k: B<{D * 256 / 8}> = bits_to_bytes::<{D * 256}>(b);
    return k;
}

// Page: 20
pub fn bits_to_bytes<const L: usize>(b: B<L>) -> B<{L / 8}> {
    assert_eq!(L.rem_euclid(8), 0, "Bit array length is not a multiple of 8");

    let mut b_out = B::<{L/8}>::new_empty();
    for i in 0..L {
        b_out[i/8] += b[i] * 2_u8.pow(i.rem_euclid(8) as u32);
    }
    return b_out;
}
