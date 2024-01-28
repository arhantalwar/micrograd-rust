pub mod engine;
use engine::Value;

use std::cell::RefCell;
use std::rc::Rc;

#[allow(unused_variables)]
fn main() {

    let x1 = Rc::new(RefCell::new(Value::new(2.0, "x1".to_string())));
    let x2 = Rc::new(RefCell::new(Value::new(0.0, "x2".to_string())));

    let w1 = Rc::new(RefCell::new(Value::new(-3.0, "w1".to_string())));
    let w2 = Rc::new(RefCell::new(Value::new(1.0, "ww".to_string())));

    let b = Rc::new(RefCell::new(Value::new(6.8813735870, "b".to_string())));

    let x1w1 = Value::mul(Rc::clone(&x1), Rc::clone(&w1), "x1w1".to_string());
    let x2w2 = Value::mul(Rc::clone(&x2), Rc::clone(&w2), "x2w2".to_string());

    let x1w1x2w2 = Value::add(Rc::clone(&x1w1), Rc::clone(&x2w2), "x1w1x2w2".to_string());

    let n = Value::add(Rc::clone(&x1w1x2w2), Rc::clone(&b), "n".to_string());

    let o = Value::tanh(Rc::clone(&n), "o".to_string());

    o.borrow_mut().grad = 1.0;

    if let Some(backward) = o.borrow().backward.as_ref() {
        backward(Rc::clone(&o));
    };

    if let Some(backward) = n.borrow().backward.as_ref() {
        backward(Rc::clone(&n));
    };

    if let Some(backward) = x1w1x2w2.borrow().backward.as_ref() {
        backward(Rc::clone(&x1w1x2w2));
    };

    if let Some(backward) = x2w2.borrow().backward.as_ref() {
        backward(Rc::clone(&x2w2));
    };

    if let Some(backward) = x1w1.borrow().backward.as_ref() {
        backward(Rc::clone(&x1w1));
    };

    println!("{:?}", o.borrow().grad);
    println!("{:?}", n.borrow().grad);
    println!("{:?}", b.borrow().grad);
    println!("{:?}", x1w1x2w2.borrow().grad);
    println!("{:?}", x1w1.borrow().grad);
    println!("{:?}", x2w2.borrow().grad);
    println!("{:?}", w2.borrow().grad);
    println!("{:?}", x2.borrow().grad);
    println!("{:?}", w1.borrow().grad);
    println!("{:?}", x1.borrow().grad);

}
