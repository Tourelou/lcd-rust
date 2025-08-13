// locale.rs

use std::env;

const OPTIONS_FR: &str =
r#"[-f|-d] [-riInv] <motif regex> <remplacement> [dirname ...]

  -ver, --version     Renommage multiple à partir d'un motif.
  -h,   --help        Montre ce message d'aide et termine."#;

const OPTIONS_ES: &str =
r#"[-f|-d] [-riInv] <patrón regex> <sustitución> [dirname ...]

  -ver, --version     Cambio de nombre múltiple a partir de un patrón.
  -h,   --help        Muestra este mensaje de ayuda y finaliza."#;

const OPTIONS_EN: &str =
r#"[-f|-d] [-riInv] <regex pattern> <remplacement> [dirname ...]

  -ver, --version     Multiple renaming from a pattern.
  -h,   --help        Show this help message and exit."#;

#[derive(Debug)]
pub struct LangStrings {
	pub usage: &'static str,
	pub options: &'static str,
	pub ver: &'static str,
	pub ver_desc: &'static str,
}

pub const LANG_FR: LangStrings = LangStrings {
	usage: "[-f|-d] [-riInv] <motif regex> <remplacement> [dirname ...]",
	options: OPTIONS_FR,
	ver: ": version",
	ver_desc: ": Renommage multiple selon un certain motif, version",
};

pub const LANG_ES: LangStrings = LangStrings {
	usage: "[-f|-d] [-riInv] <patrón de expresión regular> <reemplazo> [nombredirectorio ...]",
	options: OPTIONS_ES,
	ver: ": versión",
	ver_desc: ": Cambio de nombre múltiple basado en un patrón determinado, versión",
};

pub const LANG_EN: LangStrings = LangStrings {
	usage: "[-f|-d] [-riInv] <regex pattern> <replacement> [dirname ...]",
	options: OPTIONS_EN,
	ver: ": version",
	ver_desc: ": Multiple renaming based on a certain pattern, version",
};

pub fn set_lang_vec() -> LangStrings {
	match get_system_lang().as_str() {
		"fr" => LANG_FR,
		"es" => LANG_ES,
		_ => LANG_EN,
	}
}

fn get_system_lang() -> String {
	let raw_lang = std::env::var("LC_ALL")
		.or_else(|_| env::var("LANG"))
		.or_else(|_| env::var("LANGUAGE"))
		.unwrap_or_else(|_| "en".to_string()); // Langue par défaut (anglais)

	// Extraire uniquement le code de langue avant le premier '_'
	let lang_code = raw_lang.split('_').next().unwrap_or(&raw_lang);
	lang_code.to_string() // Retourne "fr" au lieu de "fr_CA.UTF-8"
}
