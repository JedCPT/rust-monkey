use super::enviroment::Enviroment;
use super::ast::BlockStatement;
use super::ast::IdentifierExpression;
use downcast_rs::Downcast;
use std::rc::Rc;

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
    env: Rc<Enviroment>,
    body: BlockStatement,
    paramets: Vec<IdentifierExpression>
}

impl Object for Function {
    fn get_type(&self) -> ObjectType {
        return ObjectType::Function;
    }

    fn to_string(&self) -> String {
        return "func".to_string();
    }
}