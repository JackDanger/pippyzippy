# lazippy

Pure-Rust LZMA (Lempel-Ziv-Markov chain Algorithm) encoder/decoder, part of the
[8z](https://github.com/JackDanger/8z) umbrella of pure-Rust compression codecs.

See [STATUS.md](./STATUS.md) for the current implementation state.

## Build & Test

```sh
cargo build
cargo test
cargo test -- --include-ignored   # run all tests including #[ignore] stubs
cargo bench --no-run               # verify bench targets compile
```
