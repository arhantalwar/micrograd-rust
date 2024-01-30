use std::{rc::Rc, cell::RefCell};

use crate::layer::Layer;
use crate::engine::Value;

pub struct MLP {
    layers: Vec<Rc<RefCell<Layer>>>,
}

impl MLP {

    pub fn new(no_of_inputs: i32, no_of_outputs: Vec<i32>) -> Rc<RefCell<MLP>> {

        let mut sz = no_of_outputs.clone();
        sz.insert(0, no_of_inputs);

        let mut layers: Vec<Rc<RefCell<Layer>>> = Vec::new();

        for i in 0..no_of_outputs.len() {
            layers.push(Layer::new(i as i32, (i as i32)+1));
        }

        Rc::new(RefCell::new(MLP {
            layers
        }))

    }

    pub fn eval(mlp: &Rc<RefCell<MLP>>, inputs: &Vec<f64>) -> Vec<Vec<Rc<RefCell<Value>>>> {

        let mut x: Vec<Vec<Rc<RefCell<Value>>>> = Vec::new();

        for layer in &mlp.borrow().layers {
            x.push(Layer::eval(layer, inputs));
        }

        x

    }
    
}
