# Rust Arg Parsing Benchmarks

This repo tries to assess Rust arg parsing performance.

We currently compare:

Name                                                 | Style                 | Notes
-----------------------------------------------------|-----------------------|------
No-op                                                | N/A                   | N/A
[argh](https://github.com/google/argh)               | `derive`              |
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

Name | Overhead (release) | Build (debug) | Parse (release) | Deps | Invalid UTF-8 | Downloads | Version
-----|--------------------|---------------|-----------------|------|---------------|-----------|--------
null | 0 KiB | 735ms | 1ms | 0 | Y | ![Download count](https://img.shields.io/crates/dr/None) | -
argh | 30 KiB | 6s | 1ms | 8 | N | ![Download count](https://img.shields.io/crates/dr/argh) | v0.1.6
clap | 623 KiB | 6s | 2ms | 8 | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v2.33.3
clap-minimal | 614 KiB | 6s | 2ms | 7 | N | ![Download count](https://img.shields.io/crates/dr/clap) | v3.0.0-beta.5
clap3 | 718 KiB | 15s | 1ms | 24 | N | ![Download count](https://img.shields.io/crates/dr/clap) | v3.0.0-beta.5
clap_derive | 709 KiB | 15s | 2ms | 24 | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v3.0.0-beta.5
gumdrop | 24 KiB | 6s | 1ms | 5 | N | ![Download count](https://img.shields.io/crates/dr/gumdrop) | v0.8.0
lexopt | 26 KiB | 1s | 1ms | 0 | Y | ![Download count](https://img.shields.io/crates/dr/lexopt) | v0.1.0
pico-args | 23 KiB | 1s | 2ms | 0 | Y | ![Download count](https://img.shields.io/crates/dr/pico-args) | v0.4.2
structopt | 623 KiB | 11s | 2ms | 20 | Y | ![Download count](https://img.shields.io/crates/dr/structopt) | v0.3.25
xflags | 23 KiB | 2s | 1ms | 1 | Y | ![Download count](https://img.shields.io/crates/dr/xflags) | v0.2.3

*System: Linux 5.4.0-84-generic (x86_64) w/ `-j 8`*

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
