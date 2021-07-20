# Rust Arg Parsing Benchmarks

This repo tries to assess Rust arg parsing performance.

We currently compare:

Name                                                 | Style      | Notes
-----------------------------------------------------|------------|------
No-op                                                | N/A        | N/A
[argh](https://github.com/google/argh)               | Derive     | 
[clap](https://github.com/clap-rs/clap)              | Procedural | Color, suggested fixes, completions
[gumpdrop](https://github.com/murarth/gumdrop)       | Derive     |
[pico-args](https://github.com/razrfalcon/pico-args) | Procedural | No help generation
[structopt](https://github.com/texitoi/structopt)    | Derive     | Color, suggested fixes, completions

**For a full comparison, see each parser docs**
