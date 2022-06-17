# AIPS Code Challenge

## Requirements:

1. _This_ repo: `git clone https://github.com/alphastrata/aips`.
2. (Rust, and its toolchain: cargo)[https://www.rust-lang.org/tools/install].
3. A working internet connection (for cargo to fetch crates[libraries]).
4. Some cmdline/terminal proficiency.
5. A browser, if you're wanting to read documentation for `src/lib.rs`.

- `NOTE:You may need to restart your machine/relogin to your shell (depending on your os). `

## Building:

- `cargo build --release` omit the `--release` flag for debug builds.

## Usage:

- The program reads its data from *this* directory, it requires a file called `data.txt`, see below for more details. If your data conforms to the same formatting etc you can easily append or override the `data.txt` file entirely as suits your needs.
- `./target/release/aips` for macOS/Linux and unix-like systems.
- `.\target\release\aips.exe` for Windows.

## Testing:

From your terminal of choice:

- `cargo test` to run all tests, add the `-v` flag for verbose options.

## Docs:

- `cargo doc` to generate the documentation.
- `cargo doc --open` to open the generated documentation in your browser.

### Data:
Data must be format exactly:
```
2021-12-05T15:30:00 15
2021-12-08T18:00:00 33
2022-03_27:00:00 2 
```
Notes:
*`yyyy-mm- ddThh:mm:ss n` where `n` denotes the number of cars counted, positive values only.
* data need not be contiguous

TODO:
### Benchmarking:
*`cargo bench`
