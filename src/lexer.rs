use colour::*;
use writhe::Token;

pub fn lex(input_raw: String) {
    //dont add spaces in between stuff and remake everything again god fucking damnit bro
    let mut tokens: Vec<Token> = vec![];
    let mut state = LexerState::Free;
    for line in input_raw.lines().filter(|s| !s.trim().starts_with("//")) {

        let chars: Vec<char> = line.chars().collect();
        const IDENT_CHARS: &str = "_qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";
        const IDENT_DIGITS: &str = "1234567890";
        let mut build = String::new();
        let mut i = 0;
        while i < chars.len() {
            let char = chars[i];

            if let Some(tok) = match state {
                LexerState::String => {
                    let result = None;

                    if chars[i] == '\"' {
                        state = LexerState::Free;
                    }

                    result
                }
                LexerState::Comment => {
                    if chars[i] == '/' && chars[i - 1] == '*' {
                        state = LexerState::Free;
                    }
                    None
                }
                LexerState::Ident => {
                    let result = None;
                    result
                }
                LexerState::Free => {
                    let mut result: Option<Token> = None;
                    if chars[i] == '/' && chars[i + 1] == '*' {
                        state = LexerState::Comment;
                    } else if chars[i] == '\"' {
                        state = LexerState::String;
                    } else if IDENT_CHARS.contains(char) {
                        state = LexerState::Ident;
                        build.push(char);
                    } else {
                        result = tokenize(char.to_string().as_str());
                    }
                    result
                }
            } {
                tokens.push(tok);
            }

            white_ln!("{}", char);
            i += 1;
        }
    }

    magenta_ln!("{:#?}", tokens);
}

fn tokenize(word: &str) -> Option<Token> {
    match word {
        "+" => Some(Token::Add),
        "-" => Some(Token::Subtract),
        "*" => Some(Token::Multiply),
        "/" => Some(Token::Divide),
        "," => Some(Token::Comma),
        "trait" => Some(Token::Trait),
        "struct" => Some(Token::Struct),
        "impl" => Some(Token::Impl),
        "false" | "true" => Some(Token::BooleanLiteral(if word.eq("true") {
            true
        } else {
            false
        })),
        "fn" => Some(Token::Fn),
        "let" => Some(Token::Let),
        "use" => Some(Token::Use),
        "mut" => None,
        "for" => Some(Token::For),
        "in" => Some(Token::In),
        "match" => Some(Token::Match),
        "as" => Some(Token::As),
        "enum" => Some(Token::Enum),
        ":" => Some(Token::Colon),
        ";" => Some(Token::Semicolon),
        "(" => Some(Token::RoundLeft),
        ")" => Some(Token::RoundRight),
        "[" => Some(Token::SquareLeft),
        "]" => Some(Token::SquareRight),
        "{" => Some(Token::CurlyLeft),
        "}" => Some(Token::CurlyRight),
        "=" => Some(Token::Equals),
        ">" => Some(Token::Greater),
        "<" => Some(Token::Lesser),
        "&&" => Some(Token::And),
        "||" => Some(Token::Or),
        _ => thorough_tokenize(word),
    }
}

fn thorough_tokenize(seq: &str) -> Option<Token> {
    Some(Token::Undefined)
}

enum LexerState {
    String,
    Comment,
    Ident,
    Free,
}
