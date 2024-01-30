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

        let mut rng = rand::thread_rng();

        let neuron_weights = &neuron.borrow().weights;
        let neuron_bias = neuron.borrow().bias.borrow().data;

        let act_val = Value::new(0.0, format!("act_val{:?}", rng.gen_range(0.0..1000.0)));

        for (i, j) in neuron_weights.iter().zip(inputs.iter()) {

            let j_val = Value::new(j.clone(), format!("j_val{:?}", rng.gen_range(0.0..1000.0)));
            let act_mul = Value::mul(i.clone(), j_val, format!("act{:?}", rng.gen_range(0.0..=1000.0)));

            let temp = Value::add(act_val.clone(), act_mul, format!("tmp{:?}", rng.gen_range(0.0..=1000.0)));
            act_val.borrow_mut().data += temp.borrow().data;

        }

        act_val.borrow_mut().data += neuron_bias;
        let out = Value::tanh(act_val, format!("out{:?}", rng.gen_range(0.0..1000.0)));

        out
    
    }



}
