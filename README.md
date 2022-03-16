# Rust Arg Parsing Benchmarks

This repo tries to assess Rust arg parsing performance.

We currently compare:

Name                                                 | Style                 | Notes
-----------------------------------------------------|-----------------------|------
No-op                                                | N/A                   | N/A
[argh](https://github.com/google/argh)               | `derive`              |
[bpaf](https://github.com/pacak/bpaf)                | Combinator            |
[clap](https://github.com/clap-rs/clap)              | Builder or `derive`   | Color, suggested fixes, completions
[gumpdrop](https://github.com/murarth/gumdrop)       | `derive`              |
[lexopt](https://github.com/blyxxyz/lexopt)          | Imperative            | No help generation
[pico-args](https://github.com/razrfalcon/pico-args) | Imperative            | No help generation
[structopt](https://github.com/texitoi/structopt)    | `derive`              | Color, suggested fixes, completions
[xflags](https://github.com/matklad/xflags)          | proc-macro            |

*Note: any non-performanve comparison is meant to provide context for what you
gain/lose with each crate's overhead.  For a full comparison, see each parser
docs*

# Results

Name | Overhead (release) | Build (debug) | Parse (release) | Invalid UTF-8 | Downloads | Version
-----|--------------------|---------------|-----------------|---------------|-----------|--------
null | 0 KiB | 639ms | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/None) | -
argh | 37 KiB | 5s | 3ms | N | ![Download count](https://img.shields.io/crates/dr/argh) | v0.1.7
bpaf | 145 KiB | 1s | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.3.0
clap | 591 KiB | 6s | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v3.1.6
clap-minimal | 549 KiB | 5s | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v3.1.6
clap_derive | 588 KiB | 12s | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v3.1.6
gumdrop | 32 KiB | 5s | 4ms | N | ![Download count](https://img.shields.io/crates/dr/gumdrop) | v0.8.1
lexopt | 35 KiB | 866ms | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/lexopt) | v0.2.0
pico-args | 31 KiB | 867ms | 4ms | Y | ![Download count](https://img.shields.io/crates/dr/pico-args) | v0.4.2
xflags | 22 KiB | 1s | 3ms | Y | ![Download count](https://img.shields.io/crates/dr/xflags) | v0.2.4

*System: Linux 5.4.0-104-generic (x86_64) w/ `-j 8`*

Notes:
- Overhead will be lower if your application shares dependencies with your argument parsing library.

# Running the Benchmarks

```bash
$ ./bench.py
$ ./format.py
```

# Special Thanks

- RazrFalcon for creating the [initial benchmarks](https://github.com/RazrFalcon/pico-args)
- djc for inspiration with [template-benchmarks-rs](https://github.com/djc/template-benchmarks-rs)
- sharkdp for [hyperfine](https://github.com/sharkdp/hyperfine)
