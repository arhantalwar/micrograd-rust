use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Value {
    pub data: f64,
    pub grad: f64,
    pub children: Vec<Rc<RefCell<Value>>>,
    pub ops: char,
    pub label: String
}

impl Value {

    pub fn new(data: f64, label: String) -> Value {
        Value { data, grad: 0.0, children: vec![], ops: 'N', label }
    }

    pub fn add(lhs: Rc<RefCell<Value>>, rhs: Rc<RefCell<Value>>, label: String) -> Rc<RefCell<Value>> {

        let lhs_data = lhs.borrow().data;
        let rhs_data = rhs.borrow().data;
        let sum = lhs_data + rhs_data;

        let out = Rc::new(RefCell::new(Value{
            data: sum,
            grad: 0.0,
            children: vec![lhs, rhs],
            ops: '+',
            label
        }));

        out

    }

    pub fn mul(lhs: Rc<RefCell<Value>>, rhs: Rc<RefCell<Value>>, label: String) -> Rc<RefCell<Value>> {

        let lhs_data = lhs.borrow().data;
        let rhs_data = rhs.borrow().data;
        let mul = lhs_data * rhs_data;

        let out = Rc::new(RefCell::new(Value{
            data: mul,
            grad: 0.0,
            children: vec![lhs, rhs],
            ops: '*',
            label
        }));

        out

    }

}
