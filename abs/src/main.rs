macro_rules! ABS {
    ($x:expr) => {
        if $x < 0 {
            -$x
        } else {
            $x
        }
    };
}

fn main() {
    assert_eq!(ABS!(-1), 1);
    assert_eq!(ABS!(1), 1);
    assert_eq!(ABS!(0), 0);
}
