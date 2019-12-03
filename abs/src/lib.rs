#[allow(unused_macros)]
macro_rules! ABS {
    ($x:expr) => {
        if $x < 0 {
            -$x
        } else {
            $x
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_abs_negative() {
        assert_eq!(ABS!(-1), 1);
    }

    #[test]
    fn test_abs_positive() {
        assert_eq!(ABS!(1), 1);
    }

    #[test]
    fn test_abs_zero() {
        assert_eq!(ABS!(0), 0);
    }
}
