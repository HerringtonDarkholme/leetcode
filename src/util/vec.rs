#[macro_export]
macro_rules! nested {
    (
        $([
          $([
            $($x:expr),* $(,)*
          ]),+ $(,)*
        ]), + $(,)*
    ) => {
        vec![
            $(vec![
              $( vec![ $($x.to_owned()),* ] ),*
            ]),*
        ]
    };
    (
      $([
        $($x:expr),* $(,)*
      ]),+ $(,)*
    ) => {
        vec![
            $( vec![ $($x.to_owned()),* ],)*
        ]
    };
    (
        $($x:expr),*
    ) => {
        vec![ $($x.to_owned()),* ]
    };
}
