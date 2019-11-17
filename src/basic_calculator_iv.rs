// leetcode 770
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
                term.add(next);
            } else {
                term.sub(next);
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
            factor.mul(next);
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

struct Poly {
    terms: HashMap<Vec<String>, i32>
}

impl Poly {
    fn zero() -> Self {
        Poly { terms: HashMap::new() }
    }
    fn lit(coeff: i32) -> Self {
        let mut terms = HashMap::new();
        terms.insert(vec![], coeff);
        Poly { terms }
    }
    fn var(var: String) -> Self {
        let mut terms = HashMap::new();
        terms.insert(vec![var], 1);
        Poly { terms }
    }

    // op
    fn add(&mut self, other: Self) {
        let terms = &mut self.terms;
        for (k, v) in other.terms {
            *terms.entry(k).or_insert(0) += v;
        }
    }
    fn sub(&mut self, other: Self) {
        let terms = &mut self.terms;
        for (k, v) in other.terms {
            *terms.entry(k).or_insert(0) -= v;
        }
    }
    fn mul(&mut self, other: Self) {
        let mut new = HashMap::new();
        let terms = &self.terms;
        for (k, v) in other.terms {
            for (kk, &vv) in terms.iter() {
                let mut k = k.clone();
                k.extend_from_slice(kk);
                k.sort();
                *new.entry(k).or_insert(0) += v * vv;
            }
        }
        self.terms = new;
    }

    fn evaluate(&mut self, env: &HashMap<String, i32>) {
        let terms = std::mem::replace(&mut self.terms, HashMap::new());
        let new = &mut self.terms;
        for (vars, mut coeff) in terms {
            let mut new_vars = vec![];
            for var in vars {
                if let Some(&v) = env.get(&var) {
                    coeff *= v;
                } else {
                    new_vars.push(var);
                }
            }
            *new.entry(new_vars).or_insert(0) += coeff;
        }
    }
    fn to_list(self) -> Vec<String> {
        self.terms.into_iter().filter_map(|(mut k, v)| if v == 0 {
            None
        } else {
            k.insert(0, v.to_string());
            Some(k.join("*"))
        }).collect()
    }
}
