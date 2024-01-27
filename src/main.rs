mod engine;
use crate::engine::Value;

fn main() {

    let a = Value::new(1.0, vec![]);
    let b = Value::new(2.0, vec![]);
    let c = a + b;
    let d = Value::new(12.0, vec![]);
    let e = c * d;
    
    println!("{:#?}", e);

}
