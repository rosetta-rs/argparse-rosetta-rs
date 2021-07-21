# Rust Arg Parsing Benchmarks

This repo tries to assess Rust arg parsing performance.

We currently compare:

Name                                                 | Style          | Notes
-----------------------------------------------------|----------------|------
No-op                                                | N/A            | N/A
[argh](https://github.com/google/argh)               | `derive`       |
[clap](https://github.com/clap-rs/clap)              | Imperative     | Color, suggested fixes, completions
[gumpdrop](https://github.com/murarth/gumdrop)       | `derive`       |
[lexopt](https://github.com/murarth/gumdrop)         | Imperative     | No help generation
[pico-args](https://github.com/razrfalcon/pico-args) | Imperative     | No help generation
[structopt](https://github.com/texitoi/structopt)    | `derive`       | Color, suggested fixes, completions
[xflags](https://github.com/matklad/xflags)          | proc-macro     |

**For a full comparison, see each parser docs**

# Results

Crate | Overhead (release) | Build (debug) | Deps | Version
------|--------------------|---------------|------|--------
null | 0.0 KiB | 1.2s | 0 | -
argh | 44.3 KiB | 12.6s | 8 | v0.1.5
clap | 641.5 KiB | 9.3s | 8 | v2.33.3
gumdrop | 38.4 KiB | 12.6s | 5 | v0.8.0
lexopt | 36.2 KiB | 1.9s | 0 | v0.1.0
pico-args | 28.9 KiB | 2.0s | 0 | v0.4.2
structopt | 642.3 KiB | 19.2s | 20 | v0.3.22
xflags | 29.4 KiB | 5.6s | 3 | v0.2.2

**System: Linux 4.4.0-19041-Microsoft (x86_64)**

# Running the Benchmarks

```bash
$ ./bench.py
$ ./format.py
```

# Special Thanks

- RazrFalcon for creating the [initial benchmarks](https://github.com/RazrFalcon/pico-args)
- djc for inspiration with [template-benchmarks-rs](https://github.com/djc/template-benchmarks-rs)
- sharkdp for [hyperfine](https://github.com/sharkdp/hyperfine)
