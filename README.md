# Scheme Interpreter in Rust

This project implements a lexer, parser, and an evaluator for a minimal subset of Scheme. It is written from scratch in Rust.

Check out the companion blog post, [Building a minimal Scheme Interpreter in Rust](https://chrischiedo.github.io/scheme-interpreter-rust), for a guided walkthrough of the code.

## Main Features

<ul style="margin-top: 0px; margin-bottom: 0px;">
  <li><strong>Lexer</strong>: Breaks up input string (source code) into a sequence of tokens.</li>
  <li><strong>Parser</strong>: Parses Scheme expressions into an abstract syntax tree (AST).</li>
  <li><strong>Evaluator</strong>: Evaluates expressions according to Scheme semantic rules.</li>
  <li><strong>Environment handling</strong>: Manages variable scope and function definitions.</li>
</ul>

<ul style="margin-top: 0px; margin-bottom: 0px;">
  <li><strong>Numbers</strong>: Supports signed 64-bit integers and floating point numbers, with pi being defined in the standard environment.</li>
  <li><strong>Arithmetic operations</strong>: Supports basic arithmetic operations (+, -, *, /).</li>
  <li><strong>Comparison operations</strong>: Supports comparison operators (>, <, =, >=, <=).</li>
  <li><strong>Variable definitions</strong>: Allows user-defined variables.</li>
  <li><strong>Function definitions</strong>: Allows user-defined functions with support for recursion.</li>
  <li><strong>Conditional expressions</strong>: Supports the evaluation of 'if' expressions.</li>
  <li><strong>REPL</strong>: A Read-Eval-Print Loop for interactive programming.</li>
</ul>


## Example Usage

You can start the REPL by running the following in your shell:

```bash
$ cargo run
```

In the REPL, you can enter Scheme expressions and evaluate them. Here are some examples:

```bash
schemer>
(+ 2 2)
 ==> 4
schemer>
(+ 2 2 3)
 ==> 7
schemer>
(- 4 2)
 ==> 2
schemer>
(+ 10 5 (- 10 3 3))
 ==> 19
schemer>
(* x 2)
 ==> 10
schemer>
(pow 2 16)
 ==> 65536
schemer>
(define r 10)
 ==> r
schemer>
(* pi (* r r))
 ==> 314.1592653589793
schemer>
(>= 4 2)
 ==> true
schemer>
(if (> 2 4 ) 1 2)
 ==> 2
schemer>
(define (sum a b) (+ a b))
 ==> sum
schemer>
(sum 10 20)
 ==> 30
schemer>
(define (square x) (* x x))
 ==> square
schemer>
(square 5)
 ==> 25
schemer>
(define (circle-area r) (* pi (* r r)))
 ==> circle-area
schemer>
(circle-area 3)
 ==> 28.274333882308138
schemer>
(define (fact n) (if (<= n 1) 1 (* n (fact (- n 1)))))
 ==> fact
schemer>
(fact 10)
 ==> 3628800
schemer>
(define (cube x) (define (square x) (* x x)) (* x (square x)))
 ==> cube
schemer>
(cube 3)
 ==> 27
```

## Tests

To run all the tests for the project use:

```bash
$ cargo test
```
