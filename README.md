# Rust Arg Parsing Benchmarks

This repo tries to assess Rust arg parsing performance.

We currently compare:

## Feature description

- Help - automatically generate app usage with all the arguments and show it on `--help`
- Color - use ANSI codes to add styles in generated help or error messages
- Fixes - detect possible typos and suggest fixes
- Completion - generate autocompletion scripts from parser definition. Static completion allows
  to complete flag and command names, dynamic completions allows to complete possible values as
  well
- Documentation - generate user documentation in markdown (for web usage) or man (for \*nix
  console)

Name                                                 | Style                 | Help | Color | Fixes | Comp S, D | Doc md, man |
-----------------------------------------------------|-----------------------|------|-------|-------|-----------|-------------|
[argh](https://github.com/google/argh)               | `derive`              | :heavy_check_mark: | :x: | :x: | :x:, :x: | :x:, :x: |
[bpaf](https://github.com/pacak/bpaf)             | Combinatoric or `derive` |  :heavy_check_mark:   | :heavy_check_mark:     | :heavy_check_mark:     | :x:, :heavy_check_mark:      | :heavy_check_mark:, :heavy_check_mark:        |
[clap_lex](https://github.com/clap-rs/clap)          | Imperative            | :x:    | :x:     | :x:     | :x:, :x:      | :x:, :x:        |
[clap](https://github.com/clap-rs/clap)              | Builder or `derive`   | :heavy_check_mark:    | :heavy_check_mark:     | :heavy_check_mark:     | :heavy_check_mark:, :x:      | :heavy_check_mark:, :heavy_check_mark:        |
[gumdrop](https://github.com/murarth/gumdrop)        | `derive`              | :heavy_check_mark:    | :x:     | :x:     | :x:, :x:      | :x:, :x:        |
[lexopt](https://github.com/blyxxyz/lexopt)          | Imperative            | :x:    | :x:     | :x:     | :x:, :x:      | :x:, :x:        |
[pico-args](https://github.com/razrfalcon/pico-args) | Imperative            | :x:    | :x:     | :x:     | :x:, :x:      | :x:, :x:        |
[xflags](https://github.com/matklad/xflags)          | proc-macro            | :heavy_check_mark:    | :x:     | :x:     | :x:, :x:      | :x:, :x:        |

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
