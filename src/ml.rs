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

	pub weight: Vec<Matrix>,
	pub bias: Vec<Matrix>
} impl NeuralNetwork {
	fn init(&mut self) {
		/* init to random weights */
		let z = StandardNormal;
		for l in 0..self.layers {
			let gauss = Normal::new(0.0, 1.0/(self.ninput[l] as f64)).unwrap();
			for i in 0..self.weight[l].data.len() {
				self.weight[l].data[i] = gauss.sample(&mut rand::rng());
			}
			for i in 0..self.bias[l].data.len() {
				self.bias[l].data[i] = z.sample(&mut rand::rng())
			}
		}

		println!("Initialized weights and biases.");

	}

}
