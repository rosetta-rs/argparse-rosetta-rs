# Rust Arg Parsing Benchmarks

This repo tries to assess Rust arg parsing performance.

We currently compare:

Name                                                 | Style                 | Notes
-----------------------------------------------------|-----------------------|------
No-op                                                | N/A                   | N/A
[argh](https://github.com/google/argh)               | `derive`              |
[bpaf](https://github.com/pacak/bpaf)                | Imperative            |
[clap](https://github.com/clap-rs/clap)              | Imperative or `derive`| Color, suggested fixes, completions
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
null | 0 KiB | 578ms | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/None) | -
argh | 30 KiB | 5s | 1ms | N | ![Download count](https://img.shields.io/crates/dr/argh) | v0.1.7
bpaf | 140 KiB | 1s | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.3.0
clap | 625 KiB | 4s | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v3.1.3
clap-minimal | 578 KiB | 4s | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v3.1.3
clap_derive | 631 KiB | 9s | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v3.1.3
gumdrop | 24 KiB | 4s | 1ms | N | ![Download count](https://img.shields.io/crates/dr/gumdrop) | v0.8.0
lexopt | 27 KiB | 809ms | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/lexopt) | v0.2.0
pico-args | 23 KiB | 769ms | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/pico-args) | v0.4.2
xflags | 23 KiB | 1s | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/xflags) | v0.2.3

*System: Linux 5.4.0-104-generic (x86_64) w/ `-j 12`*

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
