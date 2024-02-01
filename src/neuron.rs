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

        let mut random_number_gen = rand::thread_rng();

        let neuron_weights = &neuron.borrow().weights;
        let neuron_bias = neuron.borrow().bias.borrow().data;

        let activation_sum_accumaltor = Value::new(0.0, format!("Act_{:?}", random_number_gen.gen_range(101.00..=200.00)));

        for (weight, input) in neuron_weights.iter().zip(inputs) {

            let input_value = Value::new(*input, format!("Input_{:?}", random_number_gen.gen_range(0.00..=100.00)));

            let temp_for_mul = Value::mul(
                weight.clone(), 
                input_value,
                format!("Input_{:?}", random_number_gen.gen_range(0.00..=100.00)));

            activation_sum_accumaltor.borrow_mut().data += temp_for_mul.borrow().data;
            activation_sum_accumaltor.borrow_mut().children.push(temp_for_mul);

        }

        activation_sum_accumaltor.borrow_mut().data += neuron_bias;

        let out = Value::tanh(activation_sum_accumaltor, format!("Tan_{:?}", random_number_gen.gen_range(201.00..=300.00)));

        out

    }

}
