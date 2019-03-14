mod ast;
mod enviroment;
mod evaluator;
mod lexer;
mod object;
mod parser;
use std::cell::RefCell;
use std::io;
use std::io::prelude::*;
use std::rc::Rc;
use std::env;
// use parser::Program;
#[macro_use]
extern crate maplit;
// #[macro_use]
// extern crate downcast;
#[macro_use]
extern crate downcast_rs;
// use downcast_rs::Downcast;

// fn eval(node: Box<ast::Node>) -> Option<Box<object::Object>> {
// 	// match node.get_type() {
// 		ast::NodeType::IntegralExpression => ,
// 		_ => None
// 	// }

// }

use std::collections::HashMap;

// fn eval(program: &mut parser::Program, env: Rc<RefCell<enviroment::Enviroment>>) {
//     let mut result: Rc<object::Object>;

//     for statement in program.statements.iter().by_ref() {
//         println!("{}", statement.to_string());

//         result = statement.eval(env.clone());

//         println!("{}", result.to_string());
//     }
// }

fn print_prompt() {
    print! {">>>"};
    io::stdout().flush().unwrap();
}

fn exec_file(file_sname: String) {

}

fn main() {
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
    // let args: Vec<String> = env::args().collect();

    print_prompt();

    // Main REPL Loop.
    let mut lexer: lexer::Lexer;
    let mut parser: parser::Parser;
    // let mut enviroment = enviroment::Enviroment::new(None);
    // let env = Rc::new(RefCell::new(enviroment));
    for line in stdin.lock().lines() {
        if line.as_ref().unwrap() == "" {
            break;
        }

        lexer = lexer::Lexer::new(&line.as_ref().unwrap());
        parser = parser::Parser::new(lexer);
        let program = parser.parse_program(true);
        // parser.print_parse_errors();
        // eval(&mut program.unwrap(), env.clone());

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
