/*
Author: Jedda Boyle
Contains: Lexer
The lexer splits strings of code into tokens.
*/

pub mod token;
use token::Token;
use token::TokenType;

pub struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    val: char,            // current char under examination
    eof: bool,
}

impl Lexer {
    pub fn new(input_string: &String) -> Self {
        if input_string.len() == 0 {
            println!("Error: Lexer input string has zero length.");
        }
        Lexer {
            input: input_string.clone(),
            position: 0,
            read_position: 1,
            val: input_string.chars().nth(0).unwrap(),
            eof: false,
        }
    }

    // Return next char to be read.
    fn peak_char(self: &mut Lexer) -> Option<char> {
        return self.input.chars().nth(self.read_position);
    }

    fn read_char(self: &mut Lexer) -> Option<char> {
        let val = self.input.chars().nth(self.read_position);
        if val.is_some() {
            self.val = val.unwrap();
            self.position = self.read_position;
            self.read_position += 1;
        } else {
            self.eof = true;
        }
        return val;
    }

    // Move lexer forward until current char is not a whitespace character.
    fn skip_whitespace(self: &mut Lexer) {
        while (self.val == ' ' || self.val == '\t' || self.val == '\n' || self.val == '\r')
            && !self.eof
        {
            self.read_char();
        }
    }

    fn read_literal(self: &mut Lexer) -> Option<String> {
        // Skip forward to next non-whitespace char.
        self.skip_whitespace();

        // Return None if end of file has been reached.
        if self.eof {
            return None;
        }

        // Read current char into string.
        let mut to_return = self.val.to_string();

        // Get non alphanumeric tokens.
        if !self.val.is_alphanumeric() {
            // Check for two character == and != tokens.
            if self.val == '=' && self.peak_char().unwrap_or('0') == '=' {
                self.read_char();
                to_return.push(self.val);
            } else if self.val == '!' && self.peak_char().unwrap_or('0') == '=' {
                self.read_char();
                to_return.push(self.val);
            }
            self.read_char();
            return Some(to_return);
        }

        // Read multi-char alphanumeric tokens.
        // if read_char returns None use ! as default value which is not alphanumeric so
        // loop isn't executed.
        while self.read_char().unwrap_or('!').is_alphanumeric() {
            to_return.push(self.val);
        }

        return Some(to_return);
    }

    pub fn next_token(self: &mut Lexer) -> Option<Token> {
        let literal = self.read_literal();
        if literal.is_some() {
            return Some(Token::new(literal.unwrap()));
        } else {
            return Some(Token::new("EOF".to_string()));
        }
    }
}

// TODO remove this...
// impl Iterator for Lexer {
//     type Item = Token;

//     fn next(&mut self) -> Option<Token> {
//         let to_return = self.next_token();
//         if to_return.is_none() {
//             return None;
//         }
//         if to_return.as_ref().unwrap().token_type == TokenType::Eof {
//             return None;
//         }
//         return to_return;
//     }
// }
