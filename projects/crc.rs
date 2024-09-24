use std::sync::Once;

//nameing 
//i_ mean internal


pub static INIT: Once = Once::new();
pub static mut CRC_TABLE: Option<Vec<u32>> = None;
/// use if like function to init manually
pub mod crc32 {
    use super::CRC_TABLE;
    fn i_make_crc_table() -> Vec<u32> {
        let mut crc_table = Vec::with_capacity(256);
        for i in 0..256 {
            let mut crc = i;
            for _ in 0..8 {
                if crc & 1 == 1 {
                    crc = (crc >> 1) ^ 0xEDB88320;
                } else {
                    crc >>= 1;
                }
            }
            crc_table.push(crc);
        }
        crc_table
    }
    fn i_gen_crc(data: &[u8]) -> u32 {
        let mut crc = 0xFFFFFFFF;
        for &byte in data {
            let index = ((crc ^ (byte as u32)) & 0xFF) as usize;
            crc = (crc >> 8) ^ unsafe { CRC_TABLE.as_ref().unwrap()[index] };
        }
        crc ^ 0xFFFFFFFF
    }
    
    fn i_check_crc(data: &[u8], expected_crc: u32) -> bool {
        i_gen_crc(data) == expected_crc
    }
    pub fn init(){
        unsafe {
            super::INIT.call_once(|| {
                super::CRC_TABLE = Some(i_make_crc_table());
            });
        }
    }
    pub fn calculate_crc(data: &[u8]) -> u32 {
        let mut _data:u32 = 0;
        unsafe{_data = i_gen_crc(data)}
        return _data;
    }
    pub fn check_crc(data: &[u8], expected_crc: u32) -> bool {
        let mut _data:bool = false;
        unsafe{_data = i_check_crc(data, expected_crc)}
        return _data;
    }
}

pub struct Crc32 {}
impl Crc32 {
    pub fn new()->Crc32{
        crc32::init();
        Crc32{}
    }
    pub fn calculate_crc(&self, data: &[u8]) -> u32 {
        crc32::calculate_crc(data)
    }
    pub fn check_crc(&self, data: &[u8], expected_crc: u32) -> bool {
        crc32::check_crc(data, expected_crc)
    }
}