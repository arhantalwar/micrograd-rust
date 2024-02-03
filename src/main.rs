pub mod engine;
pub mod neuron;
pub mod layer;
pub mod mlp;

use neuron::Neuron;
use engine::Value;

use crate::engine::backward;

fn main() {

    let x1 = Value::new(2.0, "x1".to_string());
    let x2 = Value::new(0.0, "x2".to_string());
    
    let w1 = Value::new(-3.0, "w1".to_string());
    let w2 = Value::new(1.0, "w2".to_string());
    
    let b = Value::new(6.7, "b".to_string());
    
    let x1w1 = Value::mul(x1, w1, "x1w1".to_string());
    let x2w2 = Value::mul(x2, w2, "x2w2".to_string());
    
    let x1w1x2w2 = Value::add(x1w1, x2w2, "x1w1x2w2".to_string());
    
    let n = Value::add(x1w1x2w2, b, "n".to_string());
    
    let o = Value::tanh(n, "o".to_string());
    
    backward(&o);

    println!("\n");

    let nn = Neuron::new(2);
    
    let oo = Neuron::forward_pass(&nn, &vec![2.0, 0.0]);
    
    backward(&oo);

}









