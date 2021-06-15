// This function accepts any "writer" that implements `std::io::Write`. See the
// implementors in the docs (Stdout, Stderr, LineWriter, BufWriter, etc.). In
// the test it writes to a `Vec<u8>`.
// https://doc.rust-lang.org/1.39.0/std/io/trait.Write.html
//
// In the `find_content_in_file` test, it writes to a temporary file.
//
// If this function returned a `String`, it would collect everything into a
// string and dump all the results at the end instead of writing to the terminal
// directly.
pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).expect("error writing output");
        }
    }
}

#[test]
fn find_a_match() {
    // Since stdout expects bytes (not strings), we use `std::io::Write`
    // instead of `std::fmt::Write`.
    //
    // From the docs:
    // > Write is implemented for Vec<u8> by appending to the vector. The vector
    // > will grow as needed.
    let mut result = Vec::new(); // Vec<u8> inferred
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);

    // The `b` makes it a _byte string literal_ (`&[u8]` instead of `&str`).
    //
    // From the docs: "A string slice (&str) is made of bytes (u8), and a byte
    // slice (&[u8]) is made of bytes... Not all byte slices are valid string
    // slices, however: &str requires that it is valid UTF-8."
    // https://doc.rust-lang.org/std/str/fn.from_utf8.html
    //
    // There's an `&[u8]` being converted to an `&str` here:
    // https://stackoverflow.com/a/24159933/1365699
    //
    // and a discussion of `&str` and byte slices:
    // https://old.reddit.com/r/rust/comments/3btcgk/convert_a_str_back_to_a_u8_array/
    assert_eq!(result, b"lorem ipsum\n");
}
