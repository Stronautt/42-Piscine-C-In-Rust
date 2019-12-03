#[allow(dead_code)]
unsafe fn str_len(str: *const u8) -> Result<usize, String> {
    if str == std::ptr::null() {
        Err("Nullptr was sent to function".to_string())
    } else {
        let mut i: isize = 0;
        while *str.offset(i) != 0 {
            i += 1;
        }
        Ok(i as usize)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_str_len() {
        unsafe {
            assert_eq!(crate::str_len("\0".as_ptr()), Ok(0));
            assert!(crate::str_len(std::ptr::null()).is_err());
            assert_eq!(crate::str_len("Hello World\0".as_ptr()), Ok(11));
        }
    }
}
