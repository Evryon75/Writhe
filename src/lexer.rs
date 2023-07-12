use colour::*;

pub fn lex(input: Vec<String>) {
    magenta_ln!("{:#?}", input);
}

fn tokenize(word: String) {

}

fn advanced(seq: String) {

}

enum Token {
    Identifier(String),
    Fn,
    Let,
    Use,
    Mut,
    For,
    In,
    Match,
    As,
    Enum,
    Colon,
    Semicolon,
    RoundLeft,
    RoundRight,
    SquareLeft,
    SquareRight,
    CurlyLeft,
    CurlyRight,
}
