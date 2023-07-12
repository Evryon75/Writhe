use colour::*;
use writhe::Token;

pub fn lex(input: Vec<String>) {
    let mut tokens: Vec<Token> = vec![];
    for w in input {
        println!("{}", w);
        if let Some(token) = tokenize(w) {
            tokens.push(token);
        }
    }
}

fn tokenize(word: String) -> Option<Token> {
    None
}

fn thorough_tokenize(seq: String) {}
