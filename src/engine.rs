use std::ops::Add;
use std::ops::Mul;

trait Backpropagation {
    fn backward(self);
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Value {
    data: f64,
    grad: f64,
    prev: Vec<Value>,
}

impl Backpropagation for Value {
    fn backward(self) {
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        let out = Value::new(self.data + rhs.data, vec![self, rhs]);
        out.add_backward_pass();
        return out;
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        let out = Value::new(self.data * rhs.data, vec![self, rhs]);
        out.mul_backward_pass();
        return out;

    }
}

impl Value {

    pub fn new(data: f64, children: Vec<Value>) -> Value {
        Value { data, grad: 0.0, prev: children }
    }

    pub fn add_backward_pass(&self) {
        println!("Adding -> {:?}", self);
    }

    pub fn mul_backward_pass(&self) {
        println!("Multiplying -> {:?}", self);

    }

}
