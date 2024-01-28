mod engine;
use crate::engine::Value;

#[allow(unused_mut)]
fn main() {

    let mut x1 = Value::new(2.0, String::from("x1"));
    let mut x2 = Value::new(0.0, String::from("x2"));

    let mut w1 = Value::new(-3.0, String::from("w1"));
    let mut w2 = Value::new(1.0, String::from("w2"));

    let mut b = Value::new(6.8813735870195432, String::from("b"));

    let mut x1w1 = &mut x1.mul(&mut w1);
    let mut x2w2 = &mut x2.mul(&mut w2);

    let mut x1w1x2w2 = &mut x1w1.add(&mut x2w2);

    let mut n = &mut x1w1x2w2.add(&mut b);

    let o = n.tanh();

    println!("{:?}", o.data);

    (x1w1x2w2.backward)();

}
