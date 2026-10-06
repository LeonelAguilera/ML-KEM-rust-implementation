use crate::sets::{B, Zn};

pub fn byte_encode<const D: usize, const M: i64>(f: Zn<M, 256>) -> B<{32 * D}>
where 
    [(); {256 * D}]:
{
    assert!(D <= 12, "Function `byte_encode` called with wrong D parameter");
    if D < 12 {assert_eq!(M as u16, 2_u16.pow(D as u32), "M value incorrect for D selection")}
    for i in 0..256 {
        let a = f[i];
        let mut b = [0_u8; {256 * D}];
        for j in 0..D {
            b[(i * D) + j] = a.0;
        }
    }
    todo!();
}
