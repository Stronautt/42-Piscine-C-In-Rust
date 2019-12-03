fn set_mutable(dest: &mut i32, value: i32) -> i32 {
    *dest = value;
    *dest
}

fn main() {
    let mut x: i32 = 0;

    assert_eq!(set_mutable(&mut x, 21), 21);
    assert_eq!(x, 21);
}
