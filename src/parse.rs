// parse.rs

use std::env;
use crate::locale;

#[derive(Debug)]
pub struct Options {
	pub locale: locale::LangStrings,
	pub livre_name: Option<String>,
}

impl Options {
	pub fn parse_args(prg_name: &str, version: &str) -> Self {
		let args: Vec<String> = env::args().skip(1).collect();

		let mut opts = Options {
			locale: locale::set_lang_vec(),
			livre_name: None,
		};

		if args.iter().count() > 1 {
			eprintln!("{}", opts.locale.err_arguments);
			std::process::exit(5);
		}
		for arg in args.iter() {
			match arg.as_str() {
				"-h" => help(prg_name, opts.locale.usage, 0),
				"-ver" => versions(prg_name, version, opts.locale.ver),
				"--help" => help(prg_name, opts.locale.options, 0),
				"--version" => versions(prg_name, version, opts.locale.ver_desc),
				_ => {
					// Si ce n’est pas une option, on considère que c’est le nom du livre
					if opts.livre_name.is_none() {
						opts.livre_name = Some(arg.clone());
					}
				}
			}
		}
		opts
	}
}

fn help(prg_name: &str, loc_string: &str, ecode: i32) {
	println!("usage: {prg_name} {loc_string}");
	std::process::exit(ecode);
}

fn versions(prg_name: &str, version: &str, loc_string: &str) {
	println!("{prg_name}{loc_string} {version}");
	std::process::exit(0);
}
