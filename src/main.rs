pub mod engine;
pub mod neuron;
pub mod layer;
pub mod mlp;

use mlp::MLP;
use crate::neuron::Neuron;

fn main() {

    let x = vec![2.0, 3.0, -1.0];
    let n = MLP::new(3, vec![4, 4, 1]);
    let z = MLP::eval(&n, &x);

    dbg!(z);

}


