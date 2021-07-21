# Rust Arg Parsing Benchmarks

This repo tries to assess Rust arg parsing performance.

We currently compare:

Name                                                 | Style          | Notes
-----------------------------------------------------|----------------|------
No-op                                                | N/A            | N/A
[argh](https://github.com/google/argh)               | `derive`       |
[clap](https://github.com/clap-rs/clap)              | Imperative     | Color, suggested fixes, completions
[gumpdrop](https://github.com/murarth/gumdrop)       | `derive`       |
[lexopt](https://github.com/blyxxyz/lexopt)          | Imperative     | No help generation
[pico-args](https://github.com/razrfalcon/pico-args) | Imperative     | No help generation
[structopt](https://github.com/texitoi/structopt)    | `derive`       | Color, suggested fixes, completions
[xflags](https://github.com/matklad/xflags)          | proc-macro     |

*Note: any non-performanve comparison is meant to provide context for what you
gain/lose with each crate's overhead.  For a full comparison, see each parser
docs*

# Results

Crate | Overhead (release) | Build (debug) | Deps | Version
------|--------------------|---------------|------|--------
null | 0 KiB | 1s | 0 | -
argh | 44 KiB | 13s | 8 | v0.1.5
clap | 642 KiB | 9s | 8 | v2.33.3
gumdrop | 38 KiB | 13s | 5 | v0.8.0
lexopt | 36 KiB | 2s | 0 | v0.1.0
pico-args | 29 KiB | 2s | 0 | v0.4.2
structopt | 642 KiB | 19s | 20 | v0.3.22
xflags | 29 KiB | 6s | 3 | v0.2.2

*System: Linux 4.4.0-19041-Microsoft (x86_64)*

# Running the Benchmarks

```bash
$ ./bench.py
$ ./format.py
```

# Special Thanks

- RazrFalcon for creating the [initial benchmarks](https://github.com/RazrFalcon/pico-args)
- djc for inspiration with [template-benchmarks-rs](https://github.com/djc/template-benchmarks-rs)
- sharkdp for [hyperfine](https://github.com/sharkdp/hyperfine)
