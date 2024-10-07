use crate::env::Environment;
use crate::parser::{parse, Expression, Procedure};

fn eval_define(list: &[Expression], env: &mut Environment) -> Result<Expression, String> {
    if list.len() < 3 {
        return Err("'define' requires at least two arguments".into());
    }

    // Define a new function or variable
    if let Expression::List(func) = list.get(1).unwrap() {
        if let Some(Expression::Symbol(func_name)) = func.get(0) {
            let params = func[1..].to_vec();
            let body = list.get(2..).ok_or("Invalid define syntax")?.to_vec();

            let proc = Procedure {
                params,
                body,
                env: env.clone(),
            };

            let function = Expression::Function(proc);

            env.insert(func_name.clone(), function);
            Ok(Expression::Symbol(func_name.clone()))
        } else {
            Err("Invalid define syntax".into())
        }
    } else if let Expression::Symbol(var_name) = list.get(1).unwrap() {
        let value = eval_expr(list[2].clone(), env)?;
        env.insert(var_name.clone(), value.clone());
        return Ok(Expression::Symbol(var_name.clone()));
    } else {
        return Err("Invalid define syntax".into());
    }
}

fn eval_if(list: &[Expression], env: &mut Environment) -> Result<Expression, String> {
    if list.len() < 4 {
        return Err("'if' requires at least three arguments".into());
    }

    let condition = eval_expr(list[1].clone(), env)?;

    match condition {
        Expression::Bool(true) => eval_expr(list[2].clone(), env),
        Expression::Bool(false) => eval_expr(list[3].clone(), env),
        _ => Err("Invalid condition in if expression".into()),
    }
}

fn eval_list(list: &[Expression], env: &mut Environment) -> Result<Expression, String> {
    let first = &list[0];
    if let Expression::Symbol(s) = first {
        match s.as_str() {
            "define" => eval_define(&list, env),
            "if" => eval_if(&list, env),
            _ => {
                if let Some(exp) = env.get(s) {
                    match exp {
                        Expression::Func(f) => {
                            let function = f.clone();
                            let args: Result<Vec<Expression>, String> = list[1..]
                                .iter()
                                .map(|x| eval_expr(x.clone(), env))
                                .collect();
                            Ok(function(&args?))
                        }
                        Expression::Function(proc) => {
                            let env_clone = &mut env.clone();

                            let args: Result<Vec<Expression>, String> = list[1..]
                                .iter()
                                .map(|x| eval_expr(x.clone(), env_clone))
                                .collect();

                            // Create a new execution environment for the function
                            let mut local_env = proc.env.clone();

                            // Insert the function name into the new environment
                            local_env.insert(s.clone(), exp.clone());

                            for (param, arg) in proc.params.iter().zip(args?) {
                                if let Expression::Symbol(param_name) = param {
                                    local_env.insert(param_name.clone(), arg);
                                } else {
                                    return Err("Invalid parameter name".into());
                                }
                            }

                            let mut result = Expression::Bool(false);

                            for exp in proc.body.clone() {
                                result = eval_expr(exp.clone(), &mut local_env)?;
                            }

                            Ok(result)
                        }
                        _ => Err(format!("Undefined function: {}", s)),
                    }
                } else {
                    Err(format!("Undefined function: {}", s))
                }
            }
        }
    } else {
        Err("Expected a symbol".into())
    }
}

fn eval_expr(expr: Expression, env: &mut Environment) -> Result<Expression, String> {
    match expr {
        Expression::Bool(_) => Ok(expr),
        Expression::Symbol(s) => env
            .get(&s)
            .cloned()
            .ok_or_else(|| format!("Undefined symbol: {}", s)),
        Expression::Number(_) => Ok(expr),
        Expression::List(list) => eval_list(&list, env),
        Expression::Func(_) => Ok(expr),
        Expression::Function(_) => Err("Unexpected function definition".into()),
    }
}

pub fn eval(program: &str, env: &mut Environment) -> Result<Expression, String> {
    let parsed_expr = match parse(program) {
        Ok(expr) => expr,
        Err(error) => {
            eprintln!("Error during parsing: {}", error);
            std::process::exit(1);
        }
    };

    let evaluated_expression = eval_expr(parsed_expr, env)?;

    Ok(evaluated_expression)
}
