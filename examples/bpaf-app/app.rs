use bpaf::*;

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct AppArgs {
    number: u32,
    opt_number: Option<u32>,
    width: u32,
    input: Vec<std::path::PathBuf>,
}

fn main() {
    let number = long("number")
        .help("Sets a number")
        .argument("number")
        .from_str();

    let opt_number = long("opt-number")
        .help("Sets an optional number")
        .argument("opt-number")
        .from_str()
        .optional();

    let width = long("width")
        .help("Sets width")
        .argument("width")
        .from_str()
        .guard(|n: &u32| *n > 0, "Width must be positive")
        .fallback(10);

    let input = positional_os("INPUT").map(std::path::PathBuf::from).many();

    let parser = construct!(AppArgs {
        number,
        opt_number,
        width,
        input
    });

    let args = Info::default().for_parser(parser).run();

    if 10 < args.input.len() {
        println!("{:#?}", args.input.len());
    } else {
        println!("{:#?}", args);
    }
}
