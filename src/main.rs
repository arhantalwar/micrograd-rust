pub mod engine;
pub mod neuron;
pub mod layer;
use layer::Layer;
use neuron::Neuron;

fn main() {

    let a = Layer::new(1, 1);
    let x = vec![2.0];

    let z = Layer::eval(&a, &x);

    dbg!(z);

}


