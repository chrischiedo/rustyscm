use rustyscm::env::standard_env;
use rustyscm::eval::eval;
use rustyscm::parser::Expression;

use std::f64::consts::PI;

#[test]
fn test_eval_add_two_numbers() {
    let mut env = standard_env();

    let input = "(+ 2 2)";

    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Number(4.0));
}

#[test]
fn test_eval_add_three_numbers() {
    let mut env = standard_env();

    let input = "(+ 1 2 3)";

    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Number(6.0));
}

#[test]
fn test_eval_subtract_two_numbers() {
    let mut env = standard_env();

    let input = "(- 10 5)";

    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Number(5.0));
}

#[test]
fn test_eval_subtract_three_numbers() {
    let mut env = standard_env();

    let input = "(- 10 2 3)";

    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Number(5.0));
}

#[test]
fn test_eval_multiply_two_numbers() {
    let mut env = standard_env();

    let input = "(* 3 2)";

    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Number(6.0));
}

#[test]
fn test_eval_multiply_three_numbers() {
    let mut env = standard_env();

    let input = "(* 1 2 3)";

    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Number(6.0));
}

#[test]
fn test_eval_divide_two_numbers() {
    let mut env = standard_env();

    let input = "(/ 24 6)";

    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Number(4.0));
}

#[test]
fn test_eval_divide_three_numbers() {
    let mut env = standard_env();

    let input = "(/ 24 6 2)";

    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Number(2.0));
}

#[test]
fn test_eval_power() {
    let mut env = standard_env();

    let input = "(pow 2 16)";

    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Number(65536.0));
}

#[test]
fn test_equal_operator() {
    let mut env = standard_env();

    let input = "(= 1 1)";
    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Bool(true));

    let input = "(= 1 2)";
    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Bool(false));
}

#[test]
fn test_gt_operator() {
    let mut env = standard_env();

    let input = "(> 2 1)";
    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Bool(true));
}

#[test]
fn test_lt_operator() {
    let mut env = standard_env();

    let input = "(< 1 2)";
    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Bool(true));
}

#[test]
fn test_gte_operator() {
    let mut env = standard_env();

    let input = "(>= 2 2)";
    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Bool(true));
}

#[test]
fn test_lte_operator() {
    let mut env = standard_env();

    let input = "(<= 1 2)";
    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Bool(true));
}

#[test]
fn test_if_expression() {
    let mut env = standard_env();

    let input = "(if (= 1 1) 10 20)";
    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Number(10.0));

    let input = "(if (= 1 2) 10 20)";
    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Number(20.0));
}

#[test]
fn test_define() {
    let mut env = standard_env();

    let input = "(define r 10)";
    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Symbol("r".to_string()));
}

#[test]
fn test_function_definition() {
    let mut env = standard_env();

    let input = "(define (square x) (* x x))";
    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Symbol("square".to_string()));

    let input = "(square 5)";
    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Number(25.0));
}

#[test]
fn test_fibonacci() {
    let mut env = standard_env();

    let input = "(define (fib n) (if (< n 2) 1 (+ (fib (- n 1)) (fib (- n 2)))))";
    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Symbol("fib".to_string()));

    let input = "(fib 10)";
    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Number(89.0));
}

#[test]
fn test_factorial() {
    let mut env = standard_env();

    let input = "(define (fact n) (if (<= n 1) 1 (* n (fact (- n 1)))))";
    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Symbol("fact".to_string()));

    let input = "(fact 5)";
    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Number(120.0));
}

#[test]
fn test_area_of_a_circle() {
    let mut env = standard_env();

    let input = "(define r 10)";
    let result = eval(input, &mut env).unwrap();
    assert_eq!(result, Expression::Symbol("r".to_string()));

    let input = "(* pi (* r r))";
    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Number(PI * 10.0 * 10.0));
}

#[test]
fn test_nested_functions() {
    let mut env = standard_env();

    let input = "(define (cube x) (define (square x) (* x x)) (* x (square x)))";
    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Symbol("cube".to_string()));

    let input = "(cube 3)";
    let result = eval(input, &mut env).unwrap();

    assert_eq!(result, Expression::Number(27.0));
}
