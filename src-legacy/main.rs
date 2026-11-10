
pub mod mat;
pub mod ml;

use crate::{mat::Matrix, ml::*};

fn main() {
	let mut test: NeuralNetwork = NeuralNetwork { training_data: vec![], train: 10, layers: 4, ninput: vec![4, 5, 6, 2], weight: vec![], bias: vec![] };
	test.init();
}
