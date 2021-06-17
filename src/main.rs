use std::time::Duration;
use crossbeam_channel::{bounded, tick, Receiver, select};

use serde::{Serialize, Deserialize};

// env_logger is the logging adapter in this project, but there are more
// listed here:
// https://docs.rs/log/0.4.14/log/
// env_logger looks for an env var like: `RUST_LOG=info`
use log::{info, warn};

// colors
// https://docs.rs/ansi_term/0.9.0/ansi_term/
// use ansi_term::Colour::Cyan;

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

// confy reading from the config.toml file
#[derive(Debug, Default, Serialize, Deserialize)]
struct MyConfig {
    message: String,
    num: i32,
}

#[derive(Debug, StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    // #[structopt(flatten)]
    // verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    env_logger::init();
    info!("booting application");
    warn!("testing a warn message");

    let cfg: MyConfig = confy::load("grrs")?;
    info!("read config from toml file: {:?}, {:?}", cfg.message, cfg.num);

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

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    let ctrl_c_events = grrs::ctrl_channel()?;
    let ticks = tick(Duration::from_secs(1));

    loop {
        select! {
            recv(ticks) -> _ => {
                println!("working (ctrl-c to exit)");
            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("bye");
                break;
            }
        }
    }

    Ok(())
}

// Example test
fn answer() -> i32 {
    42
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}
