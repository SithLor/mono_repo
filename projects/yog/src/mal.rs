pub fn _str_to_vec_u8(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}
pub fn _vec_u8_to_str(v: Vec<u8>) -> String {
    String::from_utf8(v).unwrap()
}

pub fn _vec_u8_hex_vec_u8(v: Vec<u8>) -> Vec<u8> {
    v.iter().map(|&x| x).collect()
}
pub fn _vec_u8_hex_str(v: Vec<u8>) -> String {
    let mut s = String::new();
    for &x in &v {
        s.push_str(&format!("{:02x}", x));
    }
    s
}

