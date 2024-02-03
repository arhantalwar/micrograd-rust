pub mod engine;
pub mod neuron;
pub mod layer;
pub mod mlp;

use neuron::Neuron;
use crate::engine::backward;

fn main() {

    let n = Neuron::new(2);
    let o = Neuron::forward_pass(&n, &vec![2.0, 0.0]);
    backward(&o);

}









