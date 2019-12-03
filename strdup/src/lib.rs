#[allow(dead_code)]
unsafe fn strdup(s: *const u8) -> Result<String, String> {
    if s == std::ptr::null() {
        Err("Nullptr was sent to function".to_string())
    } else {
        let mut clone = String::new();
        let mut len: isize = 0;
        while *s.offset(len) != 0 {
            clone.push(*s.offset(len) as char);
            len += 1;
        }
        Ok(clone)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_strdup() {
        unsafe {
            assert!(crate::strdup(std::ptr::null()).is_err());

            assert_eq!(
                crate::strdup("Hello World\0".as_ptr()),
                Ok("Hello World".to_owned())
            );

            assert_eq!(crate::strdup("\0".as_ptr()), Ok("".to_owned()));
        }
    }
}
