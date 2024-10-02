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
    let expr = expr.replace("(", " ( ").replace(")", " ) ");

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize1() {
        let input = "(define r 10)";

        let actual_tokens = tokenize(input).unwrap_or(vec![]);

        let expected_tokens = vec![
            Token::OpenParen,
            Token::Symbol("define".to_string()),
            Token::Symbol("r".to_string()),
            Token::Number(10.0),
            Token::CloseParen,
        ];

        assert_eq!(actual_tokens, expected_tokens);
    }

    #[test]
    fn test_tokenize2() {
        let input = "
            (
                (define x 5)
                (define y 10)
                (* x y)
            )
        ";

        let actual_tokens = tokenize(input).unwrap_or(vec![]);

        let expected_tokens = vec![
            Token::OpenParen,
            Token::OpenParen,
            Token::Symbol("define".to_string()),
            Token::Symbol("x".to_string()),
            Token::Number(5.0),
            Token::CloseParen,
            Token::OpenParen,
            Token::Symbol("define".to_string()),
            Token::Symbol("y".to_string()),
            Token::Number(10.0),
            Token::CloseParen,
            Token::OpenParen,
            Token::Symbol("*".to_string()),
            Token::Symbol("x".to_string()),
            Token::Symbol("y".to_string()),
            Token::CloseParen,
            Token::CloseParen,
        ];

        assert_eq!(actual_tokens, expected_tokens);
    }
}
