use std::io::{self, Write};

pub(crate) fn ask(msg: &str) -> String {
    let mut stdout = io::stdout();
    write!(stdout, "{} ", msg)
        .and_then(|()| stdout.flush())
        .expect("io write failed");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("io read failed");
    input.truncate(input.trim_end().len());
    input
}

pub(crate) trait NonEmpty: Sized {
    fn non_empty(self) -> Option<Self>;
}

macro_rules! impl_non_empty {
    () => {
        fn non_empty(self) -> Option<Self> {
            (self.len() > 0).then(|| self)
        }
    };
}

impl NonEmpty for String {
    impl_non_empty!();
}

impl<'a> NonEmpty for &'a str {
    impl_non_empty!();
}

impl<T> NonEmpty for Vec<T> {
    impl_non_empty!();
}

impl<'a, T> NonEmpty for &'a [T] {
    impl_non_empty!();
}
