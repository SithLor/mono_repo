pub mod link;
pub mod color;
pub mod mal;




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color(){
        {
            
            let c_u8: color::RGBA<u8> = color::RGBA::<u8>::new(255, 255, 255);
            let c_u16: color::RGBA<u16> = color::RGBA::<u16>::new(2055, 2055, 2055);
            let c_u32: color::RGBA<u32> = color::RGBA::<u32>::new(255000, 255000, 25500);
            assert_eq!(c_u8.to_string(), "rgba(255, 255, 255, 100)");
            assert_eq!(c_u16.to_string(), "rgba(2055, 2055, 2055, 100)");
            assert_eq!(c_u32.to_string(), "rgba(255000, 255000, 25500, 100)");

        }

        {
        let c_u8: color::RGBA<u8> = color::RGBA::<u8>::new_with_alpha(255, 255, 255, 50);
        let c_u16: color::RGBA<u16> = color::RGBA::<u16>::new_with_alpha(2055, 2055, 2055, 50);
        let c_u32: color::RGBA<u32> = color::RGBA::<u32>::new_with_alpha(255000, 255000, 25500, 50);
        assert_eq!(c_u8.to_string(), "rgba(255, 255, 255, 50)");
        assert_eq!(c_u16.to_string(), "rgba(2055, 2055, 2055, 50)");
        assert_eq!(c_u32.to_string(), "rgba(255000, 255000, 25500, 50)");
        assert_eq!(c_u8.get_r(), 255);
        assert_eq!(c_u8.get_g(), 255);
        assert_eq!(c_u8.get_b(), 255);
        assert_eq!(c_u8.get_a(), 50);
        assert_eq!(c_u16.get_r(), 2055);    
        assert_eq!(c_u16.get_g(), 2055);
        assert_eq!(c_u16.get_b(), 2055);
        assert_eq!(c_u16.get_a(), 50);
        assert_eq!(c_u32.get_r(), 255000);
        assert_eq!(c_u32.get_g(), 255000);
        assert_eq!(c_u32.get_b(), 25500);
        assert_eq!(c_u32.get_a(), 50);
        }
        {
            let mut c_u8: color::RGBA<u8> = color::RGBA::<u8>::new_with_alpha(255, 255, 255, 50);
            let mut c_u16: color::RGBA<u16> = color::RGBA::<u16>::new_with_alpha(2055, 2055, 2055, 50);
            let mut c_u32: color::RGBA<u32> = color::RGBA::<u32>::new_with_alpha(255000, 255000, 25500, 50);
            c_u8.set_r(0);
            c_u8.set_g(0);
            c_u8.set_b(0);
            c_u8.set_a(0);
            c_u16.set_r(0);
            c_u16.set_g(0);
            c_u16.set_b(0);
            c_u16.set_a(0);
            c_u32.set_r(0);
            c_u32.set_g(0);
            c_u32.set_b(0);
            c_u32.set_a(0);
            assert_eq!(c_u8.get_r(), 0);
            assert_eq!(c_u8.get_g(), 0);
            assert_eq!(c_u8.get_b(), 0);
            assert_eq!(c_u8.get_a(), 0);
            assert_eq!(c_u16.get_r(), 0);
            assert_eq!(c_u16.get_g(), 0);
            assert_eq!(c_u16.get_b(), 0);
            assert_eq!(c_u16.get_a(), 0);
            assert_eq!(c_u32.get_r(), 0);
            assert_eq!(c_u32.get_g(), 0);
            assert_eq!(c_u32.get_b(), 0);  
        }

      
    }
}
