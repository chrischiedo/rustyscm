use std::collections::HashMap;
use std::f64::consts::PI;

use crate::operator_utils::*;
use crate::parser::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    contents: HashMap<String, Expression>,
}

impl Environment {
    fn new() -> Self {
        Self {
            contents: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: String, v: Expression) {
        self.contents.insert(k, v);
    }

    pub fn get(&self, k: &str) -> Option<&Expression> {
        self.contents.get(k)
    }
}

// An environment with some Scheme standard procedures
pub fn standard_env() -> Environment {
    let mut environment = Environment::new();

    // Basic arithmetic operators
    environment.insert(
        "+".to_string(),
        Expression::Func(|args: &[Expression]| match add(args) {
            Ok(expr) => expr,
            Err(e) => panic!("{}", e),
        }),
    );
    environment.insert(
        "-".to_string(),
        Expression::Func(|args: &[Expression]| match subtract(args) {
            Ok(expr) => expr,
            Err(e) => panic!("{}", e),
        }),
    );
    environment.insert(
        "*".to_string(),
        Expression::Func(|args: &[Expression]| match multiply(args) {
            Ok(expr) => expr,
            Err(e) => panic!("{}", e),
        }),
    );
    environment.insert(
        "/".to_string(),
        Expression::Func(|args: &[Expression]| match divide(args) {
            Ok(expr) => expr,
            Err(e) => panic!("{}", e),
        }),
    );

    // Exponent
    environment.insert(
        "pow".to_string(),
        Expression::Func(|args: &[Expression]| match power(args) {
            Ok(expr) => expr,
            Err(e) => panic!("{}", e),
        }),
    );

    // Comparison operators
    environment.insert(
        "=".to_string(),
        Expression::Func(|args: &[Expression]| match compare(args, "=") {
            Ok(expr) => expr,
            Err(e) => panic!("{}", e),
        }),
    );

    environment.insert(
        ">".to_string(),
        Expression::Func(|args: &[Expression]| match compare(args, ">") {
            Ok(expr) => expr,
            Err(e) => panic!("{}", e),
        }),
    );

    environment.insert(
        "<".to_string(),
        Expression::Func(|args: &[Expression]| match compare(args, "<") {
            Ok(expr) => expr,
            Err(e) => panic!("{}", e),
        }),
    );

    environment.insert(
        ">=".to_string(),
        Expression::Func(|args: &[Expression]| match compare(args, ">=") {
            Ok(expr) => expr,
            Err(e) => panic!("{}", e),
        }),
    );

    environment.insert(
        "<=".to_string(),
        Expression::Func(|args: &[Expression]| match compare(args, "<=") {
            Ok(expr) => expr,
            Err(e) => panic!("{}", e),
        }),
    );

    // PI constant
    environment.insert("pi".to_string(), Expression::Number(PI));

    environment
}
