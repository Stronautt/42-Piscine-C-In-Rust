fn range(min: i32, max: i32) -> Result<std::ops::Range<i32>, String> {
    if min == max {
        Err("Minimal and maximum values are equal".to_owned())
    } else if min > max {
        Err("Minimal value is greater than maximum".to_owned())
    } else {
        Ok(min..max)
    }
}

fn main() {
    assert!(range(10, 5).is_err());
    assert!(range(5, 5).is_err());
    for i in range(5, 10).unwrap() {
        println!("{:?}", i);
    }
}
