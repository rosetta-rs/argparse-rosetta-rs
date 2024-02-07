use bpaf::Bpaf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
/// App
struct AppArgs {
    /// Sets a number
    #[bpaf(argument("NUMBER"))]
    number: u32,

    /// Sets an optional number
    #[bpaf(argument("OPT-NUMBER"))]
    opt_number: Option<u32>,

    /// Sets width
    #[bpaf(
        argument("WIDTH"),
        guard(valid_width, "width must be positive"),
        fallback(10)
    )]
    width: u32,

    #[bpaf(positional("INPUT"))]
    input: Vec<std::path::PathBuf>,
}

fn valid_width(width: &u32) -> bool {
    *width > 0
}

fn main() {
    let args = app_args().run();

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
