use std::ops::{Add, Sub};
use std::f64::EPSILON as F_EPSILON;

// Clone is so we have the clone() method available for out struct
// Copy is so everytime we attempt to give away ownership we clone() instead
#[derive(Debug, Clone, Copy)]
pub struct Point{
	pub x: f64,
	pub y: f64,
	pub z: f64
}

impl Point {

	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Self{
			x: x,
			y: y,
			z: z
		}
  }
}

impl PartialEq for Point {
  fn eq(&self, rhs: &Self) -> bool {
    ((self.x - rhs.x).abs() +
     (self.y - rhs.y).abs() +
     (self.z - rhs.z).abs())< (3_f64 * F_EPSILON)
  }
}

impl Add<Self> for Point {
	type Output = Self;

  fn add(self, rhs: Self) -> Self {
	  Self {
	  	x: self.x + rhs.x,
	  	y: self.y + rhs.y,
	  	z: self.z + rhs.z
	  }
  }
}

impl Add<Vector> for Point {
	type Output = Self;

  fn add(self, rhs: Vector) -> Self {
	  Self {
	  	x: self.x + rhs.x,
	  	y: self.y + rhs.y,
	  	z: self.z + rhs.z
	  }
  }
}

impl Sub for Point {
	type Output = Vector;

  fn sub(self, rhs: Self) -> Vector {
      Vector::new(self.x - rhs.x,
                  self.y - rhs.y,
                  self.z - rhs.z)
  }
}

use std::ops::{Mul, BitXor};
// Clone is so we have the clone() method available for out struct
// Copy is so everytime we attempt to give away ownership we clone() instead
#[derive(Debug, Clone, Copy)]
pub struct Vector{
	pub x: f64,
	pub y: f64,
	pub z: f64
}

impl Vector {

	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Self{
			x: x,
			y: y,
			z: z
		}
	}

	pub fn is_zero(&self)-> bool {
		(self.x == 0_f64) && (self.y == 0_f64) && (self.z == 0_f64)
	}

	pub fn norm(&self)-> f64 {
		((self.x*self.x) +(self.y*self.y) +(self.z*self.z)).sqrt()
	}
}

impl PartialEq for Vector {
  fn eq(&self, rhs: &Self) -> bool {
    ((self.x - rhs.x).abs() + 
     (self.y - rhs.y).abs() + 
     (self.z - rhs.z).abs())< (3_f64 * F_EPSILON)
  }
}

impl Add for Vector {
	type Output = Self;

  fn add(self, rhs: Self) -> Self {
    Self {
    	x: self.x + rhs.x,
    	y: self.y + rhs.y, 
    	z: self.z + rhs.z
    }
  }
}

impl Sub for Vector {
	type Output = Self;

  fn sub(self, rhs: Self) -> Self {
    Self {
    	x: self.x - rhs.x,
    	y: self.y - rhs.y,
    	z: self.z - rhs.z
    }
  }
}

impl Mul<Vector> for f64{
	type Output = Vector;

	fn mul(self, rhs: Vector) -> Vector {
		Vector{
			x: self * rhs.x,
			y: self * rhs.y,
			z: self * rhs.z
		}
	}
}

impl Mul<Vector> for Vector{
	type Output = f64;

	fn mul(self, rhs: Self) -> f64 {
		(self.x*rhs.x) + (self.y*rhs.y) + (self.z*rhs.z)
	}	
}

impl BitXor for Vector {
	type Output = Self;

	fn bitxor(self, rhs: Self) -> Self{
		Self {
			x: 	((self.y * rhs.z) - (self.z*rhs.y)),
			y:	((self.z*rhs.x) - (self.x*rhs.z)),
			z: 	((self.x*rhs.y) 	- (self.y*rhs.x))
		}
	}
}

#[derive(Debug)]
pub struct Line {
	start_point: Point,
	direction_vector: Vector
}

impl Line{

	pub fn from_pv(start: Point, direction_vector: Vector) -> Option<Self>{
		if direction_vector.is_zero() {
			None 
		} 
		else {
			Some(
						Self {
							start_point: start,
							direction_vector: direction_vector
						}
					)
		}			
	}

	pub fn from_pp(start: Point, end: Point) -> Option<Self>{
		let direction_vector: Vector = end - start;
		
		Line::from_pv(start, direction_vector)
	}

	pub fn distance(&self, point: Point) -> f64 {
		// this will be the vector from the start point of the line
		// to the given point. 
		// (FromStartToPoint)
		let vec_fstp: Vector = point - self.start_point;
		let cross_prod: Vector = vec_fstp ^ self.direction_vector;
		cross_prod.norm() / self.direction_vector.norm()
	}
}

impl PartialEq for Line {
	fn eq(&self, other: &Self) -> bool {
		(other.distance(self.start_point) == 0_f64) && (other.distance(self.start_point + self.direction_vector) == 0_f64)
	}
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