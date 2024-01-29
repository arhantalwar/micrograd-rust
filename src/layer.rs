use std::rc::Rc;
use std::cell::RefCell;
use crate::Neuron;

pub struct Layer {
    neurons: Vec<Rc<RefCell<Neuron>>>,
}
