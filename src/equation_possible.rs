pub struct Solution;

use std::collections::HashSet;
use std::rc::Rc;
impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        const CH_SIZE: usize = 26;
        let mut equality = Vec::with_capacity(CH_SIZE);
        for i in 0..26 {
            let mut set = HashSet::new();
            set.insert(i);
            equality.push(Rc::new(set));
        }
        let mut inequality = vec![HashSet::<u8>::new(); CH_SIZE];
        let a = "a".as_bytes()[0];
        let equal = "=".as_bytes()[0];
        for equation in equations {
            let equation = equation.as_bytes();
            let c1 = equation[0] - a;
            let c2 = equation[3] - a;
            if equation[1] == equal {
                let union: HashSet<u8> = equality[c1 as usize].union(&equality[c2 as usize]).map(|&u| u).collect();
                let union = Rc::new(union);
                for &i in union.iter() {
                    equality[i as usize] = union.clone();
                    if union.intersection(&inequality[i as usize]).next().is_some() {
                        return false
                    }
                }
            } else {
                if equality[c1 as usize].contains(&c2) || equality[c2 as usize].contains(&c1) {
                    return false
                }
                inequality[c1 as usize].insert(c2);
                inequality[c2 as usize].insert(c1);
            }
        }
        true
    }
}

#[test]
fn test() {
    use crate::util::test::test_cases;
    test_cases(vec![
        (nested!["a==b", "a!=b"], false),
        (nested!["a==b", "a==c", "b==c"], true),
        (nested!["c==c", "b==d", "x!=z"], true),
        (nested!["a==b", "b!=c", "c==a"], false),
        (nested!["a!=b", "b==c", "c==a"], false),
        (nested!["a==b", "b==c", "c!=a"], false),
        (nested!["a!=a"], false),
        (nested!["a==b","e==c","b==c","a!=e"], false),
    ], Solution::equations_possible);
}
