pub mod engine;
pub mod neuron;
pub mod layer;
pub mod mlp;

use mlp::MLP;
use neuron::Neuron;

use crate::engine::backward;

fn main() {

    let n = MLP::new(3, vec![4, 4, 1]);
    let xs = vec![
        vec![2.0, 3.0, -1.0],
        vec![3.0, -1.0, 0.5],
        vec![0.5, 1.0,  1.0],
        vec![1.0, 1.0, -1.0],
    ];

    // let ys = vec![1.0, -1.0, -1.0, 1.0];
    let ypred = xs.iter().map(|x| MLP::eval(&n, x)).collect::<Vec<_>>();

    ypred.iter().for_each(|x| x.iter().for_each(|y| backward(y)));

    println!("{:#?}", ypred);

}










