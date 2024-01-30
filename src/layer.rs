use std::rc::Rc;
use std::cell::RefCell;

use crate::Neuron;
use crate::engine::Value;

#[derive(Debug)]
pub struct Layer(Vec<Rc<RefCell<Neuron>>>);

impl Layer {

    pub fn new(no_of_inputs: i32, no_of_neuron: i32) -> Rc<RefCell<Layer>> {

        let mut neurons: Vec<Rc<RefCell<Neuron>>> = Vec::new();

        for _ in 0..no_of_neuron {
            neurons.push(Neuron::new(no_of_inputs));
        }

        Rc::new(RefCell::new(Layer(neurons)))

    }

    pub fn eval(layer: &Rc<RefCell<Layer>>, inputs: &Vec<f64>) -> Vec<Rc<RefCell<Value>>> {

        let mut out: Vec<Rc<RefCell<Value>>> = Vec::new();

        for n in &layer.borrow().0 {
            let a = Neuron::forward_pass(&n, inputs);
            out.push(a);
        }

        out

    }
    
}
