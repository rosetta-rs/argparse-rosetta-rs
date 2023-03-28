# Rust Arg Parsing Benchmarks

This repo tries to assess Rust arg parsing performance.

We currently compare:

Name                                                 | Style                 | Notes
-----------------------------------------------------|-----------------------|------
No-op                                                | N/A                   | N/A
[argh](https://github.com/google/argh)               | `derive`              |
[bpaf](https://github.com/pacak/bpaf)                | Combinatoric or `derive` |
[clap_lex](https://github.com/clap-rs/clap)          | Imperative            | No help generation
[clap](https://github.com/clap-rs/clap)              | Builder or `derive`   | Color, suggested fixes, completions
[gumdrop](https://github.com/murarth/gumdrop)        | `derive`              |
[lexopt](https://github.com/blyxxyz/lexopt)          | Imperative            | No help generation
[pico-args](https://github.com/razrfalcon/pico-args) | Imperative            | No help generation
[xflags](https://github.com/matklad/xflags)          | proc-macro            |

See also [an examination of design trade offs](docs/tradeoffs.md)

*Note: any non-performance comparison is meant to provide context for what you
gain/lose with each crate's overhead.  For a full comparison, see each parser
docs*

# Results

Name | Overhead (release) | Build (debug) | Parse (release) | Invalid UTF-8 | Downloads | Version
-----|--------------------|---------------|-----------------|---------------|-----------|--------
null | 0 KiB | 354ms *(full)* <br/>204ms *(incremental)* | 3ms | Y | - | -
argh | 36 KiB | 4s *(full)* <br/>238ms *(incremental)* | 4ms | N | ![Download count](https://img.shields.io/crates/dr/argh) | v0.1.10
bpaf | 221 KiB | 1s *(full)* <br/>273ms *(incremental)* | 5ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.7.10
bpaf_derive | 220 KiB | 6s *(full)* <br/>274ms *(incremental)* | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.7.10
clap | 608 KiB | 6s *(full)* <br/>488ms *(incremental)* | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.2.0
clap-minimal | 395 KiB | 3s *(full)* <br/>364ms *(incremental)* | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.2.0
clap_derive | 641 KiB | 8s *(full)* <br/>511ms *(incremental)* | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.2.0
clap_lex | 32 KiB | 559ms *(full)* <br/>224ms *(incremental)* | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/clap_lex) | v0.4.1
gumdrop | 33 KiB | 4s *(full)* <br/>233ms *(incremental)* | 3ms | N | ![Download count](https://img.shields.io/crates/dr/gumdrop) | v0.8.1
lexopt | 32 KiB | 534ms *(full)* <br/>216ms *(incremental)* | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/lexopt) | v0.3.0
pico-args | 28 KiB | 532ms *(full)* <br/>218ms *(incremental)* | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/pico-args) | v0.5.0
xflags | 23 KiB | 990ms *(full)* <br/>213ms *(incremental)* | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/xflags) | v0.3.1

*System: Linux 5.4.0-104-generic (x86_64) w/ `-j 8`*

*rustc: rustc 1.68.1 (8460ca823 2023-03-20)*

Notes:
- Overhead will be lower if your application shares dependencies with your argument parsing library.

# Running the Benchmarks

```bash
$ ./bench.py
$ ./format.py
```

To be included, the crate needs meet one of the following criteria:
- 10k+ recent downloads
- Unique API design

# Special Thanks

- RazrFalcon for creating the [initial benchmarks](https://github.com/RazrFalcon/pico-args)
- djc for inspiration with [template-benchmarks-rs](https://github.com/djc/template-benchmarks-rs)
- sharkdp for [hyperfine](https://github.com/sharkdp/hyperfine)
