// parse.rs

use std::env;
use crate::locale;

pub enum ParseResult {
	Ok(VarsApp),
	ShowHelp(String),			// message localisé
	ShowVersion(String),		// message localisé
	Error(String, String, u8),	// erreur + usage localisé + code
}
#[derive(Debug)]
pub struct VarsApp {
	pub locale: locale::LangStrings,
	pub livre_name: Option<String>,
}

impl VarsApp {
	pub fn parse_args(prg_name: &str, version: &str) -> ParseResult {
		let args: Vec<String> = env::args().skip(1).collect();

		let mut opts = VarsApp {
			locale: locale::set_lang_vec(),
			livre_name: None,
		};

		if args.len() > 1 {
			return ParseResult::Error(
				opts.locale.err_arguments.to_string(),
				format!("-------\nusage: {prg_name} {}", opts.locale.options),
				5,
			);
		}

		for arg in args.iter() {
			match arg.as_str() {
				"-h" => return ParseResult::ShowHelp(format!("usage: {prg_name} {}", opts.locale.usage)),
				"--help" => return ParseResult::ShowHelp(format!("usage: {prg_name} {}", opts.locale.options)),
				"-ver" => return ParseResult::ShowVersion(format!("{prg_name}{} {}", opts.locale.ver, version)),
				"--version" => return ParseResult::ShowVersion(format!("{prg_name}{} {}", opts.locale.ver_desc, version)),
				_ => {	if opts.livre_name.is_none() {
							opts.livre_name = Some(arg.clone());
					}
				}
			}
		}
		ParseResult::Ok(opts)
	}
}
