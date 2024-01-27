use std::ops::Add;
use std::ops::Mul;

#[derive(Debug)]
struct Value {
    data: f64,
    grad: f64,
    prev: Vec<Value>
}

impl Value {
    fn new(data: f64, children: Vec<Value>) -> Value {
        Value { data, grad: 0.0, prev: children }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        Value::new(self.data + rhs.data, vec![self, rhs])
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        Value::new(self.data * rhs.data, vec![self, rhs])
    }
}

fn main() {

    let a = Value::new(1.0, vec![]);
    let b = Value::new(2.0, vec![]);
    let c = a + b;
    let d = Value::new(12.0, vec![]);
    let e = c * d;
    
    println!("{:#?}", e);

}
