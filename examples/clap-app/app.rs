use clap::{value_parser, Arg, Command};

#[derive(Debug)]
struct AppArgs {
    number: u32,
    opt_number: Option<u32>,
    width: u32,
    input: Vec<std::path::PathBuf>,
}

fn parse_width(s: &str) -> Result<u32, String> {
    let w = s.parse().map_err(|_| "not a number")?;
    if w != 0 {
        Ok(w)
    } else {
        Err("width must be positive".to_string())
    }
}

fn main() {
    let matches = Command::new("App")
        .arg(
            Arg::new("number")
                .long("number")
                .required(true)
                .help("Sets a number")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            Arg::new("opt-number")
                .long("opt-number")
                .help("Sets an optional number")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            Arg::new("width")
                .long("width")
                .default_value("10")
                .value_parser(parse_width)
                .help("Sets width"),
        )
        .arg(
            Arg::new("INPUT")
                .num_args(1..)
                .value_parser(value_parser!(std::path::PathBuf)),
        )
        .get_matches();

    let args = AppArgs {
        number: *matches.get_one::<u32>("number").unwrap(),
        opt_number: matches.get_one::<u32>("opt-number").cloned(),
        width: matches.get_one::<u32>("width").cloned().unwrap(),
        input: matches
            .get_many::<std::path::PathBuf>("INPUT")
            .unwrap_or_default()
            .cloned()
            .collect(),
    };

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
