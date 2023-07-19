pub fn as_u32(array: Vec<u8>) -> u32 {
    ((array[0] as u32) <<  0) +
    ((array[1] as u32) <<  8) +
    ((array[2] as u32) << 16) +
    ((array[3] as u32) << 24)
}

pub fn string_as_24_bytes(str: String) -> Vec<u8> {
    let mut a = str.as_bytes().to_vec();
    while a.len() > 24 {
        _ = a.pop();
    }
    while a.len() < 24 {
        // 32 is whitespace
        a.push(32);
    }
    a
}