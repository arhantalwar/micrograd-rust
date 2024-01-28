pub mod engine;

use std::cell::RefCell;
use std::rc::Rc;

use engine::Value;


fn main() {

    let a = Rc::new(RefCell::new(Value::new(10.0, "a".to_string())));
    let b = Rc::new(RefCell::new(Value::new(-3.0, "b".to_string())));
    let d = Rc::new(RefCell::new(Value::new(9.0, "d".to_string())));

    let c = Value::add(Rc::clone(&a), Rc::clone(&b), "c".to_string());

    let f = Value::mul(Rc::clone(&c), Rc::clone(&d), "f".to_string());

    println!("{:#?}", f);

}
