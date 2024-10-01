// mod lexer;

// use std::io;

// use crate::lexer::tokenize;

fn main() {
    rustyscm::repl();
}

// fn main() {
//     let mut input = String::new();
//
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");
//
//     let tokens = tokenize(&*input);
//
//     println!("Tokens: {:?}", tokens);
// }
