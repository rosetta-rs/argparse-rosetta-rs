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
null | 0 KiB | 234ms *(full)* <br/>172ms *(incremental)* | 3ms | Y | - | -
argh | 38 KiB | 3s *(full)* <br/>203ms *(incremental)* | 4ms | N | ![Download count](https://img.shields.io/crates/dr/argh) | v0.1.10
bpaf | 282 KiB | 965ms *(full)* <br/>236ms *(incremental)* | 5ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.9.4
bpaf_derive | 276 KiB | 4s *(full)* <br/>238ms *(incremental)* | 5ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.9.4
clap | 654 KiB | 3s *(full)* <br/>392ms *(incremental)* | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.4.0
clap-minimal | 427 KiB | 2s *(full)* <br/>330ms *(incremental)* | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.4.0
clap_derive | 689 KiB | 6s *(full)* <br/>410ms *(incremental)* | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.4.0
clap_lex | 27 KiB | 407ms *(full)* <br/>188ms *(incremental)* | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/clap_lex) | v0.5.1
gumdrop | 37 KiB | 3s *(full)* <br/>198ms *(incremental)* | 3ms | N | ![Download count](https://img.shields.io/crates/dr/gumdrop) | v0.8.1
lexopt | 34 KiB | 385ms *(full)* <br/>184ms *(incremental)* | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/lexopt) | v0.3.0
pico-args | 23 KiB | 384ms *(full)* <br/>185ms *(incremental)* | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/pico-args) | v0.5.0
xflags | 22 KiB | 709ms *(full)* <br/>179ms *(incremental)* | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/xflags) | v0.3.1

*System: Linux 5.4.0-124-generic (x86_64) w/ `-j 8`*

*rustc: rustc 1.72.0 (5680fa18f 2023-08-23)*

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
