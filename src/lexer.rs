use aho_corasick::AhoCorasick;
use colour::*;
use std::ops::Index;
use writhe::Token;

pub fn lex(input_raw: String) {
    let cleaned: String = AhoCorasick::new(&[
        "=", "(", ")", "[", "]", "{", "}", "\r\n", "!", "+", "-", ">", "<", "&&", "||", "%", ":",
        "/", "*",
    ])
    .unwrap()
    .replace_all(
        input_raw.as_str(),
        &[
            " = ", " ( ", " ) ", " [ ", " ] ", " { ", " } ", " \r\n ", "", " + ", " - ", " > ",
            " < ", " && ", " || ", " % ", " : ", " / ", " * ",
        ],
    );
    let lines: Vec<&str> = cleaned.lines().collect();

    let mut tokens: Vec<Token> = vec![];

    let mut line_count = 0;
    for line in lines {
        line_count += 1;

        let mut comment = false;
        if !(line.trim().starts_with("/  /") || comment) {
            let mut stringing = false;
            let mut string = String::new();

            let words: Vec<&str> = line.split_whitespace().collect();
            let mut w = 0;
            while w < words.len() {
                let word = words[w];
                if word.eq("/") && words[w + 1].eq("*") && !comment && !stringing {
                    comment = true;
                }
                if !comment {
                    if word.starts_with("\"") {
                        stringing = true;
                    }
                    if !stringing {
                        if let Some(tok) = tokenize(word) {
                            tokens.push(tok);
                        }
                    } else {
                        //todo: tidy this part up please for the love of god like
                        // im talking all around this comment not just this thing below
                        // and find out why i need that break there like its bugging me man
                        // its bugging me fr fr no cap god damnit its so late
                        let mut found = false;
                        let raw_lines: Vec<&str> = input_raw.lines().collect();
                        for i in raw_lines[line_count - 1].chars() {
                            if i.eq(&'"') { //todo: find a way to put chars in aswell
                                found = !found;
                            }
                            if found {
                                string.push(i);
                            }
                            if !found && !string.is_empty() {
                                break;
                            }
                        }
                    }
                    if word.ends_with("\"") {
                        tokens.push(Token::StringLiteral(string.clone().replace("\"", "")));
                        stringing = false;
                        string.clear();
                    }
                }
                if word.eq("/") && words[w - 1].eq("*") && comment && !stringing {
                    comment = false;
                }
                w += 1;
            }
        }
    }

    magenta_ln!("{:#?}", tokens);
}

fn tokenize(word: &str) -> Option<Token> {
    Some(match word {
        //todo: add checks for +-*/ aswell
        "trait" => Token::Trait,
        "struct" => Token::Struct,
        "impl" => Token::Impl,
        "false" | "true" => Token::BooleanLiteral(if word.eq("true") { true } else { false }),
        "fn" => Token::Fn,
        "let" => Token::Let,
        "use" => Token::Use,
        "mut" => Token::Mut,
        "for" => Token::For,
        "in" => Token::In,
        "match" => Token::Match,
        "as" => Token::As,
        "enum" => Token::Enum,
        ":" => Token::Colon,
        ";" => Token::Semicolon,
        "(" => Token::RoundLeft,
        ")" => Token::RoundRight,
        "[" => Token::SquareLeft,
        "]" => Token::SquareRight,
        "{" => Token::CurlyLeft,
        "}" => Token::CurlyRight,
        "=" => Token::Equals,
        "==" => Token::DoubleEquals,
        ">" => Token::Greater,
        "<" => Token::Lesser,
        "&&" => Token::And,
        "||" => Token::Or,
        _ => thorough_tokenize(word),
    })
}

fn thorough_tokenize(seq: &str) -> Token {
    // Probably useless function, i dont think im gonna use it or use it for the identifiers and numeric literals
    Token::Undefined
}
