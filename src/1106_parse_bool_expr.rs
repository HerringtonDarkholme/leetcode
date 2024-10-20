impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let expr: Vec<_> = expression.chars().collect();
        let ast = parse(&expr).0;
        evaluate(&ast)
    }
}

#[derive(Debug)]
enum Ast {
    True,
    False,
    Not(Box<Ast>),
    And(Vec<Ast>),
    Or(Vec<Ast>),
}
fn evaluate(ast: &Ast) -> bool {
    match ast {
        True => true,
        False => false,
        Not(a) => !evaluate(&a),
        And(a) => a.iter().all(evaluate),
        Or(a) => a.iter().any(evaluate),
    }
}

use Ast::*;
fn parse(expr: &[char]) -> (Ast, usize) {
    let c = expr.first().expect("invalid input");
    match c {
        't' => (True, 1),
        'f' => (False, 1),
        '!' => {
            let (not, size) = parse_not(&expr[1..]);
            (Not(not), size + 1)
        },
        '&' => {
            let (and, size) = parse_list(&expr[1..]);
            (And(and), size + 1)
        },
        '|' => {
            let (or, size) = parse_list(&expr[1..]);
            (Or(or), size + 1)
        },
        _ => panic!("invalid input")
    }
}

fn parse_not(expr: &[char]) -> (Box<Ast>, usize) {
    let len = expr.len();
    assert_eq!(expr[0], '(');
    let (not, size) = parse(&expr[1..len - 1]);
    assert_eq!(expr[size + 1], ')');
    (Box::new(not), size + 2)
}

fn parse_list(expr: &[char]) -> (Vec<Ast>, usize) {
    let len = expr.len();
    assert_eq!(expr[0], '(');
    let mut i = 1;
    let mut ret = vec![];
    loop {
        let (ast, s) = parse(&expr[i..]);
        ret.push(ast);
        i += s;
        if expr[i] == ',' {
            i += 1;
        } else if expr[i] == ')' {
            break;
        } else {
            panic!("invalid input");
        }
    }
    (ret, i + 1)
}
