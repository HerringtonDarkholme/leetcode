pub fn test_cases<T, R>(cases: Vec<(T, R)>, solution: fn(T)->R)
    where R: std::fmt::Debug + Eq {
    for (t, r) in cases {
        assert_eq!(solution(t), r);
    }
}
