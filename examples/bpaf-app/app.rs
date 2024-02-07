use std::path::PathBuf;

use bpaf::{construct, long, positional, Parser};

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
        .argument::<u32>("NUMBER");
    let opt_number = long("opt-number")
        .help("Sets an optional number")
        .argument::<u32>("OPT-NUMBER")
        .optional();
    let width = long("width")
        .help("Sets width")
        .argument::<String>("WIDTH")
        .parse(as_width)
        .fallback(10);
    let input = positional::<PathBuf>("INPUT").many();

    let parser = construct!(AppArgs {
        number,
        opt_number,
        width,
        input
    })
    .to_options()
    .descr("App");

    let args = parser.run();

    #[cfg(debug_assertions)]
    {
        println!("{:#?}", args.number);
        println!("{:#?}", args.opt_number);
        println!("{:#?}", args.width);
        if 10 < args.input.len() {
            println!("{:#?}", args.input.len());
        } else {
            println!("{:#?}", args);
        }
    }
    std::hint::black_box(args);
}
