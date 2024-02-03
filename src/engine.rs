use std::fmt::Debug;
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

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Value")
            .field("data", &self.data)
            .field("grad", &self.grad)
            .field("children", &self.children)
            .field("ops", &self.ops)
            .field("label", &self.label)
            .finish()
            
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Value {

    pub fn new(data: f64, label: String) -> Rc<RefCell<Value>> {
        Rc::new(RefCell::new(Value { data, grad: 0.0, children: vec![], ops: 'N', label, backward: None }))
    }

    pub fn add(lhs: Rc<RefCell<Value>>, rhs: Rc<RefCell<Value>>, label: String) -> Rc<RefCell<Value>> {

        let lhs_data = Rc::clone(&lhs);
        let rhs_data = Rc::clone(&rhs);
        let sum = lhs.borrow().data + rhs.borrow().data;

        let out = Rc::new(RefCell::new(Value{
            data: sum,
            grad: 0.0,
            children: vec![lhs.clone(), rhs.clone()],
            ops: '+',
            label,
            backward: None
        }));

        let backward = move |o: Rc<RefCell<Value>>| {
            lhs_data.borrow_mut().grad += o.borrow().grad;
            rhs_data.borrow_mut().grad += o.borrow().grad;
        };

        out.borrow_mut().backward = Some(Box::new(backward));
        out

    }

    pub fn mul(lhs: Rc<RefCell<Value>>, rhs: Rc<RefCell<Value>>, label: String) -> Rc<RefCell<Value>> {

        let lhs_data = Rc::clone(&lhs);
        let rhs_data = Rc::clone(&rhs);
        let mul = lhs.borrow().data * rhs.borrow().data;

        let out = Rc::new(RefCell::new(Value{
            data: mul,
            grad: 0.0,
            children: vec![lhs.clone(), rhs.clone()],
            ops: '*',
            label,
            backward: None
        }));

        let backward = move |o: Rc<RefCell<Value>>| {
            lhs_data.borrow_mut().grad += rhs.borrow().data * o.borrow().grad;
            rhs_data.borrow_mut().grad += lhs.borrow().data * o.borrow().grad;
        };

        out.borrow_mut().backward = Some(Box::new(backward));

        out

    }

    pub fn tanh(lhs: Rc<RefCell<Value>>, label: String) -> Rc<RefCell<Value>> {

        let lhs_data = Rc::clone(&lhs);
        let tanh = lhs.borrow().data.tanh();

        let out = Rc::new(RefCell::new(Value{
            data: tanh,
            grad: 0.0,
            children: vec![lhs.clone()],
            ops: 't',
            label,
            backward: None
        }));

        let backward = move |o: Rc<RefCell<Value>>| {
            lhs_data.borrow_mut().grad += (1.0 - tanh.powi(2)) * o.borrow().grad;
        };

        out.borrow_mut().backward = Some(Box::new(backward));

        out

    }

}

pub fn backward(root: &Rc<RefCell<Value>>) {

    let mut visited: Vec<Rc<RefCell<Value>>> = Vec::new();
    let mut topo: Vec<Rc<RefCell<Value>>> = Vec::new();

    build_topo(root, &mut visited, &mut topo);

    topo.reverse();

    root.borrow_mut().grad = 1.0;

    for i in &topo {
        if let Some(backward) = i.borrow().backward.as_ref() {
            println!("\tcalling backward for {:?}", i.borrow().label);
            backward(i.clone());
            println!("\t\tValue afterward {:?}", i.borrow().grad);
        };
    }

}

pub fn build_topo(root: &Rc<RefCell<Value>>,
                  visited: &mut Vec<Rc<RefCell<Value>>>,
                  topo: &mut Vec<Rc<RefCell<Value>>>) {

    if !visited.contains(root) {
        visited.push(Rc::clone(root));
        // println!("pushed {:?} to visited", root.borrow().label);
        for child in &root.borrow().children {
            build_topo(child, visited, topo)
        }
        topo.push(Rc::clone(root));
    }

}
