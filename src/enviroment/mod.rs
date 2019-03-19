/*
Author: Jedda Boyle
Contains: Enviroment
The Enviroment struct is used to store all values in the current scope.
Chained enviroments are used for nested scopes.
*/

// ================================================================================
// Imports
// ================================================================================

use super::object::Object;

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

// ================================================================================
// Struct
// ================================================================================

pub struct Enviroment {
    pub variables: HashMap<String, Rc<Object>>,
    pub outer: Option<Rc<RefCell<Enviroment>>>,
}

// ================================================================================
// Implementation.
// ================================================================================

impl Enviroment {
    pub fn new(outer: Option<Rc<RefCell<Enviroment>>>) -> Self {
        let to_return = Enviroment {
            variables: HashMap::new(),
            outer: outer,
        };
        return to_return;
    }

    pub fn insert(&mut self, key: String, value: Rc<Object>) {
        self.variables.insert(key, value);
    }

    // Return object that has the variable name 'key'.
    pub fn get(&self, key: &String) -> Option<Rc<Object>> {
        let to_return = self.variables.get(key);
        if to_return.is_some() {
            return Some(to_return.unwrap().clone());
        }
        // Variable not found so seach in the outer scope.
        if self.outer.is_some() {
            return self
                .outer
                .as_ref()
                .unwrap()
                .borrow()
                .get(key)
                .map(|x| x.clone());
        }
        return None;
    }
}
