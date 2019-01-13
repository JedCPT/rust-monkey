
mod lexer;
mod parser;
use std::io;
use std::io::prelude::*;
#[macro_use] extern crate maplit;


fn print_prompt() {
	print!{">>>"};io::stdout().flush().unwrap();
}

fn main () {
	let stdin = io::stdin();
	// let code = "let x = 10;".to_string();
	// // let code = "
	// // if (x+2 < 10) {
	// // 	let x = 10;
	// // } else {
	// // 	let cat = (x * 9 + 99 > true)
	// // }".to_string();
	// let lexer: lexer::Lexer = lexer::Lexer::new(&code);
	
	// let mut parser = parser::Parser::new(lexer);
	
	// parser.parse_program();
	
	print_prompt();	
	
	// Main REPL Loop.
	let mut lexer: lexer::Lexer;
	let mut parser: parser::Parser;
    for line in stdin.lock().lines() {
		
		if line.as_ref().unwrap() == "" { break; }
		
		lexer = lexer::Lexer::new(&line.as_ref().unwrap());
		parser = parser::Parser::new(lexer);
		parser.parse_program();
		parser.print_parse_errors();
		
		// for token in lexer {
		// 	println!("{}",token);
		// }

		print_prompt();

    }
	// let var: String = "
	// 	let five = 5;
	// 	let ten = 10;
   	// 	let add = fn(x, y) {
    // 		x + y;
	// 	};
   	// 	let result = add(five, ten);
    // 	!-/*5;
    // 	5 < 10 > 5;
   	// 	if (5 < 10) {
   	// 		return true;
   	// 	} else {
    //    		return false;
	// 	}
	// 	10 == 10; 10 != 9;
	// ".to_string();

	// let lexer: lexer::Lexer = lexer::Lexer::new(&var);
	// for c in lexer {
	// 	println!("{}",c);
	// }
		
}
