use clap::Parser;

#[derive(Parser, Debug)]
struct AppArgs {
    /// Sets a number.
    #[clap(long, action)]
    number: u32,

    /// Sets an optional number.
    #[clap(long, action)]
    opt_number: Option<u32>,

    /// Sets width.
    #[clap(long, default_value = "10", value_parser = parse_width)]
    width: u32,

    #[clap(action)]
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
    let args = AppArgs::parse();
    println!("{:#?}", args.number);
    println!("{:#?}", args.opt_number);
    println!("{:#?}", args.width);
    if 10 < args.input.len() {
        println!("{:#?}", args.input.len());
    } else {
        println!("{:#?}", args);
    }
}
