use colour::*;
use writhe::{panic_confirm, Token};

const IDENT_CHARS: &str = "_qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";
const IDENT_DIGITS: &str = "1234567890";
const DIGITS: &str = "1234567890._";

enum LexerState {
    Number,
    String,
    Comment,
    Ident,
    Free,
}
//todo: add iterator stuff
pub fn lex(input_raw: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut state = LexerState::Free;
    let mut line_count = 0;
    for line in input_raw.lines() {
        line_count += 1;
        if !line.trim().starts_with("//") {
            let chars: Vec<char> = line.chars().collect();
            let mut build = String::new();
            let mut dot = false;

            let mut i = 0;
            while i < chars.len() {
                let char = chars[i];

                if let Some(tok) = match state {
                    LexerState::Number => {
                        if !DIGITS.contains(chars[i]) {
                            state = LexerState::Free;
                            dot = false;
                            i -= 1;
                            Some(Token::NumericLiteral(
                                build.replace("_", "").parse::<f64>().unwrap(),
                            ))
                        } else {
                            build.push(char);
                            if chars[i] == '.' {
                                if dot {
                                    panic_confirm(format!("[LEXER ERROR] Numeric literals must not contain more than one dot [.]\nAt line {}: {} !", line_count, build).as_str())
                                } else {
                                    dot = true;
                                }
                            }
                            None
                        }
                    }
                    LexerState::String => {
                        if chars[i] == '\"' {
                            state = LexerState::Free;
                            Some(Token::StringLiteral(build.to_string()))
                        } else {
                            build.push(char);
                            None
                        }
                    }
                    LexerState::Comment => {
                        if chars[i] == '/' && chars[i - 1] == '*' {
                            state = LexerState::Free;
                        }
                        None
                    }
                    LexerState::Ident => {
                        if !IDENT_CHARS.contains(chars[i]) && !IDENT_DIGITS.contains(chars[i]) {
                            state = LexerState::Free;

                            i -= 1;

                            tokenize(&build.to_string())
                        } else {
                            build.push(char);
                            None
                        }
                    }
                    LexerState::Free => {
                        let mut result: Option<Token> = None;

                        build.clear();

                        if chars[i] == '/' && chars[i + 1] == '*' {
                            state = LexerState::Comment;
                        } else if chars[i] == '\"' {
                            state = LexerState::String;
                        } else if IDENT_CHARS.contains(char) {
                            state = LexerState::Ident;
                            build.push(char);
                        } else if IDENT_DIGITS.contains(chars[i]) {
                            state = LexerState::Number;
                            build.push(char);
                        } else {
                            result = tokenize(char.to_string().as_str());
                        };
                        if i != chars.len() - 1 {
                            //todo: try to use a match with fmt for this
                            result = if chars[i] == '&' && chars[i + 1] == '&' {
                                i += 1;
                                Some(Token::Operator(String::from("&&")))
                            } else if chars[i] == '|' && chars[i + 1] == '|' {
                                i += 1;
                                Some(Token::Operator(String::from("||")))
                            } else if chars[i] == '<' && chars[i + 1] == '=' {
                                i += 1;
                                Some(Token::Operator(String::from("<=")))
                            } else if chars[i] == '>' && chars[i + 1] == '=' {
                                i += 1;
                                Some(Token::Operator(String::from(">=")))
                            } else if chars[i] == '=' && chars[i + 1] == '=' {
                                i += 1;
                                Some(Token::Operator(String::from("==")))
                            } else if chars[i] == ':' && chars[i + 1] == ':' {
                                i += 1;
                                Some(Token::DoubleColon)
                            } else {
                                result
                            };
                        }
                        result
                    }
                } {
                    tokens.push(tok);
                }
                i += 1;
            }
        }
    }
    tokens
}

fn tokenize(word: &str) -> Option<Token> {
    match word {
        "+" => Some(Token::Operator(String::from("+"))),
        "-" => Some(Token::Operator(String::from("-"))),
        "*" => Some(Token::Operator(String::from("*"))),
        "/" => Some(Token::Operator(String::from("/"))),
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
        ">" => Some(Token::Operator(String::from(">"))),
        "<" => Some(Token::Operator(String::from("<"))),
        _ => {
            if word != " " {
                Some(Token::Identifier(word.to_string()))
            } else {
                None
            }
        }
    }
}
