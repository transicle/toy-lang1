mod lexer;
use lexer::Lexer;

fn main() {
    let source = "let x = 15 + 2";
    let mut lexer = Lexer::new(source);

    let tokens = lexer.lex();
    for token in &tokens {
        println!("{:#?}", token);
    }
}
