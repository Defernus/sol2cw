use logos::Logos;
use sol2cw_lexer::Token;

pub fn main() {
    // read first argument as path
    let input_path = std::env::args().nth(1).expect("no input file given");

    // read file
    let input = std::fs::read_to_string(input_path).expect("failed to read input file");

    let lexer = Token::lexer(&input);

    println!("{:?}", lexer.collect::<Vec<_>>());
}
