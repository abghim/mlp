use crate::mat::*;
use rand_distr::{Distribution, Normal, StandardNormal};

pub trait NNFuncs {
	fn activation(x: &Matrix) -> Matrix;
	fn loss(prediction: &Matrix, actual: &Matrix) -> f64;
	fn loss_grad_over_z_last(prediction: &Matrix, actual: &Matrix) -> f64; /* ∂L/∂Z^layers := dZ^L, essentially the dZ^L where L is the last layer */
}

pub struct NeuralNetwork {
	pub training_data: Vec<(Matrix, Matrix)>,
	pub train: usize,
	pub layers: usize,
	pub ninput: Vec<usize>,

	pub weight: Vec<Matrix>, /* weight[l] => weights from layer l to layer l+1 */
	pub bias: Vec<Matrix>
} impl NeuralNetwork {

	pub fn init(&mut self) {
		/* init empty matrices, with right dimensions */
		self.weight = vec![Matrix::new(1, 1); self.layers-1];
		self.bias = vec![Matrix::new(1, 1); self.layers-1];

		for l in 0..(self.layers-1) {
			self.weight[l] = Matrix::new(self.ninput[l], self.ninput[l+1]);
			self.bias[l] = Matrix::new(self.ninput[l+1], 1);
		}

		/* init to random weights */
		let z = StandardNormal;
		for l in 0..(self.layers-1) {

			let gauss = Normal::new(0.0, 1.0/(self.ninput[l] as f64)).unwrap();
			for i in 0..self.weight[l].data.len() {
				self.weight[l].data[i] = gauss.sample(&mut rand::rng());

			} println!("W({}): {}", l, self.weight[l]);

			for i in 0..self.bias[l].data.len() {
				self.bias[l].data[i] = z.sample(&mut rand::rng());
			} println!("W0({}): {}", l, self.bias[l]);
		}

		println!("Initialized weights and biases.");

	}

	pub fn train() {

	}

}
