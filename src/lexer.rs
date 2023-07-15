use colour::*;
use writhe::Token;

pub fn lex(input_raw: String) {
    let mut tokens: Vec<Token> = vec![];
    let mut state = LexerState::Free;
    let mut prev_state = LexerState::Free;
    for line in input_raw.lines().filter(|s| !s.trim().starts_with("//")) {
        let chars: Vec<char> = line.chars().collect();

        const IDENT_CHARS: &str = "_qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";
        const IDENT_DIGITS: &str = "1234567890";
        const DIGITS: &str = "1234567890._";

        let mut build = String::new();
        let mut i = 0;
        while i < chars.len() {
            let char = chars[i];

            if let Some(tok) = match state {
                LexerState::Number => {
                    let result = None;

                    if !DIGITS.contains(chars[i + 1]) {
                        state = LexerState::Free;
                        prev_state = LexerState::Number;
                    }

                    result
                }
                LexerState::String => {
                    let result = None;

                    if chars[i] == '\"' {
                        state = LexerState::Free;
                        prev_state = LexerState::String;
                    }

                    result
                }
                LexerState::Comment => {
                    if chars[i] == '/' && chars[i - 1] == '*' {
                        state = LexerState::Free;
                        prev_state = LexerState::Comment;
                    }
                    None
                }
                LexerState::Ident => {
                    let result = None;

                    if !IDENT_CHARS.contains(chars[i + 1]) && !IDENT_DIGITS.contains(chars[i + 1]) {
                        state = LexerState::Free;
                        prev_state = LexerState::Ident;
                    }

                    build.push(char);

                    result
                }
                LexerState::Free => {
                    let mut result: Option<Token> = None;

                    if !build.is_empty() {
                        result = Some(match prev_state {
                            LexerState::Number => Token::NumericLiteral(build.parse::<usize>().unwrap()),
                            LexerState::String => Token::StringLiteral(build.to_string()),
                            LexerState::Ident => Token::Identifier(build.to_string()),
                            LexerState::Comment => unreachable!(),
                            LexerState::Free => unreachable!()
                        })
                    }

                    if chars[i] == '/' && chars[i + 1] == '*' {
                        state = LexerState::Comment;
                        prev_state = LexerState::Free;
                    } else if chars[i] == '\"' {
                        state = LexerState::String;
                        prev_state = LexerState::Free;
                    } else if IDENT_CHARS.contains(char) {
                        state = LexerState::Ident;
                        prev_state = LexerState::Free;

                        build.push(char);
                    } else if IDENT_DIGITS.contains(chars[i]) {
                        state = LexerState::Number;
                        prev_state = LexerState::Free;
                        build.push(char);
                    } else {
                        result = tokenize(char.to_string().as_str());
                    }
                    result
                }
            } {
                tokens.push(tok);
            }

            white_ln!("{}, {:?}", char, state);
            build.clear();
            i += 1;
        }
    }

    magenta_ln!("{:#?}", tokens);
}

fn tokenize(word: &str) -> Option<Token> {
    green_ln!("{:#?}", word);
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
        "!" => None,
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

#[derive(Debug)]
enum LexerState {
    Number,
    String,
    Comment,
    Ident,
    Free,
}
