type BoxedError = Box<dyn std::error::Error + Send + Sync>;

const HELP: &str = "\
USAGE: app [OPTIONS] --number NUMBER INPUT..

OPTIONS:
  --number NUMBER       Set a number (required)
  --opt-number NUMBER   Set an optional number
  --width WIDTH         Set a width (non-zero, default 10)

ARGS:
  <INPUT>               Input file
";

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
    let args = match parse_args() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Error: {}.", err);
            std::process::exit(1);
        }
    };
    println!("{:#?}", args.number);
    println!("{:#?}", args.opt_number);
    println!("{:#?}", args.width);
    if 10 < args.input.len() {
        println!("{:#?}", args.input.len());
    } else {
        println!("{:#?}", args);
    }
}

fn parse_args() -> Result<AppArgs, BoxedError> {
    let mut number = None;
    let mut opt_number = None;
    let mut width = 10;
    let mut input = Vec::new();

    let raw = clap_lex::RawArgs::from_args();
    let mut cursor = raw.cursor();
    while let Some(arg) = raw.next(&mut cursor) {
        if arg.is_escape() {
            input.extend(raw.remaining(&mut cursor).map(std::path::PathBuf::from));
        } else if arg.is_stdio() {
            input.push(std::path::PathBuf::from("-"));
        } else if let Some((long, value)) = arg.to_long() {
            match long {
                Ok("help") => {
                    print!("{}", HELP);
                    std::process::exit(0);
                }
                Ok("number") => {
                    let value = if let Some(value) = value {
                        value.to_str().ok_or_else(|| {
                            format!("Value `{}` is not a number", value.to_string_lossy())
                        })?
                    } else {
                        let value = raw
                            .next_os(&mut cursor)
                            .ok_or_else(|| format!("`--number` is missing a value"))?;
                        value.to_str().ok_or_else(|| {
                            format!("Value `{}` is not a number", value.to_string_lossy())
                        })?
                    };
                    number = Some(value.parse()?);
                }
                Ok("opt-number") => {
                    let value = if let Some(value) = value {
                        value.to_str().ok_or_else(|| {
                            format!("Value `{}` is not a number", value.to_string_lossy())
                        })?
                    } else {
                        let value = raw
                            .next_os(&mut cursor)
                            .ok_or_else(|| format!("`--number` is missing a value"))?;
                        value.to_str().ok_or_else(|| {
                            format!("Value `{}` is not a number", value.to_string_lossy())
                        })?
                    };
                    opt_number = Some(value.parse()?);
                }
                Ok("width") => {
                    let value = if let Some(value) = value {
                        value.to_str().ok_or_else(|| {
                            format!("Value `{}` is not a number", value.to_string_lossy())
                        })?
                    } else {
                        let value = raw
                            .next_os(&mut cursor)
                            .ok_or_else(|| format!("`--number` is missing a value"))?;
                        value.to_str().ok_or_else(|| {
                            format!("Value `{}` is not a number", value.to_string_lossy())
                        })?
                    };
                    width = parse_width(value)?;
                }
                _ => {
                    return Err(format!("Unexpected flag: --{}", arg.display()).into());
                }
            }
        } else if let Some(mut shorts) = arg.to_short() {
            while let Some(short) = shorts.next_flag() {
                match short {
                    Ok('h') => {
                        print!("{}", HELP);
                        std::process::exit(0);
                    }
                    Ok(c) => {
                        return Err(format!("Unexpected flag: -{}", c).into());
                    }
                    Err(e) => {
                        return Err(format!("Unexpected flag: -{}", e.to_string_lossy()).into());
                    }
                }
            }
        } else {
            input.push(std::path::PathBuf::from(arg.to_value_os().to_owned()));
        }
    }

    Ok(AppArgs {
        number: number.ok_or("missing required option --number".to_owned())?,
        opt_number,
        width,
        input,
    })
}
