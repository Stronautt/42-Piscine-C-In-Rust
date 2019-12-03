#[allow(dead_code)]
fn swap(x: &mut i32, y: &mut i32) {
    let z = *x;
    *x = *y;
    *y = z;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_swap() {
        let mut x = 4;
        let mut y = 2;

        crate::swap(&mut x, &mut y);

        assert_eq!(x, 2);
        assert_eq!(y, 4);
    }
}
