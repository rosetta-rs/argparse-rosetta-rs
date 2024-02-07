fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    #[cfg(debug_assertions)]
    {
        if 10 < args.len() {
            println!("{:#?}", args.len());
        } else {
            println!("{:#?}", args);
        }
    }
    std::hint::black_box(args);
}
