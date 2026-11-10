use std::{fmt::Display, ops::Add};

use ndarray::{self, Array1, Array2};
use ndarray_rand::{rand_distr::Normal, RandomExt};

pub type Num = f64;

pub struct Dataset<D,L> {
    data: ndarray::Array1<D>,
    labels: ndarray::Array1<L>
}

pub trait Layer {
    fn forward(&self, input: ndarray::Array1::<Num>) -> Array1<Num>;
    fn n_mut(&mut self) -> &mut usize;
    fn units(&self) -> &usize;
    fn init(&mut self, n: usize);
}

pub trait Model<D, L> {
    fn fit(&mut self, train: Dataset<D, L>);
    fn predict(&self, input: D) -> Array1<Num>;
}

type DatasetCls = Dataset<ndarray::Array1<Num>, usize>;

pub struct Dense {
    units: usize,
    n: usize,
    w: ndarray::Array2<Num>,
    b: ndarray::Array1<Num>
}

impl Dense {
    pub fn new(u: usize) -> Self {
        Dense {
            units: u,
            n: 0,
            w: Array2::zeros((0,0)),
            b: Array1::zeros(0)
        }
    }
    fn sigmoid(input: &ndarray::Array1<Num>) -> ndarray::Array1<Num> {
        input.mapv(|x| 1.0/(1.0+(-x).exp()))
    }
}

impl Layer for Dense {
    fn forward(&self, input: ndarray::Array1::<Num>) -> Array1<Num> {
        // println!("input {:?} / weight {:?}", &input.shape(), &self.w.shape());
        Self::sigmoid(&self.w.dot(&input).add(&self.b))
    }

    fn n_mut(&mut self) -> &mut usize {
        &mut self.n
    }

    fn units(&self) -> &usize {
        &self.units
    }

    fn init(&mut self, n: usize) {
        let dist = Normal::new(0.0, 1.0).unwrap();
        self.n = n;
        self.w = Array2::random((self.units, n), dist);
        self.b = Array1::random(self.units, dist);
    }


}

struct Input {
    data: Array1<Num>
}

struct Sequential<L: Layer> {
    layers: Vec<L>,
    num_in: usize
}

impl<L: Layer> Sequential<L> {
    fn new() -> Self {
        Self {
            layers: vec![],
            num_in: 0
        }
    }

    fn layer(mut self, mut layer: L) -> Self {
        if !self.layers.is_empty() {
            layer.init(*self.layers.last().unwrap().units());
        }
        self.layers.push(layer); self
    }
}

impl<L: Layer> Model<ndarray::Array1<Num>, usize> for Sequential<L> {
    fn fit(&mut self, train: Dataset<Array1<Num>, usize>) {
        self.num_in = train.data.first().unwrap().len();
        self.layers[0].init(self.num_in);
    }

    fn predict(&self, input: ndarray::Array1<Num>) -> Array1<Num> {
        let mut prev = input;
        for layer in &self.layers {
            prev = layer.forward(prev);
        } prev
    }
}

impl Display for Dense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dense: {} in, {} out", self.n, self.units)
    }
}

#[cfg(test)]
mod test {
    use ndarray::array;

use super::*;

    #[test]
    fn test_nn1() {
        let dataset = DatasetCls{
            data: array![array![1.1, 1.0], array![2.2, 17.0], array![3.3, -0.8]],
            labels: array![17, 34, 51]
        };

        let mut model = Sequential::<Dense>::new()
            .layer(Dense::new(25))
            .layer(Dense::new(15))
            .layer(Dense::new(1));

        model.fit(dataset);

        for layer in &model.layers {
            println!("{layer}");
        }

        println!("Prediction: {}", model.predict(array![1.0, 0.0]));
    }
}
