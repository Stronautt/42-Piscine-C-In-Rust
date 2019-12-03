#[allow(dead_code)]
fn range(min: i32, max: i32) -> Result<std::ops::Range<i32>, String> {
    if min == max {
        Err("Minimal and maximum values are equal".to_owned())
    } else if min > max {
        Err("Minimal value is greater than maximum".to_owned())
    } else {
        Ok(min..max)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_range() {
        assert!(crate::range(10, 5).is_err());
        assert!(crate::range(5, 5).is_err());
        assert_eq!(crate::range(5, 10), Ok(5..10));
    }
}
