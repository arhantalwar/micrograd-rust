pub mod engine;
pub mod neuron;
pub mod layer;
pub mod mlp;

use mlp::MLP;
use neuron::Neuron;
use crate::engine::backward;

fn main() {

    let a = MLP::new(2, vec![4, 4, 1]);
    let o = MLP::eval(&a, &vec![1.0, 2.0]);

    for i in &o {
        backward(i);
    }

    println!("{:#?}", o);


}









