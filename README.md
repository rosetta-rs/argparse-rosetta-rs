# Rust Arg Parsing Benchmarks

This repo tries to assess Rust arg parsing performance.

We currently compare:

Name                                                 | Style                 | Notes
-----------------------------------------------------|-----------------------|------
No-op                                                | N/A                   | N/A
[argh](https://github.com/google/argh)               | `derive`              |
[argp](https://github.com/jirutka/argp)              | `derive`              |
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
null | 0 KiB | 399ms *(full)* <br/>200ms *(incremental)* | 3ms | Y | - | -
argh | 35 KiB | 4s *(full)* <br/>237ms *(incremental)* | 3ms | N | ![Download count](https://img.shields.io/crates/dr/argh) | v0.1.8
bpaf | 165 KiB | 1s *(full)* <br/>256ms *(incremental)* | 5ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.6.0
bpaf_derive | 160 KiB | 7s *(full)* <br/>256ms *(incremental)* | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.6.0
clap | 607 KiB | 4s *(full)* <br/>467ms *(incremental)* | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.0.0
clap-minimal | 417 KiB | 3s *(full)* <br/>380ms *(incremental)* | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.0.0
clap_derive | 637 KiB | 10s *(full)* <br/>501ms *(incremental)* | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.0.0
clap_lex | 37 KiB | 759ms *(full)* <br/>229ms *(incremental)* | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/clap_lex) | v0.3.0
gumdrop | 33 KiB | 4s *(full)* <br/>233ms *(incremental)* | 3ms | N | ![Download count](https://img.shields.io/crates/dr/gumdrop) | v0.8.1
lexopt | 36 KiB | 591ms *(full)* <br/>217ms *(incremental)* | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/lexopt) | v0.2.1
pico-args | 28 KiB | 583ms *(full)* <br/>215ms *(incremental)* | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/pico-args) | v0.5.0
xflags | 24 KiB | 1s *(full)* <br/>210ms *(incremental)* | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/xflags) | v0.2.4

*System: Linux 5.4.0-104-generic (x86_64) w/ `-j 8`*

*rustc: rustc 1.64.0 (a55dd71d5 2022-09-19)*

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
