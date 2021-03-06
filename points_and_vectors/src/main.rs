mod point;
mod vector;
mod line;

use point::point::Point;
use vector::vector::Vector;
use line::line::Line;

fn main() {
    unimplemented!();
}

#[test]
fn test_basic() {
    let v1 = Vector::new(1.0, 1.0, 1.0);
    let v2 = Vector::new(2.0, 2.0, 2.0);

    let p1 = Point::new(1.0, 1.0, 1.0);
    let p2 = Point::new(2.0, 2.0, 2.0);

    assert!(v1 == v1);
    assert!(p1 == p1);
    assert!(v1 != v2);
    assert!(p1 != p2);

    assert_eq!(p1 + v1, p2);
    assert_eq!(p2 - p1, v1);
    assert_eq!(v1 + v1, v2);
    assert_eq!(2.0 * v1, v2);
    assert_eq!(v1 * v2, 6.0);
    assert_eq!(v1 ^ v2, Vector::new(0.0, 0.0, 0.0));

    assert_eq!(
       Line::from_pv(Point::new(0.0, 0.0, 0.0), Vector::new(1.0, 1.0, 1.0)).unwrap().distance(p1),
       0_f64
    );
    assert_eq!(
        Line::from_pv(Point::new(0.0, 0.0, 0.0), Vector::new(1.0, 1.0, 1.0)),
        Line::from_pv(Point::new(0.0, 0.0, 0.0), Vector::new(2.0, 2.0, 2.0))
    );
}