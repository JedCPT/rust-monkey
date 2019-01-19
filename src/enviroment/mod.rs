use std::collections::HashMap;
use super::object::Object;
use std::rc::Rc;

pub struct Enviroment {

    pub variables: HashMap<String, Box<Object>>,
    pub outer: Option<Rc<Enviroment>>

}

impl Enviroment {
    // pub fn new() -> Self {
    //     let to_return = Enviroment {
    //         variables: HashMap::new(),
    //         outer: None,
    //     };
    //     return to_return;

    // }

    pub fn new(outer: Option<Rc<Enviroment>>) -> Self {
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
        return self.variables.get(key);
    }
    
}