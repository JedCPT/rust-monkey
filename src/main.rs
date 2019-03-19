/*
Author: Jedda Boyle
Contains: main function for rust-monkey.
File reads and evaluates the input.
*/

// ================================================================================
// Imports
// ================================================================================

// interpreter components.
mod ast;
mod enviroment;
mod lexer;
mod object;
mod parser;

use std::cell::RefCell;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::BufRead;
use std::io::prelude::Write;
use std::rc::Rc;

// Used for constant hash-map in lexer/token.rs
#[macro_use]
extern crate maplit;
// Used to downcast traits to actual type in object/mod.rs and ast/mod.rs
#[macro_use]
extern crate downcast_rs;

const DEBUG: bool = false;

fn eval(program: &mut parser::Program, env: Rc<RefCell<enviroment::Enviroment>>) {
    let mut result: Rc<object::Object>;

    for statement in program.statements.iter().by_ref() {
        result = statement.eval(env.clone());
        if result.get_type() != object::ObjectType::Null || DEBUG == true {
            println!("{}", result.to_string());
        }
    }
}

fn print_prompt() {
    print! {">>>"};
    io::stdout().flush().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Setup interpreter components.
    let mut lexer: lexer::Lexer;
    let mut parser: parser::Parser;
    let mut program: Option<parser::Program>;
    let enviroment = Rc::new(RefCell::new(enviroment::Enviroment::new(None)));

    // If there is no input file do REPL Loop.
    if args.len() == 1 {
        print_prompt();
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if line.as_ref().unwrap() == "" {
                break;
            }

            lexer = lexer::Lexer::new(&line.as_ref().unwrap());
            parser = parser::Parser::new(lexer);
            program = parser.parse_program(DEBUG);
            eval(&mut program.unwrap(), enviroment.clone());

            print_prompt();
        }
    }
    // Evaluate file.
    else if args.len() == 2 {
        let input = fs::read_to_string(args[1].clone());
        if input.is_ok() {
            let input_string = input.ok();
            lexer = lexer::Lexer::new(&input_string.as_ref().unwrap());
            parser = parser::Parser::new(lexer);
            program = parser.parse_program(DEBUG);
            eval(&mut program.unwrap(), enviroment.clone());
        } else {
            println!("{:?}", input);
        }
    } else {
        println!("Invalid number of command line arguments.");
    }
}
