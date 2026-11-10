use std::intrinsics::powf32;

use crate::trainset::Trainset;
use ndarray::{Array, Array1, Array2, s};

pub struct RegTrainset {
    x: Array2<f32>,
    y: Array1<f32>
}

fn squared(x: f32) -> f32 {x*x}

impl RegTrainset {
    fn from_vec(x: Vec<Vec<f32>>, y: Vec<f32>) -> Self {
           Self {
            x: Array2::from_shape_vec((x.len(), x[0].len()), x.into_iter().flatten().collect()).unwrap(),
            y: Array1::from_vec(y)
        }
    }
}

impl Trainset for RegTrainset {
    type Params = (Array1<f32>, f32);
    fn cost(&self, input: Self::Params) -> f32 {
        (0..self.y.len()).map(|i: usize| squared(self.x.slice(s![i,..]).dot(&input.0)+input.1)-self.y[i]).sum()
    }
    fn dcost(&self, input: Self::Params) -> Self::Params {
        (
            (0..input.0.len()).map(|j: usize| ((0..self.y.len()).map(|i: usize| (self.x.slice(s![i,..]).dot(&input.0)+input.1)-self.y[i]).sum()*input.0[j])), 
            (0..self.y.len()).map(|i: usize| (self.x.slice(s![i,..]).dot(&input.0)+input.1)-self.y[i]).sum()
        )
    }
}
