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

        for _ in 0..no_of_inputs {
            let w = Value::new(rng.gen_range(-1.0..=1.0), format!("w{:?}", rng.gen_range(0.00..100.00)));
            weights.push(w);
        }

        let bias = Value::new(rng.gen_range(-1.0..=1.0), format!("b{:?}", rng.gen_range(-1.0..=1.0)));

        Rc::new(RefCell::new(Neuron {
            weights,
            bias
        }))

    }

    pub fn forward_pass(neuron: &Rc<RefCell<Neuron>>, inputs: &Vec<f64>) -> Rc<RefCell<Value>> {

        let mut random_number_gen = rand::thread_rng();
        let mut list_of_weight_input: Vec<Rc<RefCell<Value>>> = vec![];

        let neuron_weights = &neuron.borrow().weights;
        let neuron_bias = &neuron.borrow().bias;

        for (weight, input) in neuron_weights.iter().zip(inputs) {

            let input_val = Value::new(*input,
                                       format!("Input_Value_{:?}", random_number_gen.gen_range(0.0..=50.0)));

            let mul_val = Value::mul(weight.clone(),
                                     input_val,
                                     format!("Mul_Value_{:?}", random_number_gen.gen_range(50.0..=100.0)));

            list_of_weight_input.push(mul_val);

        }

        let new_list = list_sum(list_of_weight_input);
        let list_sum = new_list.first().unwrap();

        let adding_bias = Value::add(
            neuron_bias.clone(),
            list_sum.clone(),
            format!("BIAS_{:?}", random_number_gen.gen_range(151.0..=200.0)));

        let out = Value::tanh(adding_bias,
                              format!("TANH_{:?}", random_number_gen.gen_range(721.0..=821.0)));

        out

    }

}

fn list_sum(list: Vec<Rc<RefCell<Value>>>) -> Vec<Rc<RefCell<Value>>> {

    let mut random_number_gen = rand::thread_rng();

    if list.len() == 1 {

        return list;

    } else if list.len() % 2 == 0 {

        let mut new_list: Vec<Rc<RefCell<Value>>> = vec![];

        for i in list.chunks(2) {

            let add_val = Value::add(
                i.get(0).unwrap().clone(),
                i.get(1).unwrap().clone(),
                format!("ADD_VAL_{:?}", random_number_gen.gen_range(50.0..=100.0)));

            new_list.push(add_val);

        }

        list_sum(new_list);

    } else if list.len() % 2 != 0 {

        let mut new_list: Vec<Rc<RefCell<Value>>> = vec![];

        let mut list_copy = list.clone();
        let extra_val = list_copy.pop().unwrap();

        for i in list_copy.chunks(2) {

            let add_val = Value::add(
                i.get(0).unwrap().clone(),
                i.get(1).unwrap().clone(),
                format!("ADD_VAL_{:?}", random_number_gen.gen_range(50.0..=100.0)));

            new_list.push(add_val);

        }

        new_list.push(extra_val);

        list_sum(new_list);

    }

    list

}








