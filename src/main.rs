mod lexer;
mod parser;
mod ast;
mod interpreter;



use std::io::{self,Write};
use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new();

    interpreter.set_variable("x".to_string(), 10.0);
    interpreter.set_variable("y".to_string(), 20.0);
    interpreter.set_variable("z".to_string(), 30.0);
    
    loop {
        print!("Enter expression (or type 'exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let ast = parser.parse();

        let result = interpreter.interpret(ast);

        println!("Result: {}", result);
    }
}
