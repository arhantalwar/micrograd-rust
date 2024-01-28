use std::rc::Rc;
use std::cell::RefCell;

pub struct Value {
    pub data: f64,
    pub grad: f64,
    pub children: Vec<Rc<RefCell<Value>>>,
    pub ops: char,
    pub label: String,
    pub backward: Option<Box<dyn Fn(Rc<RefCell<Value>>)>>
}

impl Value {

    pub fn new(data: f64, label: String) -> Value {
        Value { data, grad: 0.0, children: vec![], ops: 'N', label, backward: None }
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
            label,
            backward: None
        }));

        let backward = move |o: Rc<RefCell<Value>>| {
            println!("Backward function for {} called", o.borrow().data);
        };

        out.borrow_mut().backward = Some(Box::new(backward));
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
            label,
            backward: None
        }));

        let backward = move |o: Rc<RefCell<Value>>| {
            println!("Backward function for {} called", o.borrow().label);
        };

        out.borrow_mut().backward = Some(Box::new(backward));

        out

    }

    pub fn tanh(lhs: Rc<RefCell<Value>>, label: String) -> Rc<RefCell<Value>> {

        let lhs_data = lhs.borrow().data;
        let tanh = lhs_data.tanh();

        let out = Rc::new(RefCell::new(Value{
            data: tanh,
            grad: 0.0,
            children: vec![lhs],
            ops: 't',
            label,
            backward: None
        }));

        let backward = move |o: Rc<RefCell<Value>>| {
            println!("Backward function for {} called", o.borrow().label);
        };

        out.borrow_mut().backward = Some(Box::new(backward));

        out

    }

}
