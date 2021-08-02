use std::io::{self, Write};

// /// Prints some text to stdout and reads a line from stdin.
// pub(crate) fn ask(msg: &str) -> String {
//     let mut input = String::new();
//     ask_fmt(msg, &mut input);
//     input
// }

/// Prints some text to stdout, reads a line from stdin, and appends it to
/// `input`.
fn ask_fmt(msg: &str, input: &mut String) {
    let mut stdout = io::stdout();
    write!(stdout, "{} ", msg)
        .and_then(|()| stdout.flush())
        .expect("io write failed");

    let prev_len = input.len();
    io::stdin().read_line(input)
        .expect("io read failed");
    if prev_len < input.len() {
        // Only trim the part we appended
        input.truncate(prev_len + input[prev_len..].trim_end().len());
    }
}

/// Prints text to stdout and reads lines from stdin. This continues until it
/// encounters a blank line.
pub(crate) fn ask_multi_lines(msg: &str) -> String {
    let mut msg = msg; // shorten the lifetime (smh why do I need to do this)
    let space = " ".repeat(msg.len());
    let mut input = String::new();

    let mut prev_len = input.len();
    loop {
        ask_fmt(msg, &mut input);
        msg = &space;
        if prev_len == input.len() { break; }

        input.push('\n');
        prev_len = input.len();
    }

    input.with_end_trimmed()
}

/// Contains a convenience method for working with collections.
pub(crate) trait NonEmpty: Sized {
    /// Returns `None` if `self` is empty, and `Some(self)` otherwise.
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

/// Contains a convenience method for trimming strings in-place.
pub(crate) trait WithEndTrimmed {
    /// Returns `self` with any whitespace trimmed off its end.
    ///
    /// Trimming only the end is a fast operation for `String`.
    fn with_end_trimmed(self) -> Self;
}

impl WithEndTrimmed for String {
    fn with_end_trimmed(mut self) -> Self {
        self.truncate(self.trim_end().len());
        self
    }
}
