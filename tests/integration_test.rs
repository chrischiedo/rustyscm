use rustyscm::env::standard_env;
use rustyscm::eval::eval;
use rustyscm::parser::{parse, Expression};

use std::f64::consts::PI;


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

    let input1 = "(= 1 1)";
    let result1 = eval(input1, &mut env).unwrap();

    assert_eq!(result1, Expression::Bool(true));

    let input2 = "(= 1 2)";
    let result2 = eval(input2, &mut env).unwrap();

    assert_eq!(result2, Expression::Bool(false));
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

    let input1 = "(if (= 1 1) 10 20)";
    let result1 = eval(input1, &mut env).unwrap();

    assert_eq!(result1, Expression::Number(10.0));

    let input2 = "(if (= 1 2) 10 20)";
    let result2 = eval(input2, &mut env).unwrap();

    assert_eq!(result2, Expression::Number(20.0));
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

    let input1 = "(define (square x) (* x x))";
    let result1 = eval(input1, &mut env).unwrap();

    assert_eq!(result1, Expression::Symbol("square".to_string()));

    let input2 = "(square 5)";
    let result2 = eval(input2, &mut env).unwrap();

    assert_eq!(result2, Expression::Number(25.0));
}

#[test]
fn test_fibonacci() {
    let mut env = standard_env();

    let input1 = "(define (fib n) (if (< n 2) 1 (+ (fib (- n 1)) (fib (- n 2)))))";
    let result1 = eval(input1, &mut env).unwrap();

    assert_eq!(result1, Expression::Symbol("fib".to_string()));

    let input2 = "(fib 10)";
    let result2 = eval(input2, &mut env).unwrap();

    assert_eq!(result2, Expression::Number(89.0));
}

#[test]
fn test_factorial() {
    let mut env = standard_env();

    let input1 = "(define (fact n) (if (<= n 1) 1 (* n (fact (- n 1)))))";
    let result1 = eval(input1, &mut env).unwrap();

    assert_eq!(result1, Expression::Symbol("fact".to_string()));

    let input2 = "(fact 5)";
    let result2 = eval(input2, &mut env).unwrap();

    assert_eq!(result2, Expression::Number(120.0));
}

#[test]
fn test_area_of_a_circle() {
    let mut env = standard_env();

    let input1 = "(define r 10)";
    let result1 = eval(input1, &mut env).unwrap();
    assert_eq!(result1, Expression::Symbol("r".to_string()));

    let input2 = "(* pi (* r r))";
    let result2 = eval(input2, &mut env).unwrap();

    assert_eq!(result2, Expression::Number(PI * 10.0 * 10.0));
}

#[test]
fn test_nested_functions() {
    let mut env = standard_env();

    let input1 = "(define (cube x) (define (square x) (* x x)) (* x (square x)))";
    let result1 = eval(input1, &mut env).unwrap();

    assert_eq!(result1, Expression::Symbol("cube".to_string()));

    let input2 = "(cube 3)";
    let result2 = eval(input2, &mut env).unwrap();

    assert_eq!(result2, Expression::Number(27.0));
}
