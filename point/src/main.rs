mod point;

fn set_point(point: &mut point::TPoint) {
    point.x = 42;
    point.y = 21;
}

fn main() {
    let mut point = point::TPoint { x: 0, y: 0 };

    set_point(&mut point);
    assert_eq!(point.x, 42);
    assert_eq!(point.y, 21);
}
