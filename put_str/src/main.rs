unsafe fn put_str(str: *const u8) -> Result<isize, String> {
    if str == std::ptr::null() {
        Err("Nullptr was sent to function".to_string())
    } else {
        let mut i: isize = 0;
        while *str.offset(i) != 0 as u8 {
            print!("{}", *str.offset(i) as char);
            i += 1;
        }
        println!();
        Ok(i + 1)
    }
}

fn main() {
    unsafe { put_str("Hello World\0".as_ptr()).unwrap() };
}
