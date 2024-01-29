use crate::engine::Value;
use std::rc::Rc;
use std::cell::RefCell;
use rand::Rng;

#[derive(Debug)]
pub struct Neuron {
    pub weights: Vec<Rc<RefCell<Value>>>,
    pub bias: Rc<RefCell<Value>>
}

impl Neuron {

    pub fn new(no_of_inputs: i32) -> Rc<RefCell<Neuron>> {

        let mut weights: Vec<Rc<RefCell<Value>>> = Vec::new();
        let mut rng = rand::thread_rng();

        // Init random weights
        for i in 0..no_of_inputs {
            let w = Value::new(rng.gen_range(-1.0..=1.0), format!("w{}", i));
            weights.push(w);
        }

        // Init random bais
        let bias = Value::new(rng.gen_range(-1.0..=1.0), format!("b{:?}", rng.gen_range(-1.0..=1.0)));

        Rc::new(RefCell::new(Neuron {
            weights,
            bias
        }))

    }

    pub fn forward_pass(neuron: &Rc<RefCell<Neuron>>, inputs: &Vec<f64>) -> Rc<RefCell<Value>> {

        let neuron_weights = &neuron.borrow().weights;
        let mut act: f64 = 0.0;

        for (i, j) in neuron_weights.iter().zip(inputs.iter()) {
            act += i.borrow().data * j;
        }

        act += neuron.borrow().bias.borrow().data;
        let out = act.tanh();

        Value::new(out, format!("out{:?}", out))
    
    }



}
