pub mod point {

	use std::ops::{Add, Sub};
	use vector::vector::Vector;

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
}