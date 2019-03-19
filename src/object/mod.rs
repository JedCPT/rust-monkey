/*
Author: Jedda Boyle
Contains: The object model.
The objects are the types of the language.
They are passed up through the AST during evaluation.
The object classes contain all the data.
*/

// ================================================================================
// Imports
// ================================================================================

use super::ast::Node;
use super::enviroment::Enviroment;
use downcast_rs::Downcast; // Crate used to downcast objects to their actual type.
use std::cell::RefCell;
use std::rc::Rc;

// ================================================================================
// Object Trait
// ================================================================================

#[derive(PartialEq)]
pub enum ObjectType {
    Integer,
    Boolean,
    Null,
    Error,
    Function,
}

pub trait Object: Downcast {
    fn get_type(&self) -> ObjectType;
    fn to_string(&self) -> String;
}

impl_downcast!(Object);

// ================================================================================
// Object structs.
// ================================================================================

pub struct Integer {
    pub value: i64,
}

pub struct Boolean {
    pub value: bool,
}

pub struct Null {}

pub struct Error {
    pub message: String,
}

pub struct Function {
    pub env: Rc<RefCell<Enviroment>>,
    pub body: Rc<Box<Node>>,
    pub parameters: Rc<Vec<Box<Node>>>,
}

// ================================================================================
// Implement Object for each type.
// ================================================================================

impl Object for Integer {
    fn get_type(&self) -> ObjectType {
        return ObjectType::Integer;
    }

    fn to_string(&self) -> String {
        return format!("{}", self.value);
    }
}

impl Object for Boolean {
    fn get_type(&self) -> ObjectType {
        return ObjectType::Boolean;
    }

    fn to_string(&self) -> String {
        return format!("{}", self.value);
    }
}

impl Object for Null {
    fn get_type(&self) -> ObjectType {
        return ObjectType::Null;
    }

    fn to_string(&self) -> String {
        return format!("null");
    }
}

impl Object for Error {
    fn get_type(&self) -> ObjectType {
        return ObjectType::Error;
    }

    fn to_string(&self) -> String {
        return format!("{}", self.message);
    }
}

impl Object for Function {
    fn get_type(&self) -> ObjectType {
        return ObjectType::Function;
    }

    fn to_string(&self) -> String {
        let mut to_return = format!("Func (");
        for par in self.parameters.iter().by_ref() {
            to_return.push_str(&par.to_string());
            to_return.push(',')
        }
        if self.parameters.len() != 0 {
            to_return.pop(); // Remove trailing comma
        }
        to_return.push_str(&")\n".to_string());
        to_return.push_str(&self.body.to_string());
        return to_return;
    }
}
