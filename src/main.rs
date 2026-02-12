use kanttiinit::{args::parse, commands::handle_arg};
fn main() {
    if let Err(e) = handle_arg(parse(std::env::args_os())) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
