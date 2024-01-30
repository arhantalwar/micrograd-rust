pub mod engine;
pub mod neuron;
pub mod layer;
pub mod mlp;
use mlp::MLP;
use neuron::Neuron;

fn main() {

    let x = vec![2.0, 3.0, -1.0];
    let n = MLP::new(3, vec![4, 4, 1]);
    let f = MLP::eval(&n, &x);

    for i in f {
        println!("{:?}", i)
    }
}


