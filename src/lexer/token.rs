use std::fmt;
// use std::collections::HashMap;


#[derive(Copy, Clone, PartialEq, Hash, Eq)]
pub enum TokenType {
    Eof,
	Illegal,

    // Identifiers + literals
	Int,
	Ident,

    // Operators
	Assign,
	Plus,
    Minus,
    Asterisk,
    Bang,
    Slash,
    Lt,
    Gt,
    Equal,
    NotEqual,

    // Delimiters
	Comma,
	SemiColon,

    // Brackets
	LParen,
	RParen,
	LBrace,
	RBrace,
	
    // Keywords
    Let,
	Function,
    True,
    False,
    If,
    Else,
    Return

}

#[derive(Copy, Clone)]
pub enum Precedence {
    Lowest = 0,
    Equals = 1,
    LessGreater = 2,
    Sum = 3,
    Product = 4,
    Prefix = 5,
    Call = 6
}

#[derive(Clone)]
pub struct Token {
	pub token_type: TokenType,
	pub literal: String,
}




fn is_int(literal: &String) -> bool {
    return literal.chars().all(|x| x.is_digit(10));

}


fn is_identifier(literal: &String) -> bool {
    return literal.chars().all(|x| x.is_ascii_alphabetic());

}



impl Token {


    pub fn new(literal: String) -> Self {
        let mut token_type = TokenType::Illegal;

        // Operators.
        if      literal ==  "=" { token_type = TokenType::Assign; }
        else if literal ==  "+" { token_type = TokenType::Plus; }
        else if literal ==  "-" { token_type = TokenType::Minus; }
        else if literal ==  "!" { token_type = TokenType::Bang; }
        else if literal ==  "*" { token_type = TokenType::Asterisk; }
        else if literal ==  "/" { token_type = TokenType::Slash; }
        else if literal ==  "<" { token_type = TokenType::Lt; }
        else if literal ==  ">" { token_type = TokenType::Gt; }
        else if literal ==  "==" { token_type = TokenType::Equal; }
        else if literal ==  "!=" { token_type = TokenType::NotEqual; }
    
        // Delimeters
        else if literal ==  "," { token_type = TokenType::Comma; }
        else if literal ==  ";" { token_type = TokenType::SemiColon; }
        
        // Brackets
        else if literal ==  "(" { token_type = TokenType::LParen; }
        else if literal ==  ")" { token_type = TokenType::RParen; }
        else if literal ==  "{" { token_type = TokenType::LBrace; }
        else if literal ==  "}" { token_type = TokenType::RBrace; }

        // Keywords
        else if literal ==  "let" { token_type = TokenType::Let; }
        else if literal ==  "fn" { token_type = TokenType::Function; }
        else if literal ==  "if" { token_type = TokenType::If; }
        else if literal ==  "else" { token_type = TokenType::Else; }
        else if literal ==  "return" { token_type = TokenType::Return; }
        else if literal ==  "true" { token_type = TokenType::True; }
        else if literal ==  "false" { token_type = TokenType::False; }
        
        
        // Identifiers + literals
        else if literal == "EOF" { token_type = TokenType::Eof; }
        else if is_int(&literal) { token_type = TokenType::Int; }
        else if is_identifier(&literal) { token_type = TokenType::Ident; }
        
        
        return Token{token_type: token_type, literal: literal};
    }

    pub fn is_operator(&self) -> bool {
        return self.token_type == TokenType::Plus ||
               self.token_type == TokenType::Minus ||
               self.token_type == TokenType::Asterisk ||
               self.token_type == TokenType::Slash ||
               self.token_type == TokenType::Lt ||
               self.token_type == TokenType::Gt ||
               self.token_type == TokenType::Equal ||
               self.token_type == TokenType::NotEqual
    }



    pub fn get_precedence(&self) -> Precedence {
        let map = hashmap!{
            TokenType::Equal     => Precedence::Equals,
            TokenType::NotEqual  => Precedence::Equals,
            TokenType::Gt        => Precedence::LessGreater,
            TokenType::Lt        => Precedence::LessGreater,
            TokenType::Plus      => Precedence::Sum,
            TokenType::Minus     => Precedence::Sum,
            TokenType::Slash     => Precedence::Product,
            TokenType::Asterisk  => Precedence::Product,
            TokenType::LParen    => Precedence::Call
        };
        let result = map.get(&self.token_type);
        if result.is_none() {
            return Precedence::Lowest;
        } 
        return result.unwrap().clone();

        // return None;
        // return Some(.clone());
    }
}


impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        
        // Operators.
        // if      self.token_type == TokenType::Assign    { return write!(fmt,"Assign({})", self.literal)).unwrap(); }
        if      self.token_type == TokenType::Assign    { return write!(fmt, "="); }
        else if self.token_type == TokenType::Plus      { return write!(fmt, "+"); }
        else if self.token_type == TokenType::Minus     { return write!(fmt, "-"); }
        else if self.token_type == TokenType::Bang      { return write!(fmt, "!"); }
        else if self.token_type == TokenType::Asterisk  { return write!(fmt, "*"); }
        else if self.token_type == TokenType::Slash     { return write!(fmt, "/"); }
        else if self.token_type == TokenType::Lt        { return write!(fmt, "<"); }
        else if self.token_type == TokenType::Gt        { return write!(fmt, ">"); }
        else if self.token_type == TokenType::Equal     { return write!(fmt, "=="); }
        else if self.token_type == TokenType::NotEqual  { return write!(fmt, "!="); }
        
        // Delimeters
        else if self.token_type == TokenType::SemiColon { return write!(fmt, ";"); }
        else if self.token_type == TokenType::Comma     { return write!(fmt, ","); }

        // Brackets
        else if self.token_type == TokenType::LParen    { return write!(fmt, "("); }
        else if self.token_type == TokenType::RParen    { return write!(fmt, ")"); }
        else if self.token_type == TokenType::LBrace    { return write!(fmt, "{{"); }
        else if self.token_type == TokenType::RBrace    { return write!(fmt, "}}"); }

        // Keywords
        else if self.token_type == TokenType::Let       { return write!(fmt,"Let"); }
        else if self.token_type == TokenType::Function  { return write!(fmt,"Func"); }
        else if self.token_type == TokenType::If        { return write!(fmt,"If"); }
        else if self.token_type == TokenType::Else      { return write!(fmt,"Else"); }
        else if self.token_type == TokenType::Return    { return write!(fmt,"Return"); }
        else if self.token_type == TokenType::True      { return write!(fmt,"True"); } 
        else if self.token_type == TokenType::False     { return write!(fmt,"False"); } 
        
        // Identifiers + literals
        else if self.token_type == TokenType::Int       { return write!(fmt,"{}", self.literal); }
        else if self.token_type == TokenType::Ident     { return write!(fmt,"{}", self.literal); }
        else if self.token_type == TokenType::Eof       { return write!(fmt, "Eof"); }
        else { return write!(fmt,"Illegal({})", self.literal); }
        // Ok(())
    }
}
