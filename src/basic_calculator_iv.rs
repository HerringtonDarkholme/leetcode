use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn basic_calculator_iv(expression: String, evalvars: Vec<String>, evalints: Vec<i32>) -> Vec<String> {
        let env = evalvars.into_iter().zip(evalints.into_iter()).collect();
        let mut parser = Parser::new(expression);
        let mut poly = parser.parse();
        poly.evaluate(&env);
        poly.to_list()
    }
}

struct Parser {
    chars: Vec<char>,
    index: usize,
}

impl Parser {
    fn new(expression: String) -> Self {
        Parser {
            chars: expression.chars().collect(),
            index: 0,
        }
    }

    fn current(&self) -> char {
        self.chars[self.index]
    }
    fn has_more(&self) -> bool {
        self.index < self.chars.len()
    }
    fn skip_white(&mut self) {
        while self.has_more() && self.current() == ' ' {
            self.index += 1;
        }
    }

    fn parse(&mut self) -> Poly {
        self.parse_expr()
    }

    // term (`+`|`-` term)*
    fn parse_expr(&mut self) -> Poly {
        let mut term = Poly::zero();
        loop {
            self.skip_white();
            if !self.has_more() {
                break term
            }
            let op = self.current();
            if op != '+' && op != '-' {
                break term
            }
            self.index += 1;
            let next = self.parse_term();
            if op == '+' {
                term.add(&next);
            } else {
                term.sub(&next);
            }
        }
    }

    // factor (`*` factor)*
    fn parse_term(&mut self) -> Poly {
        let mut factor = Poly::zero();
        loop {
            self.skip_white();
            if !self.has_more() {
                break factor
            }
            let op = self.current();
            if op != '*' {
                break factor
            }
            self.index += 1;
            let next = self.parse_factor();
            factor.mul(&next);
        }
    }

    // Var | Lit | Paren
    fn parse_factor(&mut self) -> Poly {
        self.skip_white();
        match self.current() {
            '(' => self.parse_paren(),
            '0'...'9' => self.parse_lit(),
            'a'...'z' => self.parse_var(),
            _ => panic!("format error"),
        }
    }

    // (...)
    fn parse_paren(&mut self) -> Poly {
        assert_eq!(self.current(), '(');
        self.index += 1;
        let ret = self.parse();
        assert_eq!(self.current(), ')');
        self.index += 1;
        ret
    }

    // 123
    fn parse_lit(&mut self) -> Poly {
        let mut ret = 0;
        while self.has_more() {
            match self.current() {
                c @ '0'...'9' => {
                    let r = c as i32 - '0' as i32;
                    ret = ret * 10 + r;
                },
                _ => break,
            }
            self.index += 1;
        }
        Poly::lit(ret)
    }

    // temperature
    fn parse_var(&mut self) -> Poly {
        let mut var = String::new();
        while self.has_more() {
            match self.current() {
                c @ 'a'...'z' => var.push(c),
                _ => break,
            }
            self.index += 1;
        }
        Poly::var(var)
    }
}

#[derive(Clone)]
struct Term {
    coeff: i32,
    vars: Vec<String>,
}

impl Term {
    fn var(var: String) -> Self {
        Term {
            coeff: 1,
            vars: vec![var],
        }
    }
    fn lit(coeff: i32) -> Self {
        Term {
            coeff,
            vars: vec![],
        }
    }

    fn degree(&self) -> usize {
        self.vars.len()
    }
    fn is_same(&self, other: &Self) -> bool {
        self.vars == other.vars
    }
    fn is_zero(&self) -> bool {
        self.coeff == 0
    }

    // OP
    fn add(&mut self, other: &Self) {
        assert!(self.is_same(other));
        self.coeff += other.coeff;
    }
    fn sub(&mut self, other: &Self) {
        assert!(self.is_same(other));
        self.coeff -= other.coeff;
    }
    fn mul(&mut self, other: &Self) {
        self.coeff *= other.coeff;
        self.vars.extend_from_slice(&other.vars);
        self.vars.sort();
    }
}

struct Poly {
    terms: Vec<Vec<Term>>
}

impl Poly {
    fn zero() -> Self {
        Poly { terms: vec![] }
    }
    fn lit(coeff: i32) -> Self {
        let term = Term::lit(coeff);
        Poly {
            terms: vec![vec![term]]
        }
    }
    fn var(var: String) -> Self {
        let term = Term::var(var);
        let terms = vec![
            vec![], // constant
            vec![term], // 1st degree var
        ];
        Poly { terms }
    }

    // info
    fn max_degree(&self) -> usize {
        self.terms.len()
    }

    // op
    fn add(&mut self, other: &Self) {
        unimplemented!()
    }
    fn sub(&mut self, other: &Self) {
        unimplemented!()
    }
    fn mul(&mut self, other: &Self) {
        unimplemented!()
    }
    fn evaluate(&mut self, env: &HashMap<String, i32>) {
        unimplemented!()
    }
    fn to_list(&self) -> Vec<String> {
        unimplemented!()
    }
}
