use super::lexer;

// use super::lexer::token::TokenType;
use super::lexer::token::*;

struct Program {
    statements: Vec<Box<Statement>>
}

// Define Node and Node types.
trait Node {
    fn token_literal(&self) -> &String;
    fn to_string(&self) -> String;
}

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

// Statements.
struct LetStatement {
	token: lexer::token::Token,
	value: Box<Expression>,
	identifier: IdentifierExpression
}

impl Node for LetStatement { 
    fn token_literal(&self) -> &String { return &self.token.literal } 

    fn to_string(&self) -> String {
        return format!("[{} {} = {}]", self.token, self.identifier.to_string(), self.value.as_ref().to_string());
        
    }
}

impl Statement for LetStatement { fn statement_node(&self) {} }

struct ReturnStatement {
    token: Token,
    value: Option<Box<Expression>>
}

impl Node for ReturnStatement { 
    fn token_literal(&self) -> &String { return &self.token.literal } 

    fn to_string(&self) -> String {
        if self.value.is_none() {
            return format!("Token: {}, Value: None", self.token);
        } else {
            return format!("[{} {}]", self.token, self.value.as_ref().unwrap().to_string());
        }
    }    
}

impl Statement for ReturnStatement { fn statement_node(&self) {} }

struct ExpressionStatement {
    token: Token,
    value: Option<Box<Expression>>
}

impl Node for ExpressionStatement { 
    fn token_literal(&self) -> &String { return &self.token.literal } 

    fn to_string(&self) -> String {
        if self.value.is_none() {
            return format!("Error Expression Statement.");
        } else {
            return format!("{}", self.value.as_ref().unwrap().to_string());
        }
    }    
}

impl Statement for ExpressionStatement { fn statement_node(&self) {} }


struct BlockStatement {
    token: Token, // {
    statements: Vec<Box<Statement>>
    // consequency: Option<Box<BlockStatement>>
    // alternative: Option<Box<BlockStatement>>
}

impl Statement for BlockStatement { fn statement_node(&self) {} } 

impl Node for BlockStatement { 
    fn token_literal(&self) -> &String { return &self.token.literal }

    fn to_string(&self) -> String {
        let mut to_return = "".to_string();
        for statement in self.statements.iter().by_ref() {
            to_return.push_str(&statement.to_string());
            to_return.push('\n');
        }
        return to_return;
    }  
}

// Expressions.
struct IdentifierExpression {
    token: Token,
    // value: String
}

impl Node for IdentifierExpression { 
    fn token_literal(&self) -> &String { return &self.token.literal }

    fn to_string(&self) -> String {
        return format!("{}", self.token);
    }  
}

impl Expression for IdentifierExpression { fn expression_node(&self) {} } 

struct IntegralExpression {
    token: Token,
    value: i64
}

impl Expression for IntegralExpression { fn expression_node(&self) {} } 

impl Node for IntegralExpression { 
    fn token_literal(&self) -> &String { return &self.token.literal }

    fn to_string(&self) -> String {
        return format!("{}", self.token);
    }  
}

struct BoolExpression {
    token: Token,
    value: bool
}

struct IfElseExpression {
    token: Token,
    consequence: Option<Box<Statement>>,
    alternative: Option<Box<Statement>>,
    condition: Option<Box<Expression>>
}

impl Expression for IfElseExpression { fn expression_node(&self) {} } 

impl Node for IfElseExpression { 
    fn token_literal(&self) -> &String { return &self.token.literal }

    fn to_string(&self) -> String {
        if self.alternative.is_none() {
            return format!("{} {} do \n{}" , 
                self.token,
                self.condition.as_ref().unwrap().to_string(), 
                self.consequence.as_ref().unwrap().to_string()
            );
        }
        return format!("{} {} do \n{}else do \n{}",
            self.token,
            self.condition.as_ref().unwrap().to_string(), 
            self.consequence.as_ref().unwrap().to_string(),
            self.alternative.as_ref().unwrap().to_string()
        );
        
    }  
}

impl Expression for BoolExpression { fn expression_node(&self) {} } 

impl Node for BoolExpression { 
    fn token_literal(&self) -> &String { return &self.token.literal }

    fn to_string(&self) -> String {
        return format!("{}", self.token);
    }  
}

struct PrefixExpression {
    token: Token,
    right: Option<Box<Expression>>

}

impl Expression for PrefixExpression { fn expression_node(&self) {} } 

impl Node for PrefixExpression { 
    fn token_literal(&self) -> &String { return &self.token.literal }

    fn to_string(&self) -> String {
        return format!("[{} {}]", self.token, self.right.as_ref().unwrap().to_string());
    }  
}

struct InfixExpression {
    token: Token,
    left: Option<Box<Expression>>,
    right: Option<Box<Expression>>
}

impl Expression for InfixExpression { fn expression_node(&self) {} }

impl Node for InfixExpression { 
    fn token_literal(&self) -> &String { return &self.token.literal }

    fn to_string(&self) -> String {
        return format!("[{} {} {}]", self.left.as_ref().unwrap().to_string(), 
                                   self.token, 
                                   self.right.as_ref().unwrap().to_string());
    }  
}


struct FunctionExpression {
    token: Token,
    parameters: Vec<Box<Expression>>,
    body: Option<Box<Statement>>
}

impl Expression for FunctionExpression { fn expression_node(&self) {} }

impl Node for FunctionExpression { 
    fn token_literal(&self) -> &String { return &self.token.literal }

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
        to_return.push_str(&self.body.as_ref().unwrap().to_string());
        return to_return;
       
    }  
}

struct CallExpression {
    token: Token,
    arguments: Vec<Box<Expression>>,
    func: Box<Expression>
}


impl Expression for CallExpression { fn expression_node(&self) {} }

impl Node for CallExpression { 
    fn token_literal(&self) -> &String { return &self.token.literal }

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

pub struct Parser {
    pub lexer: lexer::Lexer,
    pub token: Option<lexer::token::Token>,
    pub next_token: Option<lexer::token::Token>,
    errors: Vec<String>
}

impl Parser {
    
    pub fn new(mut lexer: lexer::Lexer) -> Self {
        let token = lexer.next_token();
        let next_token = lexer.next_token();
		Parser{
              lexer: lexer, 
			  token: token,
			  next_token: next_token,
              errors: Vec::new()
        }
        
	}


    // Helper functions.

    fn token_type(&self) -> Option<TokenType> {
        if self.token.is_none() {
            return None;
        }
        return Some(self.token.as_ref().unwrap().token_type.clone());
    }

    pub fn advance_tokens(&mut self) {
        self.token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn advance_tokens_if_next_token_is(&mut self, expected_type: TokenType) -> bool {
        
        if self.next_token_is(expected_type) {
            self.advance_tokens();
            return true;
        } else {
            self.next_token_error(expected_type);
            return false;
        }
    }

    fn token_is(&mut self, token_type: TokenType) -> bool {
        return self.token.as_ref().unwrap().token_type == token_type
    }

    fn next_token_is(&mut self, token_type: TokenType) -> bool {
        if self.next_token.is_none() {
            return false;
        }
        return self.next_token.as_ref().unwrap().token_type == token_type
    }

    fn next_token_precedence(&mut self) -> Option<Precedence> {
        if self.next_token.is_none() {
            return None;
        }
        return Some(self.next_token.as_ref().unwrap().get_precedence());
    } 

    fn token_precedence(&mut self) -> Option<Precedence> {
        if self.token.is_none() {
            return None;
        }
        return Some(self.token.as_ref().unwrap().get_precedence());
    } 

    fn next_token_error(&mut self, expected_token_type: TokenType) {
        // TODO. Keep track of all errors.
        let expected_token = Token {
            token_type: expected_token_type,
            literal: "".to_string()
        };
        self.errors.push(format!("Expected next token to be {} but got {} instead", 
        expected_token, self.token.as_ref().unwrap().clone()));

    }

    pub fn print_parse_errors(&mut self) {
        if self.errors.len() == 0 {
            return;
        }
        println!("Woops! We ran into some monkey business here!");
        println!("Parse Errors");
        for err in self.errors.iter().as_ref() {
            println!("\t{}", err);
        }
    }

    // Parse statements.

    fn parse_return_statement(&mut self) -> Option<Box<Statement>> {
        let token = self.token.as_ref().unwrap().clone();
        self.advance_tokens();

        // Todo make value expression.
        
        let value = self.parse_expression(Precedence::Lowest);
        
        if self.next_token_is(TokenType::SemiColon) {
            self.advance_tokens();

        }
         
        let to_return = ReturnStatement{token: token, value: value};

        return Some(Box::new(to_return));
        
    }

    fn parse_let_statement(&mut self) -> Option<Box<Statement>> {
        let to_return: LetStatement;
        
        let token = self.token.as_ref().unwrap().clone();

        if !self.advance_tokens_if_next_token_is(TokenType::Ident) {
            return None;
        } 
        
        let identifier = IdentifierExpression{
            token: self.token.as_ref().unwrap().clone(), 
        };

        if !self.advance_tokens_if_next_token_is(TokenType::Assign) {
            return None;
        } 

        self.advance_tokens();
        let value = self.parse_expression(Precedence::Lowest);
        if self.next_token_is(TokenType::SemiColon) {
            self.advance_tokens();
        }
        
        
        let to_return = LetStatement {
            token: token,
            value: value,
            identifier: identifier
        };
        return Some(Box::new(to_return));
    }

    fn parse_block_statement(&mut self) -> Option<Box<Statement>> {
        let mut to_return = BlockStatement {
            token: self.token.as_ref().unwrap().clone(),
            statements: Vec::new(),
        };
        self.advance_tokens();

        while !self.token_is(TokenType::RBrace) && !self.token.is_none() {
            let statement = self.parse_statement();
            if !statement.is_none() {
                to_return.statements.push(statement.unwrap());
            }   
            self.advance_tokens();
        }
        return Some(Box::new(to_return));
    }

    fn parse_expression_statement(&mut self) -> Option<Box<Statement>> {
        
        let token = self.token.as_ref().unwrap().clone();
        let value = self.parse_expression(Precedence::Lowest);
        
        if self.next_token_is(TokenType::SemiColon) {
            self.advance_tokens();
        }
        let to_return = ExpressionStatement {
            token: token,
            value: value
        };

        return Some(Box::new(to_return));

    }

    fn parse_statement(&mut self) -> Option<Box<Statement>> {
        match self.token_type().unwrap() {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement()
        }
        
    }


    // Parse expressions.

    fn parse_identifier_expression(&mut self) -> Option<Box<Expression>> {
        return Some(Box::new(IdentifierExpression {token: self.token.as_ref().unwrap().clone()}));
    }

    fn parse_integral_expression(&mut self) -> Option<Box<Expression>> {
        let token = self.token.as_ref().unwrap().clone();
        let value_result = token.literal.parse::<i64>();
        if value_result.is_err() {
            println!("Error in parse integral.");
            return None;
        }
        let to_return = IntegralExpression {
            token: token,
            value: value_result.unwrap()
        };
        return Some(Box::new(to_return));
    }

    fn parse_bool_expression(&mut self) -> Option<Box<Expression>> {
        let to_return = BoolExpression {
            token: self.token.as_ref().unwrap().clone(),
            value: self.token_is(TokenType::True),
        };
        return Some(Box::new(to_return));
    }

     fn parse_prefix_expression(&mut self) -> Option<Box<Expression>> {
        let token = self.token.as_ref().unwrap().clone();
        self.advance_tokens();
        let right = self.parse_expression(Precedence::Prefix);
        let to_return = PrefixExpression {
            token: token,
            right: right
        };
        return Some(Box::new(to_return));
    }

    fn parse_infix_expression(&mut self, left: Option<Box<Expression>>) -> Option<Box<Expression>> {
        let token = self.token.as_ref().unwrap().clone();
        let precedence = self.token_precedence().unwrap();
        self.advance_tokens();
        let right = self.parse_expression(precedence);
        let to_return = InfixExpression {
            token: token,
            right: right,
            left: left
        };
        return Some(Box::new(to_return));

    }

    fn parse_grouped_expression(&mut self) -> Option<Box<Expression>> {
        self.advance_tokens();
        let to_return = self.parse_expression(Precedence::Lowest);
        if !self.advance_tokens_if_next_token_is(TokenType::RParen) {
            println!{"Error, no matching right bracket."};
            return None;
        }
        return to_return;

    }

    fn parse_ifelse_expression(&mut self) -> Option<Box<Expression>> {
        let token = self.token.as_ref().unwrap().clone();
        if !self.advance_tokens_if_next_token_is(TokenType::LParen) {
            return None;
        }
        self.advance_tokens();
        
        let condition = self.parse_expression(Precedence::Lowest);
        
        if !self.advance_tokens_if_next_token_is(TokenType::RParen) {
            return None;
        }
        if !self.advance_tokens_if_next_token_is(TokenType::LBrace) {
            return None;
        }
        
        let consequence = self.parse_block_statement();
        
        let mut alternative: Option<Box<Statement>> = None;
        

        if self.next_token_is(TokenType::Else) {
            self.advance_tokens();
            if !self.advance_tokens_if_next_token_is(TokenType::LBrace) {
                return None;
            }
            alternative = self.parse_block_statement();
        }
       
        let to_return = IfElseExpression{
            token: token,
            condition: condition,
            consequence: consequence,
            alternative: alternative
        };
        return Some(Box::new(to_return));
    }

    fn parse_func_expression(&mut self) -> Option<Box<Expression>> {
        let token = self.token.as_ref().unwrap().clone();
        if !self.advance_tokens_if_next_token_is(TokenType::LParen) {
            // println!("here2");
            return None;
        }
        
        

        self.advance_tokens();
        let mut parameters: Vec<Box<Expression>> = Vec::new();
        while !self.token_is(TokenType::RParen) {
        // while true {
            // println!("{}", self.token.as_ref().unwrap());
            parameters.push(self.parse_identifier_expression().unwrap());
            if self.next_token_is(TokenType::RParen) {
                self.advance_tokens();
                break;
            }            
            if !self.advance_tokens_if_next_token_is(TokenType::Comma) {
                // println!("{}", self.token.as_ref().unwrap());
                // println!("here2");
                return None;
            }     
            self.advance_tokens();      
        }
        if !self.advance_tokens_if_next_token_is(TokenType::LBrace) {
            // println!("here23");
            return None;
        }
        let body = self.parse_block_statement();

        let to_return = FunctionExpression {
            token: token,
            body: body,
            parameters: parameters
        };
        // println!("{}", to_return.body.as_ref().unwrap().to_string());
        return Some(Box::new(to_return));
    }


// struct CallExpression {
//     token: Token,
//     arguments: Vec<Box<Expression>>,
//     func: Box<Expression>
// }
    fn parse_call_expression (&mut self, func: Option<Box<Expression>>) -> Option<Box<Expression>> {
        let mut to_return = CallExpression {
            token: self.token.as_ref().unwrap().clone(),
            arguments: Vec::new(),
            func: func.unwrap()
        };
        self.advance_tokens();
        
        if self.next_token_is(TokenType::RParen) {
            return Some(Box::new(to_return));
        }

        to_return.arguments.push(self.parse_expression(Precedence::Lowest).unwrap());
        // let mut args: Vec<Box<Expression>>;
        while self.next_token_is(TokenType::Comma) {
            self.advance_tokens();
            self.advance_tokens();
            to_return.arguments.push(self.parse_expression(Precedence::Lowest).unwrap());

        }
        if !self.advance_tokens_if_next_token_is(TokenType::RParen) {
            return None;
        }

        return Some(Box::new(to_return));

    }


    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<Expression>> {
         let mut left_expression = match self.token_type().unwrap() {
            TokenType::Ident => self.parse_identifier_expression(),
            TokenType::Int => self.parse_integral_expression(),
            TokenType::Bang => self.parse_prefix_expression(),
            TokenType::Minus => self.parse_prefix_expression(),
            TokenType::True => self.parse_bool_expression(),
            TokenType::False => self.parse_bool_expression(),
            TokenType::LParen => self.parse_grouped_expression(),
            TokenType::If => self.parse_ifelse_expression(),
            TokenType::Function => self.parse_func_expression(),
            _ => None 
        };
        // println!("LEFT:{}", left_expression.as_ref().unwrap().to_string());
                // println!("LEFT:{}", self.next_token.as_ref().unwrap());
        

        while !self.next_token_is(TokenType::SemiColon) && 
              !self.next_token.is_none() &&
              (precedence as u8) < (self.next_token_precedence().unwrap() as u8) {
            
            if self.next_token_is(TokenType::LParen) {
                self.advance_tokens();
                left_expression = self.parse_call_expression(left_expression);
                

            } else if self.next_token.as_ref().unwrap().is_operator() { // Maybe next token isn't an operator
                self.advance_tokens();
                left_expression = self.parse_infix_expression(left_expression);
                // return left_expression;
            } else {
                return left_expression;
            }
             
            
        }
        
        return left_expression;

    }



   

    pub fn parse_program(&mut self) {

        let mut program = Program{statements: Vec::new()};
        let mut statement: Option<Box<Statement>>;
        
        while !self.token.is_none() {
            let statement = self.parse_statement();
            // let x = catch {self.parse_statement()};
            if !statement.is_none() {
                println!("{}", statement.as_ref().unwrap().to_string());
                program.statements.push(statement.unwrap());
            } 

            self.advance_tokens();
            

        }
        

    }

    
    
}

