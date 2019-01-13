
pub mod token;


pub struct Lexer {
	input: String,
	position: Option<usize>, // current position in input (points to current char) 
	read_position: usize, // current reading position in input (after current char) 
	pub val: Option<char>, // current char under examination
	length: usize
}


impl Lexer {
	pub fn new(input_string: &String) -> Self {
		Lexer{input: input_string.clone(), 
			  position: Some(0),
			  read_position: 1,
			  val: Some(input_string.chars().nth(0).unwrap()),
			  length: input_string.chars().count()
              }
	}

    fn peak_char(self: &mut Lexer) -> Option<char> {
        if self.read_position < self.length {
            return self.input.chars().nth(self.read_position);
        }
        return None;

    }

    fn read_char(self: &mut Lexer) -> Option<char> {
        if self.read_position < self.length {
            self.val = self.input.chars().nth(self.read_position);	
            self.position = Some(self.read_position);	
            self.read_position += 1;
        } else {
            self.val = None;
            self.position = None;
        }
        return self.val;
    }

     fn skip_whitespace(self: &mut Lexer) {
        if self.val.is_none() { return; }
        while self.val.unwrap() == ' '  || 
              self.val.unwrap() == '\t' || 
              self.val.unwrap() == '\n' || 
              self.val.unwrap() == '\r' {
                  self.read_char();
              }
    }

    fn read_literal(self: &mut Lexer) -> Option<String> {
	    // Skip forward to next non-whitespace char.
        self.skip_whitespace();
        
        // Return None if end of file has been reached. 
        if self.val.is_none() {
            return None;
        } 
        
        // Read current char into string.
        let mut to_return = self.val.unwrap().to_string();
        
        // Get non alphanumeric tokens.
        if !self.val.unwrap().is_alphanumeric() {

            // Check for two character == and != tokens.
            if self.val.unwrap() == '=' && self.peak_char().unwrap_or('0') == '=' {
                self.read_char(); 
                to_return.push(self.val.unwrap());
            }
            else if self.val.unwrap() == '!' && self.peak_char().unwrap_or('0') == '=' {
                self.read_char(); 
                to_return.push(self.val.unwrap());
            }
            self.read_char();
            return Some(to_return);

        }

        // Read multi-char alphanumeric tokens.
        // if read_char returns None use ! as default value which is not alphanumeric and 
        // therefore the loop won't execute. 
        while self.read_char().unwrap_or('!').is_alphanumeric() { 
            to_return.push(self.val.unwrap());
        }
        
        return Some(to_return);

    }

    pub fn next_token(self: &mut Lexer) -> Option<token::Token> {
        let literal = self.read_literal();
        if literal.is_none() {
            return None;
        }
	    return Some(token::Token::new(literal.unwrap()));

    }

}


impl Iterator for Lexer {
    type Item = token::Token;
    
    fn next(&mut self) -> Option<token::Token> {
        return self.next_token();
     }
 }
