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
null | 0 KiB | 163ms *(full)* <br/>115ms *(incremental)* | 1ms | Y | - | -
argh | 38 KiB | 4s *(full)* <br/>138ms *(incremental)* | 2ms | N | ![Download count](https://img.shields.io/crates/dr/argh) | v0.1.14
bpaf | 256 KiB | 831ms *(full)* <br/>137ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.9.23
bpaf_derive | 253 KiB | 4s *(full)* <br/>140ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.9.23
clap | 574 KiB | 3s *(full)* <br/>170ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.5.60
clap-minimal | 377 KiB | 2s *(full)* <br/>155ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.5.60
clap_derive | 596 KiB | 4s *(full)* <br/>182ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.5.60
clap_lex | 28 KiB | 336ms *(full)* <br/>129ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/clap_lex) | v1.0.0
gumdrop | 28 KiB | 2s *(full)* <br/>135ms *(incremental)* | 1ms | N | ![Download count](https://img.shields.io/crates/dr/gumdrop) | v0.8.1
lexopt | 37 KiB | 329ms *(full)* <br/>124ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/lexopt) | v0.3.2
pico-args | 24 KiB | 300ms *(full)* <br/>123ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/pico-args) | v0.5.0
xflags | 23 KiB | 539ms *(full)* <br/>123ms *(incremental)* | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/xflags) | v0.3.2

*System: Linux 6.17.9-76061709-generic (x86_64), rustc 1.94.0 (4a4ef493e 2026-03-02) w/ `-j 8`*

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
