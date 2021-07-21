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

## Special Thanks

- RazrFalcon for creating the [initial benchmarks](https://github.com/RazrFalcon/pico-args)
- djc for inspiration with [template-benchmarks-rs](https://github.com/djc/template-benchmarks-rs)
- sharkdp for [hyperfine](https://github.com/sharkdp/hyperfine)
