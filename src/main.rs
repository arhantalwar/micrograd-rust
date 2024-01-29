pub mod engine;
pub mod neuron;
pub mod layer;
use engine::{Value, backward};
use neuron::Neuron;

use std::rc::Rc;

fn main() {

    let x1 = Value::new(2.0, "x1".to_string());
    let x2 = Value::new(0.0, "x2".to_string());

    let w1 = Value::new(-3.0, "w1".to_string());
    let w2 = Value::new(1.0, "w2".to_string());

    let b = Value::new(6.8813735870, "b".to_string());

    let x1w1 = Value::mul(Rc::clone(&x1), Rc::clone(&w1), "x1w1".to_string());
    let x2w2 = Value::mul(Rc::clone(&x2), Rc::clone(&w2), "x2w2".to_string());

    let x1w1x2w2 = Value::add(Rc::clone(&x1w1), Rc::clone(&x2w2), "x1w1x2w2".to_string());

    let n = Value::add(Rc::clone(&x1w1x2w2), Rc::clone(&b), "n".to_string());

    let o = Value::tanh(Rc::clone(&n), "o".to_string());

    backward(&o);

    let a = Neuron::new(2);
    Neuron::forward_pass(&a, vec![1.0, 2.0]);


}


