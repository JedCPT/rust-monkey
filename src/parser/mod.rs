/*****************************************************************************
 Jedda Boyle
 
 CONTAINS:
 
 NOTES:
 *****************************************************************************/

use super::lexer::Lexer;

use super::lexer::token::TokenType;
use super::lexer::token::Token;
use super::lexer::token::Precedence;

use super::ast::Expression;
use super::ast::Statement;
use super::ast::LetStatement;
use super::ast::ReturnStatement;
use super::ast::BlockStatement;
use super::ast::ExpressionStatement;

use super::ast::IdentifierExpression;
use super::ast::FunctionExpression;
use super::ast::CallExpression;
use super::ast::IfElseExpression;
use super::ast::BoolExpression;
use super::ast::InfixExpression;
use super::ast::PrefixExpression;
use super::ast::IntegralExpression;

use std::rc::Rc;

/*****************************************************************************/


pub struct Program {
    pub statements: Vec<Box<Statement>>
}

pub struct Parser {
    pub lexer: Lexer,
    pub token: Token,
    pub next_token: Token,
    errors: Vec<String>
}

impl Parser {
    
    pub fn new(mut lexer: Lexer) -> Self {
        let token = lexer.next_token();
        let next_token = lexer.next_token();
        if token.is_none() || next_token.is_none() {
            println!("Error, parser given empty lexer.")
        }
		Parser{
              lexer: lexer, 
			  token: token.unwrap(),
			  next_token: next_token.unwrap(),
              errors: Vec::new()
        }
        
	}


    // Helper functions.

    fn token_type(&self) -> TokenType {
        
        return self.token.token_type.clone();
    }

    pub fn advance_tokens(&mut self) {
        self.token = self.next_token.clone();
        self.next_token = self.lexer.next_token().unwrap();
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
        return self.token.token_type == token_type;
    }

    fn next_token_is(&mut self, token_type: TokenType) -> bool {
        return self.next_token.token_type == token_type;
    }

    fn next_token_precedence(&mut self) -> Precedence {
        return self.next_token.get_precedence();
    } 

    fn token_precedence(&mut self) -> Precedence {
        return self.token.get_precedence();
    } 

    fn next_token_error(&mut self, expected_token_type: TokenType) {
        let expected_token = Token {
            token_type: expected_token_type,
            literal: "".to_string()
        };
        self.errors.push(format!("Expected next token to be {} but got {} instead", 
        expected_token, self.token.clone()));

    }

    fn add_parse_error(&mut self, error_message: String) {
        self.errors.push(error_message);
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
        let token = self.token.clone();
        self.advance_tokens();

        let value = self.parse_expression(Precedence::Lowest);
        
        if value.is_none() {
            self.add_parse_error("Return statement missing a value expression.".to_string());
            return None;
        }
        
        if self.next_token_is(TokenType::SemiColon) {
            self.advance_tokens();

        }
         
        let to_return = ReturnStatement{token: token, value: value.unwrap()};

        return Some(Box::new(to_return));
        
    }

    fn parse_let_statement(&mut self) -> Option<Box<Statement>> {
        // let to_return: LetStatement;
        
        
        let token = self.token.clone();

        if !self.advance_tokens_if_next_token_is(TokenType::Ident) {
            return None;
        } 
        
        let identifier = IdentifierExpression{
            token: self.token.clone(), 
        };

        if !self.advance_tokens_if_next_token_is(TokenType::Assign) {
            return None;
        } 

        self.advance_tokens();
        let value = self.parse_expression(Precedence::Lowest);
        if value.is_some() {
            
            if self.next_token_is(TokenType::SemiColon) {
                self.advance_tokens();
            }
            
            
            let to_return = LetStatement {
                token: token,
                value: value.unwrap(),
                identifier: identifier
            };
            return Some(Box::new(to_return));
        } else {
            self.add_parse_error("Let statement is missing a value expression".to_string());
            return None;
        }
    }

    fn parse_block_statement(&mut self) -> Option<Box<Statement>> {
        let mut to_return = BlockStatement {
            token: self.token.clone(),
            statements: Vec::new(),
        };
        self.advance_tokens();

        while !self.token_is(TokenType::RBrace) && !self.token_is(TokenType::Eof) {
            let statement = self.parse_statement();
            if !statement.is_none() {
                to_return.statements.push(statement.unwrap());
            }   
            self.advance_tokens();
        }
        return Some(Box::new(to_return));
    }

    fn parse_expression_statement(&mut self) -> Option<Box<Statement>> {
        
        let token = self.token.clone();
        let value = self.parse_expression(Precedence::Lowest);
        if value.is_none() {
            self.add_parse_error("Expression statement is missing a value expression.".to_string());
            return None;
        }
        
        if self.next_token_is(TokenType::SemiColon) {
            self.advance_tokens();
        }
        let to_return = ExpressionStatement {
            token: token,
            value: value.unwrap()
        };

        return Some(Box::new(to_return));

    }

    fn parse_statement(&mut self) -> Option<Box<Statement>> {
        match self.token_type() {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement()
        }
        
    }


    // Parse expressions.

    fn parse_identifier_expression(&mut self) -> Option<Box<Expression>> {
        return Some(Box::new(IdentifierExpression {token: self.token.clone()}));
    }

    fn parse_integral_expression(&mut self) -> Option<Box<Expression>> {
        let token = self.token.clone();
        let value_result = token.literal.parse::<i64>();
        if value_result.is_err() {
            self.add_parse_error("Integral value not of type int.".to_string());
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
            token: self.token.clone(),
            value: self.token_is(TokenType::True),
        };
        return Some(Box::new(to_return));
    }

     fn parse_prefix_expression(&mut self) -> Option<Box<Expression>> {
        let token = self.token.clone();
        self.advance_tokens();
        let right = self.parse_expression(Precedence::Prefix);
        if right.is_none() {
            self.add_parse_error("Prefix expression missing right operand".to_string());
            return None;
        }
        let to_return = PrefixExpression {
            token: token,
            right: right.unwrap()
        };
        return Some(Box::new(to_return));
    }

    fn parse_infix_expression(&mut self, left: Box<Expression>) -> Option<Box<Expression>> {
        let token = self.token.clone();
        let precedence = self.token_precedence();
        self.advance_tokens();
        let right = self.parse_expression(precedence);
        if right.is_none() {
            self.add_parse_error("Prefix expression missing right operand".to_string());
            return None;
        }
        let to_return = InfixExpression {
            token: token,
            right: right.unwrap(),
            left: left
        };
        return Some(Box::new(to_return));

    }

    fn parse_grouped_expression(&mut self) -> Option<Box<Expression>> {
        self.advance_tokens();
        let to_return = self.parse_expression(Precedence::Lowest);
        if !self.advance_tokens_if_next_token_is(TokenType::RParen) {
            return None;
        }
        return to_return;

    }

    fn parse_ifelse_expression(&mut self) -> Option<Box<Expression>> {
        let token = self.token.clone();
        if !self.advance_tokens_if_next_token_is(TokenType::LParen) {
            return None;
        }
        self.advance_tokens();
        
        let condition = self.parse_expression(Precedence::Lowest);
        if condition.is_none() {
            self.add_parse_error("If expression invalid condition.".to_string());
            return None;
        }
        
        if !self.advance_tokens_if_next_token_is(TokenType::RParen) {
            return None;
        }
        if !self.advance_tokens_if_next_token_is(TokenType::LBrace) {
            return None;
        }
        
        let consequence = self.parse_block_statement();
        if consequence.is_none() {
            self.add_parse_error("If expression missing a consequence.".to_string());
            return None;
        }
        
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
            condition: condition.unwrap(),
            consequence: consequence.unwrap(),
            alternative: alternative
        };
        return Some(Box::new(to_return));
    }

    fn parse_func_expression(&mut self) -> Option<Box<Expression>> {
        let token = self.token.clone();
        if !self.advance_tokens_if_next_token_is(TokenType::LParen) {
            return None;
        }
        
        

        self.advance_tokens();
        let mut parameters: Vec<Box<Expression>> = Vec::new();
        while !self.token_is(TokenType::RParen) {
            parameters.push(self.parse_identifier_expression().unwrap());
            if self.next_token_is(TokenType::RParen) {
                self.advance_tokens();
                break;
            }            
            if !self.advance_tokens_if_next_token_is(TokenType::Comma) {
                return None;
            }     
            self.advance_tokens();      
        }
        if !self.advance_tokens_if_next_token_is(TokenType::LBrace) {
            return None;
        }
        let body = self.parse_block_statement();
        if body.is_none() {
            self.add_parse_error("Function missing a block statement".to_string());
            return None;
        }

        let to_return = FunctionExpression {
            token: token,
            body: Rc::new(body.unwrap()),
            parameters: Rc::new(parameters)
        };
        return Some(Box::new(to_return));
    }

    fn parse_call_expression (&mut self, func: Box<Expression>) -> Option<Box<Expression>> {
        let mut to_return = CallExpression {
            token: self.token.clone(),
            arguments: Vec::new(),
            func: func
        };
        self.advance_tokens();
        
        if self.next_token_is(TokenType::RParen) {
            return Some(Box::new(to_return));
        }

        let mut expression = self.parse_expression(Precedence::Lowest);
        if expression.is_none() {
            self.add_parse_error("Error in call argument".to_string());
            return None;
        }
        to_return.arguments.push(expression.unwrap());

        while self.next_token_is(TokenType::Comma) {
            self.advance_tokens();
            self.advance_tokens();
            // expression = self.parse_expression(Precedence::Lowest);

            // to_return.arguments.push(self.parse_expression(Precedence::Lowest).unwrap());
            expression = self.parse_expression(Precedence::Lowest);
            if expression.is_none() {
                self.add_parse_error("Error in call argument".to_string());
                return None;
            }
            to_return.arguments.push(expression.unwrap());

        }
        if !self.advance_tokens_if_next_token_is(TokenType::RParen) {
            return None;
        }

        return Some(Box::new(to_return));

    }


    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<Expression>> {
         let mut left_expression = match self.token_type() {
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

        if left_expression.is_none() {
            return None;
        }
        

        while !self.next_token_is(TokenType::SemiColon) && 
              !self.next_token_is(TokenType::Eof) &&
              (precedence as u8) < (self.next_token_precedence() as u8) {
            
            if self.next_token_is(TokenType::LParen) {
                self.advance_tokens();
                left_expression = self.parse_call_expression(left_expression.unwrap());
                

            } else if self.next_token.is_operator() { // Maybe next token isn't an operator
                self.advance_tokens();
                left_expression = self.parse_infix_expression(left_expression.unwrap());
                // return left_expression;
            } else {
                return left_expression;
            }
             
            
        }
        
        return left_expression;

    }

    pub fn parse_program(&mut self) -> Option<Program> {

        let mut program = Program{statements: Vec::new()};
        let mut statement: Option<Box<Statement>>;
        
        while !self.token_is(TokenType::Eof) {
            statement = self.parse_statement();
            if !statement.is_none() {
                // println!("{}", statement.as_ref().unwrap().to_string());
                program.statements.push(statement.unwrap());
            } 

            self.advance_tokens();
            

        }
        return Some(program);
        

    }

    
    
}

