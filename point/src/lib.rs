mod point;

#[allow(dead_code)]
fn set_point(point: &mut point::TPoint) {
    point.x = 42;
    point.y = 21;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_set_point() {
        let mut point = crate::point::TPoint { x: 0, y: 0 };

        crate::set_point(&mut point);
        assert_eq!(point.x, 42);
        assert_eq!(point.y, 21);
    }
}
