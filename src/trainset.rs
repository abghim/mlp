pub trait Trainset {
    type Params;
    fn cost(&self, input: Self::Params) -> f32;
    fn dcost(&self, input: Self::Params) -> Self::Params;
}
