// main.rs

mod parse;
mod locale;

const PRGNAME: &str = "prg";
const VERSION: &str = "2025-08-13";
fn main() {
	let opts = parse::Options::parse_args(PRGNAME, VERSION);
	println!("{:#?}", opts);
	println!("{}", opts.livre_name.unwrap());
}
