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
null | 0 KiB | 461ms *(full)* <br/>246ms *(incremental)* | 1ms | Y | - | -
argh | 36 KiB | 5s *(full)* <br/>292ms *(incremental)* | 1ms | N | ![Download count](https://img.shields.io/crates/dr/argh) | v0.1.10
bpaf | 221 KiB | 2s *(full)* <br/>348ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.7.10
bpaf_derive | 220 KiB | 8s *(full)* <br/>349ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.7.10
clap | 607 KiB | 7s *(full)* <br/>574ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.1.12
clap-minimal | 426 KiB | 4s *(full)* <br/>448ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.1.12
clap_derive | 644 KiB | 12s *(full)* <br/>644ms *(incremental)* | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.1.12
clap_lex | 33 KiB | 882ms *(full)* <br/>277ms *(incremental)* | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/clap_lex) | v0.3.3
gumdrop | 33 KiB | 5s *(full)* <br/>301ms *(incremental)* | 1ms | N | ![Download count](https://img.shields.io/crates/dr/gumdrop) | v0.8.1
lexopt | 32 KiB | 721ms *(full)* <br/>266ms *(incremental)* | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/lexopt) | v0.3.0
pico-args | 28 KiB | 662ms *(full)* <br/>266ms *(incremental)* | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/pico-args) | v0.5.0
xflags | 23 KiB | 1s *(full)* <br/>286ms *(incremental)* | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/xflags) | v0.3.1

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
