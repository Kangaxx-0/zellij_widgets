#[macro_export]
/// This macro is useful for asserting that two buffers are equal.
///
/// # Parameters
/// - `$actual_expr` - The actual buffer to compare
/// - `$expected_expr` - The expected buffer to compare
macro_rules! assert_buffer_eq {
    ($actual_expr:expr, $expected_expr:expr) => {
        match (&$actual_expr, &$expected_expr) {
            (actual, expected) => {
                if actual.area != expected.area {
                    panic!(
                        indoc::indoc!(
                            "
                            buffer areas not equal
                            expected:  {:?}
                            actual:    {:?}"
                        ),
                        expected, actual
                    );
                }
                // shouldn't get here, but this guards against future behavior
                // that changes equality but not area or content
                assert_eq!(actual, expected, "buffers not equal");
            }
        }
    };
}

#[macro_export]
/// This macro is useful for printing debug information to the buffer.
///
/// # Parameters
/// - `$buf` - The buffer to write to
/// - `$x` - The x position to write to
/// - `$y` - The y position to write to
/// - `$max_cols` - The maximum number of columns to write to
/// - `$msg` - The message to write
macro_rules! debug_line {
    ($buf:expr, $x:expr, $y:expr, $max_cols:expr, $msg:expr) => {{
        let line = Line::raw($msg);
        $buf.set_line($x, $y, &line, $max_cols);
    }};
}

#[macro_export]
/// This macro is useful for asserting that the content of a buffer is equal to a slice of strings.
///
/// # Parameters
/// - `$buf` - The buffer to compare
/// - `$slice_str` - The slice of strings to compare
macro_rules! assert_buffer_content {
    ($buf:expr, $slice_str:expr) => {
        for (i, symbol) in $slice_str.iter().enumerate() {
            assert_eq!($buf.content()[i].symbol, String::from(*symbol));
        }
    };
}
