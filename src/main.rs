pub mod engine;
pub mod neuron;
pub mod layer;
pub mod mlp;

use layer::Layer;
use crate::neuron::Neuron;

fn main() {

    let a = Layer::new(2, 1);
    let b = Layer::eval(&a, vec![2.0, 0.0].as_ref());

    println!("{:#?}", a);
    println!("{:#?}", b);

}










