pub struct Solution;

use std::ops::{Add, Sub, Mul, Div};

#[derive(Eq, PartialEq, Clone, Copy)]
struct Rational {
    nom: i32,
    den: i32,
}

fn gcd(a: i32, b: i32) -> i32 {
    let (mut a, mut b) = {
        let a = a.abs();
        let b = b.abs();
        (a.min(b), a.max(b))
    };
    if a <= 1 {
        return 1
    }
    while a != 0 {
        let tmp = a;
        a = b % a;
        b = tmp;
    }
    b
}

impl Rational {
    fn new(nom: i32, den: i32) -> Self {
        assert_ne!(den, 0, "denominator must not be 0");
        let g = gcd(nom, den);
        Rational {
            nom: nom / g,
            den: den / g,
        }
    }
}

impl Add for Rational {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        let nom = self.nom * o.den + o.nom * self.den;
        let den = self.den * o.den;
        Rational::new(nom, den)
    }
}

impl Sub for Rational {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        let nom = self.nom * o.den - o.nom * self.den;
        let den = self.den * o.den;
        Rational::new(nom, den)
    }
}

impl Mul for Rational {
    type Output = Self;
    fn mul(self, o: Self) -> Self {
        let nom = self.nom * o.nom;
        let den = self.den * o.den;
        Rational::new(nom, den)
    }
}

impl Div for Rational {
    type Output = Option<Self>;
    fn div(self, o: Self) -> Self::Output {
        if o.nom == 0 {
            None
        } else {
            let nom = self.nom * o.den;
            let den = self.den * o.nom;
            Some(Rational::new(nom, den))
        }
    }
}

impl From<i32> for Rational {
    fn from(i: i32) -> Self {
        Rational::new(i, 1)
    }
}

impl Solution {
    pub fn judge_point24(nums: Vec<i32>) -> bool {
        let mut nums = nums
            .into_iter()
            .map(Rational::from)
            .collect();
        judge(&mut nums)
    }
}

fn judge(nums: &mut Vec<Rational>) -> bool {
    if nums.len() == 1 {
        return nums[0] == 24.into()
    }
    let l = nums.len();
    for i in 0..l {
        let r1 = nums.remove(i);
        for j in 0..l-1 {
            let r2 = nums.remove(j);
            let mut rs = [r1+r2, r1-r2, r1*r2, r1+r2];
            if r2.nom != 0 {
                rs[3] = (r1/r2).unwrap();
            };
            for &r in &rs {
                nums.push(r);
                if judge(nums) {
                    return true
                }
                nums.pop();
            }
            nums.insert(j, r2);
        }
        nums.insert(i, r1)
    }
    false
}
