use clap::{App, Arg};

#[derive(Debug)]
struct AppArgs {
    help: bool,
    number: u32,
    opt_number: Option<u32>,
    width: u32,
    input: Vec<std::path::PathBuf>,
}

fn is_width(s: &str) -> Result<(), String> {
    let w: u32 = s.parse().map_err(|_| "not a number")?;
    if w != 0 {
        Ok(())
    } else {
        Err("width must be positive".to_string())
    }
}

fn main() {
    let matches = App::new("App")
        .arg(
            Arg::new("number")
                .long("number")
                .required(true)
                .about("Sets a number")
                .takes_value(true),
        )
        .arg(
            Arg::new("opt-number")
                .long("opt-number")
                .about("Sets an optional number")
                .takes_value(true),
        )
        .arg(
            Arg::new("width")
                .long("width")
                .default_value("10")
                .validator(is_width)
                .about("Sets width")
                .takes_value(true),
        )
        .arg(Arg::new("INPUT").takes_value(true).multiple_values(true))
        .get_matches();

    let args = AppArgs {
        help: matches.is_present("help"),
        number: matches.value_of_t("number").unwrap(),
        opt_number: matches.value_of_t("opt-number").ok(),
        width: matches.value_of_t("width").unwrap(),
        input: matches
            .values_of_os("INPUT")
            .unwrap()
            .map(|s| s.into())
            .collect(),
    };

    if 10 < args.input.len() {
        println!("{:#?}", args.input.len());
    } else {
        println!("{:#?}", args);
    }
}
