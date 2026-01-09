use std::panic;

#[derive(Clone)]
pub struct Matrix {
    pub r: usize,
    pub c: usize,
    pub data: Vec<f64>
}


impl Matrix {
	pub fn new(r: usize, c: usize) -> Matrix {
		Matrix {
			r,
			c,
			data:vec![0.0;r*c]
		}
	}

	pub fn from_vec(r: usize, c: usize, data: Vec<f64>) -> Matrix {
		Matrix {
			r,
			c,
			data
		}
	}

	pub fn get(&self, r: usize, c: usize) -> f64 {
		if r >= self.r || c >= self.c {
			panic!("Row or column index out of bounds");
		}
		self.data[r*self.c+c]
	}

	pub fn set(&mut self, r: usize, c: usize, f: f64) {
		if r >= self.r || c >= self.c {
			panic!("Row or column index out of bounds");
		}
		self.data[r*self.c+c] = f;
	}
}

impl std::ops::Add for Matrix {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		if self.r != rhs.r || self.c != rhs.c {
			panic!("Bad dimensions");
		}
		let mut ret: Vec<f64> = vec![0.0;self.r*self.c];

		for (i, x) in self.data.iter().enumerate() {
			ret[i] = x + rhs.data[i];
		}

		Self::from_vec(self.r, self.c, ret)
	}
}

impl std::ops::Sub for Matrix {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		if self.r != rhs.r || self.c != rhs.c {
			panic!("Bad dimensions");
		}
		let mut ret: Vec<f64> = vec![0.0;self.r*self.c];

		for (i, x) in self.data.iter().enumerate() {
			ret[i] = x - rhs.data[i];
		}

		Self::from_vec(self.r, self.c, ret)
	}
}



impl std::ops::Mul<Matrix> for Matrix {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {
		if self.c != rhs.r {
			panic!("Bad dimensions");
		}
		let mut ret: Matrix = Matrix::new(self.r, rhs.c);

		for i in 0..ret.data.len() {
			let c = i%rhs.c;
			let r = i/rhs.c;

			let mut nt: f64 = 0.0;
			for k in 0..self.c {
				nt += self.get(r, k)*rhs.get(k, c);
			}
			ret.set(r, c, nt);
		} ret
	}
}

impl std::ops::Mul<f64> for Matrix {
	type Output = Self;
	fn mul(self, rhs: f64) -> Self::Output {
		let mut ret = self.clone();
		for i in 0..ret.data.len() {
			ret.data[i] *= rhs;
		} ret
	}
}

impl std::ops::Mul<Matrix> for f64 {
	type Output = Matrix;
	fn mul(self, rhs: Matrix) -> Self::Output {
		let mut ret = rhs.clone();
		for i in 0..ret.data.len() {
			ret.data[i] *= self;
		} ret
	}
}

impl std::ops::Neg for Matrix {
	type Output = Self;
	fn neg(self) -> Self::Output {
		self.clone() * (-1.0)
	}
}


#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn new_initializes_zeros() {
		let m = Matrix::new(2, 3);
		for r in 0..2 {
			for c in 0..3 {
				assert_eq!(m.get(r, c), 0.0);
			}
		}
	}

	#[test]
	fn set_get_roundtrip() {
		let mut m = Matrix::new(2, 2);
		m.set(0, 1, 2.5);
		m.set(1, 0, -3.0);
		assert_eq!(m.get(0, 1), 2.5);
		assert_eq!(m.get(1, 0), -3.0);
	}

	#[test]
	#[should_panic(expected = "Row or column index out of bounds")]
	fn get_out_of_bounds_panics() {
		let m = Matrix::new(1, 1);
		let _ = m.get(1, 0);
	}

	#[test]
	#[should_panic(expected = "Row or column index out of bounds")]
	fn set_out_of_bounds_panics() {
		let mut m = Matrix::new(1, 1);
		m.set(0, 1, 1.0);
	}

	#[test]
	fn add_matrices() {
		let a = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
		let b = Matrix::from_vec(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
		let c = a + b;
		assert_eq!(c.get(0, 0), 6.0);
		assert_eq!(c.get(0, 1), 8.0);
		assert_eq!(c.get(1, 0), 10.0);
		assert_eq!(c.get(1, 1), 12.0);
	}

	#[test]
	#[should_panic(expected = "Bad dimensions")]
	fn add_bad_dimensions_panics() {
		let a = Matrix::new(1, 2);
		let b = Matrix::new(2, 1);
		let _ = a + b;
	}

	#[test]
	fn sub_matrices() {
		let a = Matrix::from_vec(2, 2, vec![9.0, 8.0, 7.0, 6.0]);
		let b = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
		let c = a - b;
		assert_eq!(c.get(0, 0), 8.0);
		assert_eq!(c.get(0, 1), 6.0);
		assert_eq!(c.get(1, 0), 4.0);
		assert_eq!(c.get(1, 1), 2.0);
	}

	#[test]
	#[should_panic(expected = "Bad dimensions")]
	fn sub_bad_dimensions_panics() {
		let a = Matrix::new(2, 3);
		let b = Matrix::new(3, 2);
		let _ = a - b;
	}

	#[test]
	fn mul_matrix_matrix() {
		let a = Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
		let b = Matrix::from_vec(3, 2, vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
		let c = a * b;
		assert_eq!(c.get(0, 0), 58.0);
		assert_eq!(c.get(0, 1), 64.0);
		assert_eq!(c.get(1, 0), 139.0);
		assert_eq!(c.get(1, 1), 154.0);
	}

	#[test]
	#[should_panic(expected = "Bad dimensions")]
	fn mul_bad_dimensions_panics() {
		let a = Matrix::new(2, 2);
		let b = Matrix::new(3, 3);
		let _ = a * b;
	}

	#[test]
	fn mul_matrix_scalar() {
		let a = Matrix::from_vec(2, 2, vec![1.0, -2.0, 3.0, -4.0]);
		let c = a.clone() * 2.0;
		let d = 2.0 * a;
		assert_eq!(c.get(0, 0), 2.0);
		assert_eq!(d.get(0, 0), 2.0);

		assert_eq!(c.get(0, 1), -4.0);
		assert_eq!(d.get(0, 1), -4.0);

		assert_eq!(c.get(1, 0), 6.0);
		assert_eq!(c.get(1, 1), -8.0);
	}

	#[test]
	fn mul_scalar_matrix() {
		let a = Matrix::from_vec(2, 2, vec![1.5, 2.0, -3.0, 4.0]);
		let c = 3.0 * a;
		assert_eq!(c.get(0, 0), 4.5);
		assert_eq!(c.get(0, 1), 6.0);
		assert_eq!(c.get(1, 0), -9.0);
		assert_eq!(c.get(1, 1), 12.0);
	}

	#[test]
	fn neg_matrix() {
		let a = Matrix::from_vec(2, 2, vec![1.0, -2.0, 3.5, -4.5]);
		let c = -a;
		assert_eq!(c.get(0, 0), -1.0);
		assert_eq!(c.get(0, 1), 2.0);
		assert_eq!(c.get(1, 0), -3.5);
		assert_eq!(c.get(1, 1), 4.5);
	}

}
