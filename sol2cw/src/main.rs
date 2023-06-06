use sol2cw_lexer::Lexer;

pub fn main() {
    // read first argument as path
    let input_path = std::env::args().nth(1).expect("no input file given");

    // read file
    let input = std::fs::read_to_string(input_path).expect("failed to read input file");

    let lexer = Lexer::new(&input);

    println!("{:?}", lexer.to_parsed());
}
