
#[allow(dead_code)]
pub struct Value<'a> {
    pub data: f64,
    pub grad: f64,
    pub children: Vec<&'a Value<'a>>,
    pub ops: char,
    pub label: String,
    pub backward: Box<dyn Fn() + 'a>
}

impl<'a> Value<'a> {

    pub fn new(data: f64, label: String) -> Value<'a> {

        fn none() {}

        Value { data, grad: 0.0, children: vec![], ops: 'N', label, backward: Box::new(none) }

    }

    pub fn add(self: &'a Self, rhs: &'a Self) -> Value<'a> {

        let backward = move || {
            println!("{:?}", self.data);
            println!("{:?}", self.label);
        };

        let out = Value { 
            data: (self.data + rhs.data),
            grad: 0.0,
            children: vec![self, rhs],
            ops: '+',
            label: String::from("add"),
            backward: Box::new(backward)
        };

        return out;

    }

    pub fn mul(self: &'a Self, rhs: &'a Self) -> Value<'a> {

        fn none() {}

        let out = Value {
            data: (self.data * rhs.data),
            grad: 0.0,
            children: vec![self, rhs],
            ops: '*',
            label: String::from("mul"),
            backward: Box::new(none)
        };

        return out;

    }

    pub fn tanh(self: &'a Self) -> Value<'a> {

        fn none() {}

        let out = Value { 
            data: (self.data.tanh()),
            grad: 0.0,
            children: vec![self], 
            ops: 'T',
            label: String::from("tanh"),
            backward: Box::new(none)
        };
        
        return out;

    }

}
