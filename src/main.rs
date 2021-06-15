// colors
// https://docs.rs/ansi_term/0.9.0/ansi_term/
use ansi_term::Colour::Cyan;

use indicatif::ProgressBar;
// https://docs.rs/indicatif/0.16.2/indicatif/


// custom errors, for example:
// let content = std::fs::read_to_string(path)
//     .with_context(|| format!("could not read file `{}`", path))?;
// https://docs.rs/anyhow/1.0.41/anyhow/trait.Context.html
use anyhow::{Context, Result};

// use structs to create `clap` code
// https://docs.rs/structopt/0.3.21/structopt/
use structopt::StructOpt;

// BufWriter has better performance than println!, because it buffers up to
// 8 kB.
// https://rust-cli.github.io/book/tutorial/output.html
use std::io::{self, Write};

#[derive(Debug, StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let stdout = io::stdout();
    // let mut handle = io::BufWriter::new(stdout);
    // locking here prevents the system from locking and unlocking over and over
    // The tutorial said you can combine the two approaches, but didn't provide
    // and example.
    let mut handle = stdout.lock();
    writeln!(handle, "foo: {}", 127)?;

    let args = Cli::from_args();
    // println!("{:?}", args);

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    let bar = ProgressBar::new(20000);
    for _ in 0..20000{
        bar.inc(1);
    }
    bar.finish();

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", Cyan.paint(line));
        }
    }
    Ok(())
}
