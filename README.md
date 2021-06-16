Some experiments and notes based on [this](https://rust-cli.github.io/book/index.html).

```text
$ RUST_LOG=info cargo run -- main src/main.rs
$ cargo build --release
```

Other tips from the tutorial:

-   You can use [ctrlc](https://crates.io/crates/ctrlc) to handle `ctrl-c`.
-   You can handle signals with [Signal-hook](https://crates.io/crates/signal-hook).
-   [crossbeam-channel](https://crates.io/crates/crossbeam-channel)
