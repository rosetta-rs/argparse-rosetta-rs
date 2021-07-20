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

Crate | null | argh | clap | gumdrop | lexopt | pico-args | structopt | xflags
--|---|---|---|---|---|---|---|--
Binary Overhead (release) | 0.0 KiB | 44.3 KiB | 641.5 KiB | 38.4 KiB | 36.2 KiB | 28.9 KiB | 642.3 KiB | 29.4 KiB
Build Time (debug) | 1.2 | 12.6 | 9.3 | 12.6 | 1.9 | 2.0 | 19.2 | 5.6
Dependencies | 0 | 8 | 8 | 5 | 0 | 0 | 20 | 3
Version | - | v0.1.5 | v2.33.3 | v0.8.0 | v0.1.0 | v0.4.2 | v0.3.22 | v0.2.2

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
