fn swap(x: &mut i32, y: &mut i32) {
    let z = *x;
    *x = *y;
    *y = z;
}

fn main() {
    let mut x = 4;
    let mut y = 2;

    swap(&mut x, &mut y);

    assert_eq!(x, 2);
    assert_eq!(y, 4);
}
