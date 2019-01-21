use super::enviroment::Enviroment;
use super::ast::Statement;
use super::ast::Expression;
use downcast_rs::Downcast;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(PartialEq)]
pub enum ObjectType {
    Integer,
    Boolean,
    Null,
    Error,
    Function,
}

pub trait Object: Downcast {
    // pub value: i64
    fn get_type(&self) -> ObjectType;
    fn to_string(&self) -> String;
}

impl_downcast!(Object);

pub struct Integer {
    pub value: i64
}


impl Object for Integer {
    fn get_type(&self) -> ObjectType {
        return ObjectType::Integer;
    }

    fn to_string(&self) -> String {
        return format!("{}", self.value);
    }
}

pub struct Boolean {
    pub value: bool
}


impl Object for Boolean {
    fn get_type(&self) -> ObjectType {
        return ObjectType::Boolean;

    }

    fn to_string(&self) -> String {
        return format!("{}", self.value);
    }
}

pub struct Null {
    // pub value: bool
}


impl Object for Null {
    fn get_type(&self) -> ObjectType {
        return ObjectType::Null;

    }

    fn to_string(&self) -> String {
        return format!("null");
    }
}

pub struct Error {
    pub message: String
}

impl Object for Error {
    fn get_type(&self) -> ObjectType {
        return ObjectType::Error;

    }

    fn to_string(&self) -> String {
        return format!("{}", self.message);
    }
}

pub struct Function {
    pub env: Rc<RefCell<Enviroment>>,
    pub body: Rc<Box<Statement>>,
    pub parameters: Rc<Vec<Box<Expression>>>
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