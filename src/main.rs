// env_logger is the logging adapter in this project, but there are more
// listed here:
// https://docs.rs/log/0.4.14/log/
// env_logger looks for an env var like: `RUST_LOG=info`
use log::{info, warn};

// colors
// https://docs.rs/ansi_term/0.9.0/ansi_term/
use ansi_term::Colour::Cyan;

// https://docs.rs/indicatif/0.16.2/indicatif/
use indicatif::ProgressBar;

// custom errors, for example:
// let content = std::fs::read_to_string(path)
//     .with_context(|| format!("could not read file `{}`", path))?;
// https://docs.rs/anyhow/1.0.41/anyhow/trait.Context.html
use anyhow::{Context, Result};

// use structs to create `clap` code
// https://docs.rs/structopt/0.3.21/structopt/
use structopt::StructOpt;

// Add a --verbose flag to CLIs
// https://crates.io/crates/clap-verbosity-flag
// extern crate clap_verbosity_flag;

// BufWriter has better performance than println!, because it buffers up to
// 8 kB (by default).
// https://rust-cli.github.io/book/tutorial/output.html
use std::io::{self, Write};

#[derive(Debug, StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    // #[structopt(flatten)]
    // verbose: clap_verbosity_flag::Verbosity,
}

// This function accepts any "writer" that implements `std::io::Write`. See the
// implementors in the docs (Stdout, Stderr, LineWriter, BufWriter, etc.). In
// the test it writes to a `Vec<u8>`.
// https://doc.rust-lang.org/1.39.0/std/io/trait.Write.html
//
// If this function returned a `String`, it would collect everything into a
// string and dump all the results at the end instead of writing to the terminal
// directly.
fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).expect("error writing output");
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();
    info!("booting application");
    warn!("testing a warn message");
    let stdout = io::stdout();
    // let mut handle = io::BufWriter::new(stdout);
    // locking here prevents the system from locking and unlocking over and over
    // The tutorial said you can combine the two approaches, but didn't provide
    // an example.
    let mut handle = stdout.lock();
    writeln!(handle, "foo: {}", 127)?;

    let args = Cli::from_args();
    // println!("{:?}", args);

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let bar = ProgressBar::new(20000);
    for _ in 0..20000 {
        bar.inc(1);
    }
    bar.finish();

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

#[test]
fn find_a_match() {
    // Since stdout expects bytes (not strings), we use `std::io::Write`
    // instead of `std::fmt::Write`. From the docs:
    // > Write is implemented for Vec<u8> by appending to the vector. The vector
    // > will grow as needed.
    let mut result = Vec::new(); // Vec<u8> inferred
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);

    // The `b` makes it a _byte string literal_ (`&[u8]` instead of `&str`)
    assert_eq!(result, b"lorem ipsum\n");
}

// Example of testing
// fn answer() -> i32 {
//     42
// }

// #[test]
// fn check_answer_validity() {
//     assert_eq!(answer(), 42);
// }
