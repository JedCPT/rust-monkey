/*****************************************************************************
 Jedda Boyle
 
 CONTAINS:
 Definition and implementations of the Abstract Syntax Tree nodes. 
 The nodes are either an expression or statement.
 
 NOTES:
 *****************************************************************************/


use super::lexer::token::Token;
use super::lexer::token::TokenType;
use super::object::Object;
use super::object::ObjectType;
use super::object;
use super::enviroment::Enviroment;
// use super::object::Integer;
// use super::object::Boolean
// #[macro_use]
use downcast_rs::Downcast;
use std::cell::RefCell;
use std::rc::Rc;
// extern crate downcast_rs;

// #[macro_use]
// extern crate downcast;
// use downcast::Any;

/*****************************************************************************/

// macro_rules! is_none {
//     ($x:expr) => {
//         if $x.is_none() {
//             println!("Option on line {} in file {} is None.", line!(), file!());
//             return None;
//         }
//     }
// }

#[derive(PartialEq)]
pub enum NodeType {
    LetStatement,
    ReturnStatement,
    ExpressionStatement,
    BlockStatement,

    IdentifierExpression,
    IntegralExpression,
    BoolExpression,
    PrefixExpression,
    InfixExpression,
    IfElseExpression,
    FunctionExpression,
    CallExpression
}

pub trait Node {

    fn get_type(&self) -> NodeType;

    fn to_string(&self) -> String;

    fn eval(&self, env: Rc<RefCell<Enviroment>>) -> Rc<Object>;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node { 
    fn expression_node(&self); 
}

/*****************************************************************************
 Statement Nodes.
 *****************************************************************************/

pub struct LetStatement {
	pub token: Token,
	pub value: Box<Expression>,
	pub identifier: IdentifierExpression
}

pub struct ReturnStatement {
    pub token: Token,
    pub value: Box<Expression>
}

pub struct ExpressionStatement {
    pub token: Token,
    pub value: Box<Expression>
}

pub struct BlockStatement {
    pub token: Token, 
    pub statements: Vec<Box<Statement>>
}

// Make sure all statements adhere to the statement trait.

impl Statement for LetStatement        { fn statement_node(&self) {} }

impl Statement for ReturnStatement     { fn statement_node(&self) {} }

impl Statement for ExpressionStatement { fn statement_node(&self) {} }

impl Statement for BlockStatement      { fn statement_node(&self) {} } 

// Implement Node for each statement.

impl Node for LetStatement { 

    fn get_type(&self) -> NodeType { return NodeType::LetStatement; }

    fn eval(&self, env: Rc<RefCell<Enviroment>>) -> Rc<Object> {
        let value = self.value.eval(env.clone());
        if value.get_type() == ObjectType::Error {
            return value;
        }
        env.borrow_mut().insert(self.identifier.token.literal.clone(), value);
        return Rc::new(object::Null{});
    }
    
    fn to_string(&self) -> String {
        return format!("[{} {} = {}]", self.token, self.identifier.to_string(), self.value.to_string());
        
    }
}

impl Node for ReturnStatement { 

    fn get_type(&self) -> NodeType { return NodeType::ReturnStatement; }

    fn eval(&self, env: Rc<RefCell<Enviroment>>) -> Rc<Object> {
        return self.value.eval(env);
    }

    fn to_string(&self) -> String {
        return format!("[{} {}]", self.token, self.value.to_string());
    }    

}


impl Node for ExpressionStatement { 

    fn get_type(&self) -> NodeType { return NodeType::ExpressionStatement; }

    fn eval(&self, env: Rc<RefCell<Enviroment>>) -> Rc<Object> {
        return self.value.eval(env);
    }

    fn to_string(&self) -> String {
        return format!("[{}]", self.value.to_string());
    }    

}


impl Node for BlockStatement { 

    fn get_type(&self) -> NodeType { return NodeType::BlockStatement; }

    fn eval(&self, env: Rc<RefCell<Enviroment>>) -> Rc<Object> {
        let mut result: Rc<Object> = Rc::new(object::Null{});
        for statement in self.statements.iter().by_ref() {
            result = statement.eval(env.clone());
            if statement.get_type() == NodeType::ReturnStatement {
                return result;
            }
        }
        return result;
        
    }

    fn to_string(&self) -> String {
        let mut to_return = "".to_string();
        for statement in self.statements.iter().by_ref() {
            to_return.push_str(&statement.to_string());
            to_return.push('\n');
        }
        return to_return;
    }  

}

/*****************************************************************************
 Expression Nodes.
 *****************************************************************************/

pub struct IdentifierExpression {
    pub token: Token,
}

pub struct IntegralExpression {
    pub token: Token,
    pub value: i64
}

pub struct BoolExpression {
    pub token: Token,
    pub value: bool
}

pub struct PrefixExpression {
    pub token: Token,
    pub right: Box<Expression>
}

pub struct InfixExpression {
    pub token: Token,
    pub left: Box<Expression>,
    pub right: Box<Expression>
}

pub struct IfElseExpression {
    pub token: Token,
    pub condition: Box<Expression>,
    pub consequence: Box<Statement>,
    pub alternative: Option<Box<Statement>>
}

pub struct FunctionExpression {
    pub token: Token,
    pub parameters: Rc<Vec<Box<Expression>>>,
    pub body: Rc<Box<Statement>>
}

pub struct CallExpression {
    pub token: Token,
    pub arguments: Vec<Box<Expression>>,
    pub func: Box<Expression>
}

// Ensure that all expressions adhere to the Node trait.

impl Expression for IdentifierExpression { fn expression_node(&self) {} } 

impl Expression for IntegralExpression   { fn expression_node(&self) {} } 

impl Expression for BoolExpression       { fn expression_node(&self) {} }

impl Expression for PrefixExpression     { fn expression_node(&self) {} } 

impl Expression for InfixExpression      { fn expression_node(&self) {} }

impl Expression for IfElseExpression     { fn expression_node(&self) {} } 

impl Expression for FunctionExpression   { fn expression_node(&self) {} }

impl Expression for CallExpression       { fn expression_node(&self) {} }

// Implement Node for each expression.

impl Node for IdentifierExpression { 

    fn get_type(&self) -> NodeType { return NodeType::IdentifierExpression; }

    fn eval(&self, env: Rc<RefCell<Enviroment>>) -> Rc<Object> {
        let borrowed_env = env.borrow();
        // let value = env.borrow().get(&self.token.literal);
        let value = borrowed_env.get(&self.token.literal);
        
        if value.is_none() {
            return Rc::new(object::Error{message: format!("Variable {} not in scope.", self.token.literal).to_string()});
        }
        if value.as_ref().unwrap().get_type() == ObjectType::Integer {
            let int_value = value.as_ref().unwrap().downcast_ref::<object::Integer>().unwrap();
            return Rc::new(object::Integer{value: int_value.value})
        } 
        if value.as_ref().unwrap().get_type() == ObjectType::Boolean {
            let bool_value = value.as_ref().unwrap().downcast_ref::<object::Boolean>().unwrap();
            return Rc::new(object::Boolean{value: bool_value.value});

        }
        if value.as_ref().unwrap().get_type() == ObjectType::Function {
            let func_value = value.as_ref().unwrap().downcast_ref::<object::Function>().unwrap();
            return Rc::new(
                object::Function{env: func_value.env.clone(), 
                                 body: func_value.body.clone(), 
                                 parameters: func_value.parameters.clone()
                }
            );
        }
        return Rc::new(object::Error{message: format!("Variable {} in scope but not returned.", self.token.literal).to_string()});


    }

    fn to_string(&self) -> String {
        return format!("{}", self.token);
    }  

}

impl Node for IntegralExpression { 

    fn get_type(&self) -> NodeType { return NodeType::IntegralExpression; }

    fn eval(&self, env: Rc<RefCell<Enviroment>>) -> Rc<Object> {
        return Rc::new(object::Integer{value: self.value});
    }

    fn to_string(&self) -> String {
        return format!("{}", self.token);
    }  

}

impl Node for BoolExpression { 

    fn get_type(&self) -> NodeType { return NodeType::BoolExpression; }

    fn eval(&self, env: Rc<RefCell<Enviroment>>) -> Rc<Object> {
        return Rc::new(object::Boolean{value: self.value});
    }

    fn to_string(&self) -> String {
        return format!("{}", self.token);
    }  

}

impl PrefixExpression {
   
    fn eval_minus_operator(&self, right: &Rc<Object>) -> Rc<Object> {
        let operand = right.downcast_ref::<object::Integer>();
        if operand.is_some() {
            let to_return = object::Integer{value: -operand.unwrap().value};
            return Rc::new(to_return);
        }
        return Rc::new(object::Error{message: "Error: Prefix operand is not an integer as expected".to_string()});
        
    }

    fn eval_bang_operator(&self, right: &Rc<Object>) -> Rc<Object> {
        let operand = right.downcast_ref::<object::Boolean>();
        if operand.is_some() {
            let to_return = object::Boolean{value: !operand.unwrap().value};
            return Rc::new(to_return);
        }
        return Rc::new(object::Error{message: "Error: Prefix operand is not a boolean as expected".to_string()});
    }
}

impl Node for PrefixExpression { 

    fn get_type(&self) -> NodeType { return NodeType::PrefixExpression; }

    fn eval(&self, env: Rc<RefCell<Enviroment>>) -> Rc<Object> {
        let right = self.right.eval(env.clone());
        // println!("here");
        if right.get_type() == ObjectType::Error { return right; }
       
        match self.token.token_type {
            TokenType::Bang => {
                return self.eval_bang_operator(&right);
                
            },
            TokenType::Minus => {
                return self.eval_minus_operator(&right);
                
            },
             _ => {
                 return Rc::new(object::Error{message: "Error: Prefix operator not - or !..".to_string()});
             }

        }
        
    }

    fn to_string(&self) -> String {
        return format!("[{} {}]", self.token, self.right.to_string());
    }  

}

impl InfixExpression {

    fn eval_boolean_infix_expression(&self, left: &Rc<Object>, right: &Rc<Object>) -> Rc<Object> {
        let l_value = left.downcast_ref::<object::Boolean>().unwrap().value;
        let r_value = right.downcast_ref::<object::Boolean>().unwrap().value;
        let value = match self.token.token_type { 
            TokenType::Equal =>Some(l_value == r_value),
            TokenType::NotEqual => Some(l_value != r_value),
            _ => None
        };
        if value.is_some() {
            return Rc::new(object::Boolean{value: value.unwrap()});
        }
        return Rc::new(object::Error{message: "Boolean infix being evaluated with invalid operand.".to_string()});

    }

    fn eval_integer_infix_expression(&self, left: &Rc<Object>, right: &Rc<Object>) -> Rc<Object> {
        let l_value = left.downcast_ref::<object::Integer>().unwrap().value;
        let r_value = right.downcast_ref::<object::Integer>().unwrap().value;
         
        let int_value = match self.token.token_type {
            TokenType::Plus => Some(l_value + r_value),
            TokenType::Minus => Some(l_value - r_value),
            TokenType::Asterisk => Some(l_value * r_value),
            TokenType::Slash => Some(l_value / r_value),
            _ => None
        };
        if int_value.is_some() {
            return Rc::new(object::Integer{value: int_value.unwrap()});
        } 
        let bool_value = match self.token.token_type {
            TokenType::Lt => Some(l_value < r_value),
            TokenType::Gt => Some(l_value > r_value),
            TokenType::Equal => Some(l_value == r_value),
            TokenType::NotEqual => Some(l_value != r_value),
            _ => None
        };
        if bool_value.is_some() {
            return Rc::new(object::Boolean{value: bool_value.unwrap()});
        }
        return Rc::new(object::Error{message: "Integer infix being evaluated with invalid operand.".to_string()})
        
    }
}

impl Node for InfixExpression { 

    fn get_type(&self) -> NodeType { return NodeType::InfixExpression; }

    fn eval(&self, env: Rc<RefCell<Enviroment>>) -> Rc<Object> {
        
        let r_operand = self.right.eval(env.clone());
        let l_operand = self.left.eval(env.clone());
       
        if r_operand.get_type() == ObjectType::Integer && 
           l_operand.get_type() == ObjectType::Integer {
               return self.eval_integer_infix_expression(&l_operand, &r_operand);

        } else if 
            r_operand.get_type() == ObjectType::Boolean && 
            l_operand.get_type() == ObjectType::Boolean {
                return self.eval_boolean_infix_expression(&l_operand, &r_operand);

        } else {
            return Rc::new(object::Error{message: "Default".to_string()}); 
        }
        
        return Rc::new(object::Error{message: "Default".to_string()});

        
    }

    fn to_string(&self) -> String {
        return format!("[{} {} {}]", self.left.to_string(), 
                                     self.token, 
                                     self.right.to_string());
    }  

}

impl Node for IfElseExpression { 

    fn get_type(&self) -> NodeType { return NodeType::IfElseExpression; }

    fn eval(&self, env: Rc<RefCell<Enviroment>>) -> Rc<Object> {
        let condition = self.condition.eval(env.clone());
        if condition.get_type() != ObjectType::Boolean {
            if condition.get_type() == ObjectType::Error {
                return condition;
            } else {
                return Rc::new(object::Error{message: "Condition is not of boolean type.".to_string()});
            }
            // Error.
        }
        let bool_condition = condition.downcast_ref::<object::Boolean>().unwrap();
        if bool_condition.value == true {
             return self.consequence.eval(env.clone());

        } else if self.alternative.is_some() {
            return self.alternative.as_ref().unwrap().eval(env.clone());

        } else {
            return Rc::new(object::Null{});
        }

    }

    fn to_string(&self) -> String {
        if self.alternative.is_none() {
            return format!("{} {} do \n{}" , 
                self.token,
                self.condition.to_string(), 
                self.consequence.to_string()
            );
        }
        return format!("{} {} do \n{}else do \n{}",
            self.token,
            self.condition.to_string(), 
            self.consequence.to_string(),
            self.alternative.as_ref().unwrap().to_string()
        );
        
    }  
}

impl Node for FunctionExpression { 

    fn get_type(&self) -> NodeType { return NodeType::FunctionExpression; }

    fn eval(&self, env: Rc<RefCell<Enviroment>>) -> Rc<Object> {
        
        let to_return = object::Function {
            body: self.body.clone(),
            parameters: self.parameters.clone(),
            env: Rc::new(RefCell::new(Enviroment::new(Some(env.clone())))),
        };

        return Rc::new(to_return);
    }

    fn to_string(&self) -> String {
        let mut to_return = format!("{} (", self.token);
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


// pub struct CallExpression {
//     pub token: Token,
//     pub arguments: Vec<Box<Expression>>,
//     pub func: Box<Expression>
// }

impl CallExpression {
    fn eval_arguments(&self, env: Rc<RefCell<Enviroment>> ) -> Vec<Rc<Object>> {
        let mut to_return: Vec<Rc<Object>> = Vec::new();
        let mut result: Rc<Object>;
        for arg in self.arguments.iter().by_ref() {
            result = arg.eval(env.clone());
            if result.get_type() == ObjectType::Error {
                to_return.clear();
                to_return.push(result);
                return to_return;

            }
            println!("{}", result.to_string());
            to_return.push(result);
            
        }

        return to_return;

    }

    fn extend_enviroment(&self, args:Vec<Rc<Object>>) {


    }
}

impl Node for CallExpression { 

    fn get_type(&self) -> NodeType { return NodeType::CallExpression; }

    fn eval(&self, env: Rc<RefCell<Enviroment>>) -> Rc<Object> {
        let func = self.func.eval(env.clone());
        if func.get_type() != ObjectType::Function {
            return Rc::new(object::Error{message: "Invalid function.".to_string()});
        }
        // if self.func.get_type() == NodeType::IdentifierExpression {
        //     // let ident = self.func.downcast_ref::<object::Integer>().unwrap().value;
        //     let func = self.func.eval(env);
        //     // func = env.borrow().get(&self.func.token.literal);
        //     // println!("sfsdf");
        // }
        println!("{}", self.func.to_string());
        // let func = env.get()
        let arguments = self.eval_arguments(env);
        if arguments.len() == 1 && arguments[0].get_type() == ObjectType::Error {
            return Rc::new(object::Error{message: "Couldn't evaluate function arguments.".to_string()});
        }
        return Rc::new(object::Error{message: "Default".to_string()});
    }

    fn to_string(&self) -> String {
        let mut to_return = format!("[{} (", self.func.to_string());
        for arg in self.arguments.iter().by_ref() {
            to_return.push_str(&arg.to_string());
            to_return.push(',')
        }
        if self.arguments.len() != 0 {
            to_return.pop(); // Remove trailing comma
        }
        to_return.push_str(&")]".to_string());
        return to_return;
       
    }  
}