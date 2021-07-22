fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    println!("{:#?}", args);
}
