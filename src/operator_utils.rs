use crate::parser::Expression;

use anyhow::{anyhow, Result};

pub fn power(args: &[Expression]) -> Result<Expression> {
    if args.len() != 2 {
        return Err(anyhow!(
            "Exponent operator requires two arguments: base and exponent factor"
        ));
    }

    let base = match args.get(0) {
        Some(Expression::Number(num)) => Ok(num),
        _ => Err(anyhow!("Expected a number")),
    }?;

    let n = match args.get(1) {
        Some(Expression::Number(num)) => Ok(num),
        _ => Err(anyhow!("Expected a number")),
    }?;

    let result = base.powf(*n);

    Ok(Expression::Number(result))
}

pub fn add(args: &[Expression]) -> Result<Expression> {
    let sum = args.iter().try_fold(0.0, |acc, arg| {
        if let Expression::Number(num) = arg {
            Ok(acc + num)
        } else {
            Err(anyhow!("Expected a number"))
        }
    })?;

    Ok(Expression::Number(sum))
}

pub fn subtract(args: &[Expression]) -> Result<Expression> {
    let first = get_number(args);

    let result = args.iter().skip(1).try_fold(first, |acc, arg| {
        if let Expression::Number(num) = arg {
            Ok(acc - num)
        } else {
            Err(anyhow!("Expected a number"))
        }
    })?;

    Ok(Expression::Number(result))
}

pub fn multiply(args: &[Expression]) -> Result<Expression> {
    let product = args.iter().try_fold(1.0, |acc, arg| {
        if let Expression::Number(num) = arg {
            Ok(acc * num)
        } else {
            Err(anyhow!("Expected a number"))
        }
    })?;

    Ok(Expression::Number(product))
}

pub fn divide(args: &[Expression]) -> Result<Expression> {
    let first = get_number(args);

    let quotient = args.iter().skip(1).try_fold(first, |acc, arg| {
        if let Expression::Number(num) = arg {
            if *num == 0.0 {
                panic!("Cannot divide by zero")
            }
            Ok(acc / num)
        } else {
            Err(anyhow!("Expected a number"))
        }
    })?;

    Ok(Expression::Number(quotient))
}

pub fn compare(args: &[Expression], op: &str) -> Result<Expression> {
    if args.len() != 2 {
        return Err(anyhow!(
            "Comparison operators require exactly two arguments"
        ));
    }

    let a = match args.get(0) {
        Some(Expression::Number(num)) => Ok(num),
        _ => Err(anyhow!("Expected a number")),
    }?;

    let b = match args.get(1) {
        Some(Expression::Number(num)) => Ok(num),
        _ => Err(anyhow!("Expected a number")),
    }?;

    let result = match op {
        "=" => a == b,
        ">" => a > b,
        "<" => a < b,
        ">=" => a >= b,
        "<=" => a <= b,
        _ => panic!("Unknown operator"),
    };

    Ok(Expression::Bool(result))
}

fn get_number(args: &[Expression]) -> f64 {
    args.iter()
        .next()
        .and_then(|arg| {
            if let Expression::Number(num) = arg {
                Some(*num)
            } else {
                None
            }
        })
        .expect("Expected a number")
}
