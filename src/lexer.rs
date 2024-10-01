use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Number(f64),
    Symbol(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::Number(n) => write!(f, "{}", n),
            Token::Symbol(s) => write!(f, "{}", s),
        }
    }
}

pub fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let expr = expr
                        .replace("(", " ( ")
                        .replace(")", " ) ");

    let words = expr.split_whitespace();

    let mut tokens: Vec<Token> = Vec::new();

    for word in words {
        match word {
            "(" => tokens.push(Token::OpenParen),
            ")" => tokens.push(Token::CloseParen),
            _ => {
                let i = word.parse::<f64>();
                if i.is_ok() {
                    tokens.push(Token::Number(i.unwrap()));
                } else {
                    tokens.push(Token::Symbol(word.to_string()));
                }
            }
        }
    }

    Ok(tokens)
}
