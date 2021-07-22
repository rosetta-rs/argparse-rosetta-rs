fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    if 10 < args.len() {
        println!("{:#?}", args.len());
    } else {
        println!("{:#?}", args);
    }
}
