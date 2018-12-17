pub mod vector{

	use std::ops::{Add, Sub, Mul, BitXor};
	use std::f64::EPSILON as F_EPSILON;

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
				y:	((self.z*rhs.x) 	- (self.x*rhs.z)),
				z: 	((self.x*rhs.y) 	- (self.y*rhs.x))
			}
		}
	}
}