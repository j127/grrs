Some experiments and notes based on [this](https://rust-cli.github.io/book/index.html).

```text
$ RUST_LOG=info cargo run -- main src/main.rs
$ cargo build --release
```

Other tips from the tutorial:

-   You can use [ctrlc](https://crates.io/crates/ctrlc) to handle `ctrl-c`.
-   You can handle signals with [Signal-hook](https://crates.io/crates/signal-hook).
-   [crossbeam-channel](https://crates.io/crates/crossbeam-channel)
-   For exit codes see [exitcode](https://crates.io/crates/exitcode) and [this page](https://rust-cli.github.io/book/in-depth/exit-code.html).
-   For friendlier panic messages, check out [human-panic](https://crates.io/crates/human-panic).
-   To check if your program is running in a terminal or being piped into another program, see [atty](https://crates.io/crates/atty). (example: to turn colors off)
-   JSON: [serde_json](https://crates.io/crates/serde_json)
-   [How to generate a `man` page](https://rust-cli.github.io/book/in-depth/docs.html).

There are other useful crates mentioned [here](https://rust-cli.github.io/book/crates/index.html).

-   [CLI crates](https://lib.rs/command-line-interface)
-   [Database interfaces](https://lib.rs/database)
-   [HTTP clients](https://lib.rs/web-programming/http-client) (take a look at [reqwest](https://github.com/seanmonstar/reqwest))
