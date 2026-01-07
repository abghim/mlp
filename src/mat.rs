#[derive(Clone)]
pub struct Matrix {
    r: usize,
    c: usize,
    data: Vec<f64>
}


impl Matrix {
	fn new(r: usize, c: usize) -> Matrix {
		Matrix {
			r,
			c,
			data:vec![0.0;r*c]
		}
	}

	fn from_vec(r: usize, c: usize, data: Vec<f64>) -> Matrix {
		Matrix {
			r,
			c,
			data
		}
	}

	fn get(&self, r: usize, c: usize) -> f64 {
		self.data[r*self.c+c]
	}

	fn set(&mut self, r: usize, c: usize, f: f64) {
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

impl std::ops::Mul for Matrix {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {
		if self.c != rhs.r {
			panic!("Bad dimensions");
		}
		let mut ret: Vec<f64> = vec![0.0;self.r*self.c];

		for (i, x) in self.data.iter().enumerate() {
			ret[i] = x - rhs.data[i];
		}

		Self::from_vec(self.r, rhs.c, ret)
	}
}



#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn matrix1() {
		let m: Matrix = Matrix::new(3, 4);

	}

}
