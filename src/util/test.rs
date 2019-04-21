pub fn test_cases<T, R>(cases: Vec<(T, R)>, solution: fn(T)->R)
    where R: std::fmt::Debug + Eq {
    for (t, r) in cases {
        assert_eq!(solution(t), r);
    }
}

#[macro_export]
macro_rules! vec_str {
    ( $($x: expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.to_owned());
            )*
            temp_vec
        }
    }
}

