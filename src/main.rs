use kanttiinit::{args::parse, commands::handle_arg};
fn main() {
    let args = parse(std::env::args_os());
    handle_arg(args);
}
