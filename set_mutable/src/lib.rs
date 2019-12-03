#[allow(dead_code)]
fn set_mutable(dest: &mut i32, value: i32) -> i32 {
    *dest = value;
    *dest
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_set_mutable() {
        let mut x: i32 = 0;

        assert_eq!(crate::set_mutable(&mut x, 21), 21);
        assert_eq!(x, 21);
    }
}
