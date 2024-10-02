use std::fmt;

use crate::env::Environment;
use crate::lexer::{tokenize, Token};


#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Bool(bool),
    Number(f64),
    Symbol(String),
    List(Vec<Expression>),
    Func(fn(&[Expression]) -> Expression),
    Function(Procedure),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Procedure {
    pub params: Vec<Expression>,
    pub body: Vec<Expression>,
    pub env: Environment,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Bool(a) => write!(f, "{}", a),
            Expression::Number(n) => write!(f, "{}", n),
            Expression::Symbol(s) => write!(f, "{}", s),
            Expression::List(list) => {
                let formatted_list: Vec<String> =
                    list.iter().map(|exp| format!("{}", exp)).collect();
                write!(f, "({})", formatted_list.join(" "))
            }
            Expression::Func(_) => write!(f, "<function>"),
            Expression::Function(_) => write!(f, "<function>"),
        }
    }
}

fn parse_token_list(tokens: &mut Vec<Token>) -> Result<Expression, String> {
    let token = tokens.pop();

    if token != Some(Token::OpenParen) {
        return Err(format!("Expected OpenParen, found {:?}", token));
    }

    let mut list: Vec<Expression> = Vec::new();

    while !tokens.is_empty() {
        let token = tokens.pop();

        if token == None {
            return Err("Did not find enough tokens".to_string());
        }

        let tok = token.unwrap();

        match tok {
            Token::Number(n) => list.push(Expression::Number(n)),
            Token::Symbol(s) => list.push(Expression::Symbol(s)),
            Token::OpenParen => {
                tokens.push(Token::OpenParen);
                let sub_list = parse_token_list(tokens)?;
                list.push(sub_list);
            }
            Token::CloseParen => {
                return Ok(Expression::List(list));
            }
        }
    }

    Ok(Expression::List(list))
}

pub fn parse(input: &str) -> Result<Expression, String> {
    let token_result = match tokenize(input) {
        Ok(val) => val,
        Err(err) => return Err(format!("{}", err)),
    };

    let mut tokens = token_result.into_iter().rev().collect();

    parse_token_list(&mut tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse1() {
        let input = "(define r 10)";

        let actual_parsed_expr = parse(input).unwrap();

        let expected_expr = Expression::List(vec![
            Expression::Symbol("define".to_string()),
            Expression::Symbol("r".to_string()),
            Expression::Number(10.0),
        ]);

        assert_eq!(actual_parsed_expr, expected_expr);
    }

    #[test]
    fn test_parse2() {
        let input = "(
                         (define x 5)
                         (define y 10)
                         (* x y)
                       )";

        let actual_parsed_expr = parse(input).unwrap();

        let expected_expr = Expression::List(vec![
            Expression::List(vec![
                Expression::Symbol("define".to_string()),
                Expression::Symbol("x".to_string()),
                Expression::Number(5.0),
            ]),
            Expression::List(vec![
                Expression::Symbol("define".to_string()),
                Expression::Symbol("y".to_string()),
                Expression::Number(10.0),
            ]),
            Expression::List(vec![
                Expression::Symbol("*".to_string()),
                Expression::Symbol("x".to_string()),
                Expression::Symbol("y".to_string()),
            ]),
        ]);

        assert_eq!(actual_parsed_expr, expected_expr);
    }
}
