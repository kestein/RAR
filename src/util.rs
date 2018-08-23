

pub fn get_bit_at(input: u64, n: u64) -> bool {
    if n < 64 {
        input & (1 << n) != 0
    } else {
        false
    }
}
#[test]
fn test_get_bit_at() {
    assert_eq!(get_bit_at(0xFF , 1), true);
    assert_eq!(get_bit_at(0x7F, 7), false);
    assert_eq!(get_bit_at(0x7F, 6), true);
    assert_eq!(get_bit_at(0x00, 0), false);
    assert_eq!(get_bit_at(0x01, 0), true);
    assert_eq!(get_bit_at(0x02, 0), false);
    assert_eq!(get_bit_at(0x02, 1), true);
}


pub fn split_u32(x: u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}
#[test]
fn test_split_u32() {
    assert_eq!(split_u32(0x00000000), [0x00, 0x00, 0x00, 0x00]);
    assert_eq!(split_u32(0x000000FF), [0x00, 0x00, 0x00, 0xFF]);
    assert_eq!(split_u32(0x0000FF00), [0x00, 0x00, 0xFF, 0x00]);
    assert_eq!(split_u32(0x00FF0000), [0x00, 0xFF, 0x00, 0x00]);
    assert_eq!(split_u32(0xFF000000), [0xFF, 0x00, 0x00, 0x00]);
    assert_eq!(split_u32(0x0F0F0F0F), [0x0F, 0x0F, 0x0F, 0x0F]);
}

pub fn split_u64(x: u64) -> [u8; 8] {
    let x = x.clone();
    let b1 : u8 = ((x >> 56) & 0xff) as u8;
    let b2 : u8 = ((x >> 48) & 0xff) as u8;
    let b3 : u8 = ((x >> 40) & 0xff) as u8;
    let b4 : u8 = ((x >> 32) & 0xff) as u8;
    let b5 : u8 = ((x >> 24) & 0xff) as u8;
    let b6 : u8 = ((x >> 16) & 0xff) as u8;
    let b7 : u8 = ((x >> 8) & 0xff) as u8;
    let b8 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4, b5, b6, b7, b8];
}
#[test]
fn test_split_u64() {
    assert_eq!(split_u64(0x00000000), [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(split_u64(0x000000FF), [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF]);
    assert_eq!(split_u64(0x0000FF00), [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00]);
    assert_eq!(split_u64(0x00FF0000), [0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00]);
    assert_eq!(split_u64(0xFF000000), [0x00, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00]);
    assert_eq!(split_u64(0x0F0F0F0F), [0x00, 0x00, 0x00, 0x00, 0x0F, 0x0F, 0x0F, 0x0F]);
}

pub fn split_u64_t(x: u64) -> [u8; 8] {
    let x = x.clone();
    let b1 : u8 = ((x >> 56) & 0xff) as u8;
    let b2 : u8 = ((x >> 48) & 0xff) as u8;
    let b3 : u8 = ((x >> 40) & 0xff) as u8;
    let b4 : u8 = ((x >> 32) & 0xff) as u8;
    let b5 : u8 = ((x >> 24) & 0xff) as u8;
    let b6 : u8 = ((x >> 16) & 0xff) as u8;
    let b7 : u8 = ((x >> 8) & 0xff) as u8;
    let b8 : u8 = (x & 0xff) as u8;
    return [b8, b7, b6, b5, b4, b3, b2, b1];
}
#[test]
fn test_split_u64_t() {
    assert_eq!(split_u64_t(0x00000000), [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(split_u64_t(0x000000FF), [0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(split_u64_t(0x0000FF00), [0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(split_u64_t(0x00FF0000), [0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(split_u64_t(0xFF000000), [0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(split_u64_t(0x0F0F0F0F), [0x0F, 0x0F, 0x0F, 0x0F, 0x00, 0x00, 0x00, 0x00]);
}


/*
    let data = [
        0xF3, 0xE1, 0x82, 0xEB, 0x0B, 0x01, 0x05, 0x07,
        0x00, 0x06, 0x01, 0x01, 0x80, 0x80, 0x80, 0x00, 0x8C, 
        0x0D, 0x88, 0xE2, 0x24, 0x02, 0x03, 0x0B, 0xC6, 0x10, 
        0x04, 0xC6, 0x10, 0x20, 0x93, 0xF2, 0x9A, 0xCB, 0x80, 
        0x00, 0x00, 0x08, 0x74, 0x65, 0x78, 0x74, 0x2E, 0x74, 
        0x78, 0x74, 0x0A, 0x03, 0x02, 0x78, 0x27, 0x3C, 0x1E, 
        0x7D, 0xF2, 0xD3
    ];
    */