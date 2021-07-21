use clap::Clap;

#[derive(Clap, Debug)]
struct AppArgs {
    /// Sets a number.
    #[clap(long)]
    number: u32,

    /// Sets an optional number.
    #[clap(long)]
    opt_number: Option<u32>,

    /// Sets width.
    #[clap(long, default_value = "10", parse(try_from_str = parse_width))]
    width: u32,

    #[clap(parse(from_os_str))]
    input: std::path::PathBuf,
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
    println!("{:#?}", args);
}
