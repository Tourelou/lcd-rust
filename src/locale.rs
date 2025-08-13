// locale.rs

use std::env;

const OPTIONS_FR: &str =
r#"['fichier Livre de compte']

  -ver, --version     Livre comptable en CLI.
  -h,   --help        Montre ce message d'aide et termine.
  
  fichier Livre de compte:
    Par défaut sans argument, il s'appelle LivreComptable.xxxx
    où xxxx sont les 4 digits de l'année courante.

    Si un paramètre est donné:
        Exemples: lcd compte, compte sera sauvé à côté de l'exécutable.
                  lcd ./compte, compte sera sauvé dans le répertoire courant.
                  lcd /home/user/compte, compte sera sauvé vers ce path."#;

const OPTIONS_ES: &str =
r#"['archivo Libro de cuentas']

  -ver, --version     Libro contable en CLI.
  -h,   --help        Muestra este mensaje de ayuda y termina.
  
  archivo Libro de cuentas:
    Por defecto, sin argumento, se llama LLivreComptable.xxxx
    donde xxxx son los 4 dígitos del año actual.

    Si se da un parámetro:
        Ejemplos: lcd cuenta, cuenta se guardará junto al ejecutable.
                  lcd ./cuenta, cuenta se guardará en el directorio actual.
                  lcd /home/user/cuenta, cuenta se guardará en esa ruta."#;

const OPTIONS_EN: &str =
r#"['accounting book file']

  -ver, --version     Accounting book in CLI.
  -h,   --help        Shows this help message and exits.
  
  accounting book file:
    By default, without argument, it's named LivreComptable.xxxx
    where xxxx are the 4 digits of the current year.

    If a parameter is given:
        Examples: lcd account, account will be saved next to the executable.
                  lcd ./account, account will be saved in the current directory.
                  lcd /home/user/account, account will be saved to that path."#;

#[derive(Debug)]
pub struct LangStrings {
	pub usage: &'static str,
	pub options: &'static str,
	pub ver: &'static str,
	pub ver_desc: &'static str,
	pub err_arguments: &'static str,
}

pub const LANG_FR: LangStrings = LangStrings {
	usage: "['Livre de compte']",
	options: OPTIONS_FR,
	ver: ": version",
	ver_desc: ": Livre comptable en ligne de commande, version",
	err_arguments: "Trop d'argument: un seul fichier LivreComptable s.v.p."
};

pub const LANG_ES: LangStrings = LangStrings {
    usage: "['Libro de cuentas']",
    options: OPTIONS_ES,
    ver: ": versión",
    ver_desc: ": Libro contable en línea de comandos, versión",
	err_arguments: "Demasiados argumentos: Solo un archivo 'LivreComptable', por favor.",
};

pub const LANG_EN: LangStrings = LangStrings {
    usage: "['Accounting book']",
    options: OPTIONS_EN,
    ver: ": version",
    ver_desc: ": Accounting book in command line interface, version",
	err_arguments: "Too many arguments: Only one 'LivreComptable' file please.",
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
