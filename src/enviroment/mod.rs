use std::collections::HashMap;
use super::object::Object;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Enviroment {

    pub variables: HashMap<String, Box<Object>>,
    pub outer: Option<Rc<RefCell<Enviroment>>>

}

impl Enviroment {

    pub fn new(outer: Option<Rc<RefCell<Enviroment>>>) -> Self {
        let to_return = Enviroment {
            variables: HashMap::new(),
            outer: outer,
        };
        return to_return;
    }

    pub fn insert(&mut self, key: String, value: Box<Object>) {
        self.variables.insert(key, value);
    }

    pub fn get(&self, key: &String) -> Option<&Box<Object>> {
        let to_return = self.variables.get(key);
        if to_return.is_some() {
            return to_return;
        }
        if self.outer.is_some() {
            let x = self.outer.as_ref().unwrap().clone();
            return x.borrow().get(key);
        }
        return None;
        
    }
    
}