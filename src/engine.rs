use std::ops::Add;
use std::ops::Mul;

trait Backpropagation {
    fn backward(&self);
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Value {
    pub data: f64,
    pub grad: f64,
    pub prev: Vec<Value>,
}

impl Backpropagation for Value {
    fn backward(&self) {
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = Value::new(self.data + rhs.data, vec![self, rhs]);
        out.backward_pass_add();
        return out;
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Value::new(self.data * rhs.data, vec![self, rhs]);
        out.backward_pass_mul();
        return out;

    }
}

impl Value {

    pub fn new(data: f64, children: Vec<Value>) -> Value {
        Value { data, grad: 0.0, prev: children }
    }

    pub fn backward_pass_add(&mut self) {
        self.prev[0].data += self.grad;
        self.prev[1].data += self.grad;
    }

    pub fn backward_pass_mul(&mut self) {
        self.prev[0].data += self.prev[1].data * self.grad;
        self.prev[1].data += self.prev[0].data * self.grad;
    }

    pub fn backward_pass_sigmoid(&mut self) {
        self.prev[0].grad +=  ((1.0 / (1.0 + (-self.data).exp())) * (1.0 - (1.0 / (1.0 + (-self.data).exp())))) * self.grad;
    }

    pub fn sigmoid(self) -> Value {
        let mut out = Value::new(1.0 / (1.0 + (-self.data).exp()), vec![self, ]);
        out.backward_pass_sigmoid();
        return out;
    }

}
