#[allow(dead_code)]
unsafe fn str_cmp(s1: *const u8, s2: *const u8) -> Result<i8, String> {
    if s1 == std::ptr::null() || s2 == std::ptr::null() {
        Err("Nullptr was sent to function".to_string())
    } else {
        let mut i: isize = 0;
        while *s1.offset(i) != 0 && *s2.offset(i) != 0 {
            if *s1.offset(i) != *s2.offset(i) {
                break;
            }
            i += 1;
        }
        Ok(*s1.offset(i) as i8 - *s2.offset(i) as i8)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_str_cmp() {
        unsafe {
            assert_eq!(crate::str_cmp("\0".as_ptr(), "\0".as_ptr()), Ok(0));
            assert!(crate::str_cmp(std::ptr::null(), "\0".as_ptr()).is_err());
            assert!(crate::str_cmp(std::ptr::null(), std::ptr::null()).is_err());
            assert!(crate::str_cmp("\0".as_ptr(), std::ptr::null()).is_err());
            assert_eq!(
                crate::str_cmp("Hello World\0".as_ptr(), "Hello World\0".as_ptr()),
                Ok(0)
            );
            assert_eq!(
                crate::str_cmp("Hello World\0".as_ptr(), "Hello world\0".as_ptr()),
                Ok(-32)
            );
            assert_eq!(
                crate::str_cmp("Hello World\0".as_ptr(), "Hello\0".as_ptr()),
                Ok(32)
            );
        }
    }
}
