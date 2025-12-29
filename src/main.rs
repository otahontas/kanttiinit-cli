use kanttiinit::{args::parse, commands::handle_arg};
fn main() {
    handle_arg(parse(std::env::args_os()));
}
