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

    fn eval(&self, env: &mut Enviroment) -> Box<Object>;
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

    fn eval(&self, env: &mut Enviroment) -> Box<Object> {
        let value = self.value.eval(env);
        if value.get_type() == ObjectType::Error {
            return value;
        }
        env.insert(self.identifier.token.literal.clone(), value);
        return Box::new(object::Null{});
    }
    
    fn to_string(&self) -> String {
        return format!("[{} {} = {}]", self.token, self.identifier.to_string(), self.value.to_string());
        
    }
}

impl Node for ReturnStatement { 

    fn get_type(&self) -> NodeType { return NodeType::ReturnStatement; }

    fn eval(&self, env: &mut Enviroment) -> Box<Object> {
        return self.value.eval(env);
    }

    fn to_string(&self) -> String {
        return format!("[{} {}]", self.token, self.value.to_string());
    }    

}


impl Node for ExpressionStatement { 

    fn get_type(&self) -> NodeType { return NodeType::ExpressionStatement; }

    fn eval(&self, env: &mut Enviroment) -> Box<Object> {
        return self.value.eval(env);
    }

    fn to_string(&self) -> String {
        return format!("[{}]", self.value.to_string());
    }    

}


impl Node for BlockStatement { 

    fn get_type(&self) -> NodeType { return NodeType::BlockStatement; }

    fn eval(&self, env: &mut Enviroment) -> Box<Object> {
        let mut result: Box<Object> = Box::new(object::Null{});
        for statement in self.statements.iter().by_ref() {
            result = statement.eval(env);
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
    pub parameters: Vec<Box<Expression>>,
    pub body: Box<Statement>
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

    fn eval(&self, env: &mut Enviroment) -> Box<Object> {
        let value = env.get(&self.token.literal);
        
        if value.is_none() {
            return Box::new(object::Error{message: format!("Variable {} not in scope.", self.token.literal).to_string()});
        }
        if value.unwrap().get_type() == ObjectType::Integer {
            let int_value = value.unwrap().downcast_ref::<object::Integer>();
            return Box::new(object::Integer{value: int_value.unwrap().value})
        } 
        if value.unwrap().get_type() == ObjectType::Boolean {
            let bool_value = value.unwrap().downcast_ref::<object::Boolean>();
            return Box::new(object::Boolean{value: bool_value.unwrap().value});

        }
        return Box::new(object::Null{});

    }

    fn to_string(&self) -> String {
        return format!("{}", self.token);
    }  

}

impl Node for IntegralExpression { 

    fn get_type(&self) -> NodeType { return NodeType::IntegralExpression; }

    fn eval(&self, env: &mut Enviroment) -> Box<Object> {
        return Box::new(object::Integer{value: self.value});
    }

    fn to_string(&self) -> String {
        return format!("{}", self.token);
    }  

}

impl Node for BoolExpression { 

    fn get_type(&self) -> NodeType { return NodeType::BoolExpression; }

    fn eval(&self, env: &mut Enviroment) -> Box<Object> {
        return Box::new(object::Boolean{value: self.value});
    }

    fn to_string(&self) -> String {
        return format!("{}", self.token);
    }  

}

impl PrefixExpression {
   
    fn eval_minus_operator(&self, right: &Box<Object>) -> Box<Object> {
        let operand = right.downcast_ref::<object::Integer>();
        if operand.is_some() {
            let to_return = object::Integer{value: -operand.unwrap().value};
            return Box::new(to_return);
        }
        return Box::new(object::Error{message: "Error: Prefix operand is not an integer as expected".to_string()});
        
    }

    fn eval_bang_operator(&self, right: &Box<Object>) -> Box<Object> {
        let operand = right.downcast_ref::<object::Boolean>();
        if operand.is_some() {
            let to_return = object::Boolean{value: !operand.unwrap().value};
            return Box::new(to_return);
        }
        return Box::new(object::Error{message: "Error: Prefix operand is not a boolean as expected".to_string()});
    }
}

impl Node for PrefixExpression { 

    fn get_type(&self) -> NodeType { return NodeType::PrefixExpression; }

    fn eval(&self, env: &mut Enviroment) -> Box<Object> {
        let right = self.right.eval(env);
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
                 return Box::new(object::Error{message: "Error: Prefix operator not - or !..".to_string()});
             }

        }
        
    }

    fn to_string(&self) -> String {
        return format!("[{} {}]", self.token, self.right.to_string());
    }  

}

impl InfixExpression {

    fn eval_boolean_infix_expression(&self, left: &Box<Object>, right: &Box<Object>) -> Box<Object> {
        let l_value = left.downcast_ref::<object::Boolean>().unwrap().value;
        let r_value = right.downcast_ref::<object::Boolean>().unwrap().value;
        let value = match self.token.token_type { 
            TokenType::Equal =>Some(l_value == r_value),
            TokenType::NotEqual => Some(l_value != r_value),
            _ => None
        };
        if value.is_some() {
            return Box::new(object::Boolean{value: value.unwrap()});
        }
        return Box::new(object::Error{message: "Boolean infix being evaluated with invalid operand.".to_string()});

    }

    fn eval_integer_infix_expression(&self, left: &Box<Object>, right: &Box<Object>) -> Box<Object> {
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
            return Box::new(object::Integer{value: int_value.unwrap()});
        } 
        let bool_value = match self.token.token_type {
            TokenType::Lt => Some(l_value < r_value),
            TokenType::Gt => Some(l_value > r_value),
            TokenType::Equal => Some(l_value == r_value),
            TokenType::NotEqual => Some(l_value != r_value),
            _ => None
        };
        if bool_value.is_some() {
            return Box::new(object::Boolean{value: bool_value.unwrap()});
        }
        return Box::new(object::Error{message: "Integer infix being evaluated with invalid operand.".to_string()})
        
    }
}

impl Node for InfixExpression { 

    fn get_type(&self) -> NodeType { return NodeType::InfixExpression; }

    fn eval(&self, env: &mut Enviroment) -> Box<Object> {
        
        let r_operand = self.right.eval(env);
        let l_operand = self.left.eval(env);
       
        if r_operand.get_type() == ObjectType::Integer && 
           l_operand.get_type() == ObjectType::Integer {
               return self.eval_integer_infix_expression(&l_operand, &r_operand);

        } else if 
            r_operand.get_type() == ObjectType::Boolean && 
            l_operand.get_type() == ObjectType::Boolean {
                return self.eval_boolean_infix_expression(&l_operand, &r_operand);

        } else {
            return Box::new(object::Error{message: "Default".to_string()}); 
        }
        
        return Box::new(object::Error{message: "Default".to_string()});

        
    }

    fn to_string(&self) -> String {
        return format!("[{} {} {}]", self.left.to_string(), 
                                     self.token, 
                                     self.right.to_string());
    }  

}

impl Node for IfElseExpression { 

    fn get_type(&self) -> NodeType { return NodeType::IfElseExpression; }

    fn eval(&self, env: &mut Enviroment) -> Box<Object> {
        let condition = self.condition.eval(env);
        if condition.get_type() != ObjectType::Boolean {
            if condition.get_type() == ObjectType::Error {
                return condition;
            } else {
                return Box::new(object::Error{message: "Condition is not of boolean type.".to_string()});
            }
            // Error.
        }
        let bool_condition = condition.downcast_ref::<object::Boolean>().unwrap();
        if bool_condition.value == true {
             return self.consequence.eval(env);

        } else if self.alternative.is_some() {
            return self.alternative.as_ref().unwrap().eval(env);

        } else {
            return Box::new(object::Null{});
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

    fn eval(&self, env: &mut Enviroment) -> Box<Object> {
        println!("made it");
        return Box::new(object::Error{message: "Default".to_string()});
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

impl Node for CallExpression { 

    fn get_type(&self) -> NodeType { return NodeType::CallExpression; }

    fn eval(&self, env: &mut Enviroment) -> Box<Object> {
        return Box::new(object::Error{message: "Default".to_string()});
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