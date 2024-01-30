use std::{rc::Rc, cell::RefCell};

use crate::layer::Layer;
use crate::engine::Value;

#[derive(Debug)]
pub struct MLP {
    pub layers: Vec<Rc<RefCell<Layer>>>,
}

impl MLP {

    pub fn new(no_of_inputs: i32, no_of_outputs: Vec<i32>) -> Rc<RefCell<MLP>> {

        let mut sz = no_of_outputs.clone();
        sz.insert(0, no_of_inputs);

        let mut layers: Vec<Rc<RefCell<Layer>>> = Vec::new();

        for i in 0..no_of_outputs.len() {
            layers.push(Layer::new(*sz.get(i).unwrap(), *sz.get(i + 1).unwrap()));
        }

        Rc::new(RefCell::new(MLP {
            layers
        }))

    }

    pub fn eval(mlp: &Rc<RefCell<MLP>>, inputs: &Vec<f64>) -> Vec<Rc<RefCell<Value>>> {

        let mut x: Vec<Rc<RefCell<Value>>> = Vec::new();
        let mut x_inputs: Vec<f64> = inputs.clone();

        for layer in &mlp.borrow().layers {

            x = Layer::eval(&layer, &x_inputs);

            x_inputs.clear();

            for i in &x {
                x_inputs.push(i.borrow().data);
            }

        }

        x

    }
    
}
