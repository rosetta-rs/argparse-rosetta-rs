use std::path::PathBuf;

use bpaf::{construct, long, positional_os, Info, Parser};

#[derive(Debug, Clone)]
struct AppArgs {
    number: u32,
    opt_number: Option<u32>,
    width: u32,
    input: Vec<std::path::PathBuf>,
}

fn as_width(s: String) -> Result<u32, String> {
    let w: u32 = s.parse().map_err(|_| "not a number")?;
    if w != 0 {
        Ok(w)
    } else {
        Err("width must be positive".to_string())
    }
}

fn main() {
    let number = long("number")
        .help("Sets a number")
        .argument("NUMBER")
        .from_str::<u32>();
    let opt_number = long("opt-number")
        .help("Sets an optional number")
        .argument("OPT-NUMBER")
        .from_str()
        .optional();
    let width = long("width")
        .help("Sets width")
        .argument("WIDTH")
        .parse(as_width)
        .fallback(10);
    let input = positional_os("INPUT").map(PathBuf::from).many();

    let parser = construct!(AppArgs {
        number,
        opt_number,
        width,
        input
    });
    let args = Info::default().descr("App").for_parser(parser).run();

    if 10 < args.input.len() {
        println!("{:#?}", args.input.len());
    } else {
        println!("{:#?}", args);
    }
}
