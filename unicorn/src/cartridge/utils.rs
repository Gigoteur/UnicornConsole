pub fn read_u8(v: &mut Vec<u8>) -> usize {
    let u: Vec<_> = v.drain(0..2).collect();

    ((u[1] as usize) << 4) | u[0] as usize
}