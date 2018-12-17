pub mod line{

	use point::point::Point;
	use vector::vector::Vector;

	#[derive(Debug)]	
	pub struct Line {
		start_point: 			Point,
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
								start_point: 			start,
								direction_vector: direction_vector
							}
						)
			}			
		}

		pub fn from_pp(start: Point, end: Point) -> Option<Self>{
			let direction_vector :Vector = end - start;
			
			Line::from_pv(start, direction_vector)
		}

		pub fn distance(&self, point: Point) -> f64 {
			// this will be the vector from the start point of the line
			// to the given point. 
			// (FromStartToPoint)
			let vec_fstp 	 :Vector = point 		- self.start_point;
			let cross_prod :Vector = vec_fstp ^ self.direction_vector;
			cross_prod.norm() / self.direction_vector.norm()
		}
	}

	impl PartialEq for Line {
		fn eq(&self, other: &Self) -> bool {
			(other.distance(self.start_point) == 0_f64) && (other.distance(self.start_point + self.direction_vector) == 0_f64)
		}
	}
}