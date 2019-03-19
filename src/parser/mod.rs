/*
Author: Jedda Boyle
Contains: Parser
The parser does the syntatic analysis on the tokens produced by the lexer
to ensure that they represent valid rust-monkey code.
The parser returns an abstract syntax tree represenation of the code which
can be evaluated.
*/

// ================================================================================
// Imports
// ================================================================================

use super::lexer::token::Precedence;
use super::lexer::token::Token;
use super::lexer::token::TokenType;
use super::lexer::Lexer;

use super::ast::BlockStatement;
use super::ast::ExpressionStatement;
use super::ast::LetStatement;
use super::ast::Node;
use super::ast::ReturnStatement;

use super::ast::BoolExpression;
use super::ast::CallExpression;
use super::ast::FunctionExpression;
use super::ast::IdentifierExpression;
use super::ast::IfElseExpression;
use super::ast::InfixExpression;
use super::ast::IntegralExpression;
use super::ast::PrefixExpression;

use std::rc::Rc;

// ================================================================================
// Define structures.
// ================================================================================

pub struct Program {
    pub statements: Vec<Box<Node>>,
}

pub struct Parser {
    pub lexer: Lexer,
    pub token: Token,
    pub next_token: Token,
    errors: Vec<String>, // Keep a vector of strings which record all the erros incounted in the parsing.
}

// ================================================================================
// Implement structures.
// ================================================================================

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let token = lexer.next_token();
        let next_token = lexer.next_token();
        if token.is_none() || next_token.is_none() {
            println!("Error, parser given empty lexer.")
        }
        Parser {
            lexer: lexer,
            token: token.unwrap(),
            next_token: next_token.unwrap(),
            errors: Vec::new(),
        }
    }

    // Parse tokens given by the lexer to return a program which is a vector of statements.
    pub fn parse_program(&mut self, debug: bool) -> Option<Program> {
        let mut program = Program {
            statements: Vec::new(),
        };
        let mut statement: Option<Box<Node>>;

        while !self.token_is(TokenType::Eof) {
            statement = self.parse_statement();
            if !statement.is_none() {
                if debug {
                    println!("{}", statement.as_ref().unwrap().to_string());
                }
                program.statements.push(statement.unwrap());
            }
            self.advance_tokens();
        }
        self.print_parse_errors();
        return Some(program);
    }

    // ================================================================================
    // Get functions.
    // ================================================================================

    fn token_type(&self) -> TokenType {
        return self.token.token_type.clone();
    }

    fn token_is(&mut self, token_type: TokenType) -> bool {
        return self.token.token_type == token_type;
    }

    fn next_token_is(&mut self, token_type: TokenType) -> bool {
        return self.next_token.token_type == token_type;
    }

    fn token_precedence(&mut self) -> Precedence {
        return self.token.get_precedence();
    }

    fn next_token_precedence(&mut self) -> Precedence {
        return self.next_token.get_precedence();
    }

    // ================================================================================
    // Advance functions.
    // ================================================================================

    pub fn advance_tokens(&mut self) {
        self.token = self.next_token.clone();
        self.next_token = self.lexer.next_token().unwrap();
    }

    fn advance_tokens_if_next_token_is(&mut self, expected_type: TokenType) -> bool {
        if self.next_token_is(expected_type) {
            self.advance_tokens();
            return true;
        }
        return false;
    }

    // ================================================================================
    // Error handling functions.
    // ================================================================================

    fn log_parse_error(&mut self, error_message: String) {
        self.errors.push(error_message);
    }

    fn log_next_token_error(&mut self, expected_token_type: TokenType) {
        let expected_token = Token {
            token_type: expected_token_type,
            literal: "".to_string(),
        };
        self.errors.push(format!(
            "Expected next token to be {} but got {} instead",
            expected_token,
            self.next_token.clone()
        ));
    }

    pub fn print_parse_errors(&mut self) {
        if self.errors.len() == 0 {
            return;
        }
        println!("Woops! We ran into some monkey business here!");
        for err in self.errors.iter().as_ref() {
            println!("\t{}", err);
        }
    }

    // ================================================================================
    // Functions for parsing statements.
    // ================================================================================

    fn parse_statement(&mut self) -> Option<Box<Node>> {
        match self.token_type() {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<Node>> {
        // Parse identifier.
        if !self.advance_tokens_if_next_token_is(TokenType::Ident) {
            self.log_parse_error("Let statement is missing an identifier.".to_string());
            return None;
        }
        let identifier = IdentifierExpression {
            token: self.token.clone(),
        };

        // Move over assignment operator.
        if !self.advance_tokens_if_next_token_is(TokenType::Assign) {
            self.log_parse_error("Let statement is missing an assignment operator.".to_string());
            return None;
        }

        // Parse expression.
        self.advance_tokens();
        let value = self.parse_expression(Precedence::Lowest);
        if value.is_none() {
            self.log_parse_error("Let statement is missing a value expression".to_string());
            return None;
        }

        // Check if line ends in semicolon.
        if !self.advance_tokens_if_next_token_is(TokenType::SemiColon) {
            self.log_parse_error("Line does not end in semicolon.".to_string());
            return None;
        }

        let to_return = LetStatement {
            token: Token::new("let".to_string()),
            value: value.unwrap(),
            identifier: identifier,
        };
        return Some(Box::new(to_return));
    }

    fn parse_return_statement(&mut self) -> Option<Box<Node>> {
        // Parse return expression.
        self.advance_tokens();
        let value = self.parse_expression(Precedence::Lowest);
        if value.is_none() {
            self.log_parse_error("Return statement missing a value expression.".to_string());
            return None;
        }

        // Check that line ends in a semicolon.
        if !self.advance_tokens_if_next_token_is(TokenType::SemiColon) {
            self.log_parse_error("Line does not end in a semicolon.".to_string());
            return None;
        }

        let to_return = ReturnStatement {
            token: Token::new("return".to_string()),
            value: value.unwrap(),
        };
        return Some(Box::new(to_return));
    }

    fn parse_expression_statement(&mut self) -> Option<Box<Node>> {
        // Parse expression.
        let value = self.parse_expression(Precedence::Lowest);
        if value.is_none() {
            self.log_parse_error("Expression statement is missing a value expression.".to_string());
            return None;
        }

        // Check that line ends in a semicolon.
        if !self.advance_tokens_if_next_token_is(TokenType::SemiColon) {
            self.log_parse_error("Line does not end in a semicolon.".to_string());
            return None;
        }

        let to_return = ExpressionStatement {
            token: Token::new("(".to_string()),
            value: value.unwrap(),
        };
        return Some(Box::new(to_return));
    }

    fn parse_block_statement(&mut self) -> Option<Box<Node>> {
        let mut to_return = BlockStatement {
            token: Token::new("(".to_string()),
            statements: Vec::new(),
        };

        // Move over opening LBrace.
        self.advance_tokens();

        // Parse each statement in the block.
        while !self.token_is(TokenType::RBrace) {
            let statement = self.parse_statement();
            if !statement.is_none() {
                to_return.statements.push(statement.unwrap());
            }
            self.advance_tokens();
        }
        return Some(Box::new(to_return));
    }

    // ================================================================================
    // Functions for parsing expressions.
    // ================================================================================

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<Node>> {
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
            _ => None,
        };

        if left_expression.is_none() {
            self.log_parse_error("Expression doesn't have a valid token type.".to_string());
            return None;
        }

        // Parsing expresion using Pratt parsing.
        // https://en.wikipedia.org/wiki/Pratt_parser
        while !self.next_token_is(TokenType::SemiColon)
            && (precedence as u8) < (self.next_token_precedence() as u8)
        {
            if self.next_token_is(TokenType::LParen) {
                self.advance_tokens();
                left_expression = self.parse_call_expression(left_expression.unwrap());
            } else if self.next_token.is_operator() {
                self.advance_tokens();
                left_expression = self.parse_infix_expression(left_expression.unwrap());
            } else {
                return left_expression;
            }
        }

        return left_expression;
    }

    fn parse_identifier_expression(&mut self) -> Option<Box<Node>> {
        let to_return = IdentifierExpression {
            token: self.token.clone(),
        };
        return Some(Box::new(to_return));
    }

    fn parse_bool_expression(&mut self) -> Option<Box<Node>> {
        let to_return = BoolExpression {
            token: self.token.clone(),
            value: self.token_is(TokenType::True),
        };
        return Some(Box::new(to_return));
    }

    fn parse_integral_expression(&mut self) -> Option<Box<Node>> {
        // Covert literal into integral.
        let value_result = self.token.literal.parse::<i64>();
        if value_result.is_err() {
            self.log_parse_error("Integral value not of type int.".to_string());
            return None;
        }

        let to_return = IntegralExpression {
            token: self.token.clone(),
            value: value_result.unwrap(),
        };
        return Some(Box::new(to_return));
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<Node>> {
        // Record prefix operator.
        let token = self.token.clone();

        // Parse prefic expresion operand.
        self.advance_tokens();
        let right = self.parse_expression(Precedence::Prefix);
        if right.is_none() {
            self.log_parse_error("Prefix expression missing right operand".to_string());
            return None;
        }

        let to_return = PrefixExpression {
            token: token,
            right: right.unwrap(),
        };
        return Some(Box::new(to_return));
    }

    fn parse_infix_expression(&mut self, left: Box<Node>) -> Option<Box<Node>> {
        // Record infix operator and its precedence.
        let token = self.token.clone();
        let precedence = self.token_precedence();
        self.advance_tokens();

        // Parse right operand of operator.
        let right = self.parse_expression(precedence);
        if right.is_none() {
            self.log_parse_error("Prefix expression missing right operand".to_string());
            return None;
        }

        let to_return = InfixExpression {
            token: token,
            right: right.unwrap(),
            left: left,
        };
        return Some(Box::new(to_return));
    }

    fn parse_grouped_expression(&mut self) -> Option<Box<Node>> {
        // Mover over opening bracket.
        self.advance_tokens();

        let to_return = self.parse_expression(Precedence::Lowest);
        if !self.advance_tokens_if_next_token_is(TokenType::RParen) {
            self.log_parse_error("Grouped expression missing a closing parenthesis.".to_string());
            return None;
        }
        return to_return;
    }

    fn parse_ifelse_expression(&mut self) -> Option<Box<Node>> {
        // Move forward until the condition.
        if !self.advance_tokens_if_next_token_is(TokenType::LParen) {
            self.log_next_token_error(TokenType::LParen);
            return None;
        }
        self.advance_tokens();

        // Parse the condition.
        let condition = self.parse_expression(Precedence::Lowest);
        if condition.is_none() {
            self.log_parse_error("Invalid conditional.".to_string());
            return None;
        }

        // Move forward over closing paren and opening brace.
        if !self.advance_tokens_if_next_token_is(TokenType::RParen) {
            self.log_next_token_error(TokenType::RParen);
            return None;
        }
        if !self.advance_tokens_if_next_token_is(TokenType::LBrace) {
            self.log_next_token_error(TokenType::LBrace);
            return None;
        }

        // Parse 'true' block statement.
        let consequence = self.parse_block_statement();
        if consequence.is_none() {
            self.log_parse_error("If expression missing a consequence.".to_string());
            return None;
        }

        // Parse 'else' block statement.
        let mut alternative: Option<Box<Node>> = None;
        if self.next_token_is(TokenType::Else) {
            self.advance_tokens();
            if !self.advance_tokens_if_next_token_is(TokenType::LBrace) {
                self.log_next_token_error(TokenType::LBrace);
                return None;
            }
            alternative = self.parse_block_statement();
        }

        let to_return = IfElseExpression {
            token: Token::new("if".to_string()),
            condition: condition.unwrap(),
            consequence: consequence.unwrap(),
            alternative: alternative,
        };
        return Some(Box::new(to_return));
    }

    fn parse_func_expression(&mut self) -> Option<Box<Node>> {
        // Move over opening brace.
        if !self.advance_tokens_if_next_token_is(TokenType::LParen) {
            self.log_next_token_error(TokenType::LParen);
            return None;
        }
        self.advance_tokens();

        // Parse parameters.
        let mut parameters: Vec<Box<Node>> = Vec::new();
        while !self.token_is(TokenType::RParen) {
            parameters.push(self.parse_identifier_expression().unwrap());
            if self.next_token_is(TokenType::RParen) {
                self.advance_tokens();
                break;
            }
            if !self.advance_tokens_if_next_token_is(TokenType::Comma) {
                self.log_parse_error("Missing comma between function arguments".to_string());
                return None;
            }
            self.advance_tokens();
        }
        if !self.advance_tokens_if_next_token_is(TokenType::LBrace) {
            return None;
        }

        // Process body of function.
        let body = self.parse_block_statement();
        if body.is_none() {
            self.log_parse_error("Function missing a block statement".to_string());
            return None;
        }

        let to_return = FunctionExpression {
            token: Token::new("fn".to_string()),
            body: Rc::new(body.unwrap()),
            parameters: Rc::new(parameters),
        };
        return Some(Box::new(to_return));
    }

    fn parse_call_expression(&mut self, func: Box<Node>) -> Option<Box<Node>> {
        // Parse function name.
        let mut to_return = CallExpression {
            token: self.token.clone(),
            arguments: Vec::new(),
            func_identifier: func, // Func is ast::IdentifierExpression
        };
        // Move over opening paren for args.
        self.advance_tokens();

        // Check if function call has any arguments.
        if self.token_is(TokenType::RParen) {
            return Some(Box::new(to_return));
        }

        // Parse argruments to call.
        let mut expression = self.parse_expression(Precedence::Lowest);
        if expression.is_none() {
            self.log_parse_error("Error in call argument".to_string());
            return None;
        }
        to_return.arguments.push(expression.unwrap());

        // Continue if there are more arguments to parse.
        while self.next_token_is(TokenType::Comma) {
            self.advance_tokens();
            self.advance_tokens();

            expression = self.parse_expression(Precedence::Lowest);
            if expression.is_none() {
                self.log_parse_error("Error in parsing call argument".to_string());
                return None;
            }
            to_return.arguments.push(expression.unwrap());
        }

        if !self.advance_tokens_if_next_token_is(TokenType::RParen) {
            self.log_next_token_error(TokenType::RParen);
            return None;
        }

        return Some(Box::new(to_return));
    }
}
