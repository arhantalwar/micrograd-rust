use neuron::Neuron;

pub mod engine;
pub mod neuron;
pub mod layer;
pub mod mlp;

fn main() {

    let a = Neuron::new(2);
    let b = Neuron::forward_pass(&a, &vec![1.0, 2.0]);

    println!("--------------------- weights --------------------");
    println!("{:?}", a.borrow().weights);
    println!("--------------------- bias --------------------");
    println!("{:?}", a.borrow().bias);
    println!("--------------------- b --------------------");
    println!("{:?}", b.borrow().data);

}









