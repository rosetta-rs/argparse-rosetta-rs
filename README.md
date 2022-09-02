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

*Note: any non-performance comparison is meant to provide context for what you
gain/lose with each crate's overhead.  For a full comparison, see each parser
docs*

# Results

Name | Overhead (release) | Build (debug) | Parse (release) | Invalid UTF-8 | Downloads | Version
-----|--------------------|---------------|-----------------|---------------|-----------|--------
null | 0 KiB | 408ms | 3ms | Y | - | -
argh | 35 KiB | 5s | 3ms | N | ![Download count](https://img.shields.io/crates/dr/argh) | v0.1.8
bpaf | 144 KiB | 1s | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.5.5
bpaf_derive | 144 KiB | 9s | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.5.5
clap | 695 KiB | 6s | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v3.2.20
clap-minimal | 646 KiB | 5s | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v3.2.20
clap_derive | 733 KiB | 12s | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v3.2.20
clap_lex | 37 KiB | 799ms | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/clap_lex) | v0.2.4
gumdrop | 33 KiB | 4s | 3ms | N | ![Download count](https://img.shields.io/crates/dr/gumdrop) | v0.8.1
lexopt | 36 KiB | 615ms | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/lexopt) | v0.2.1
pico-args | 24 KiB | 600ms | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/pico-args) | v0.5.0
xflags | 24 KiB | 1s | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/xflags) | v0.2.4

*System: Linux 5.4.0-104-generic (x86_64) w/ `-j 8`*

*rustc: rustc 1.63.0 (4b91a6ea7 2022-08-08)*

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
