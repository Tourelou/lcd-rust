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

const MSG_TERM_112_FR: &str =
r#"Redimentionnez le terminal.

Un minimum de 112 colonnes sont nécessaire à l'affichage.

Appuyez sur Entrée pour une deuxième chance... "#;

const  MSG_TERM_112_ES: &str =
r#"Ajusta el tamaño de la terminal.

Se requiere un mínimo de 112 columnas para la visualización.

Presiona Intro para una segunda oportunidad... "#;

const  MSG_TERM_112_EN: &str = 
r#"Resize the terminal.

A minimum of 112 columns are required for the display.

Press Enter for a second chance... "#;

const HEADER_FR: &str =
r#"--------------------------------------------------------
--- {1}, par Daniel Vaillancourt, version {2} ---
--------------------------------------------------------"#;

const HEADER_ES: &str =
r#"--------------------------------------------------------
--- {1}, por Daniel Vaillancourt, versión {2} ---
--------------------------------------------------------"#;

const HEADER_EN: &str =
r#"--------------------------------------------------------
--- {1},  by Daniel Vaillancourt, version {2} ---
--------------------------------------------------------"#;

#[derive(Debug)]
pub struct LangStrings {
	pub usage: &'static str,
	pub options: &'static str,
	pub ver: &'static str,
	pub ver_desc: &'static str,
	pub term_size: &'static str,
	pub err_arguments: &'static str,
	pub err_chdir: &'static str,
	pub err_is_dir: &'static str,
	pub header: &'static str,
	pub new_db: &'static str,
	pub ouverture: &'static str,
}

pub const LANG_FR: LangStrings = LangStrings {
	usage: "['Livre de compte']",
	options: OPTIONS_FR,
	ver: ": version",
	ver_desc: ": Livre comptable en ligne de commande, version",
	term_size: MSG_TERM_112_FR,
	err_arguments: "Erreur: Trop d'argument: un seul fichier LivreComptable s.v.p.",
	err_chdir: "Erreur changement de répertoire vers",
	err_is_dir: "est un répertoire",
	header: HEADER_FR,
	new_db: "n'existe pas.\n\nCréation d'un nouveau livre ?",
	ouverture: "Ouverture du livre «{1}» à partir du répertoire «{2}»",
};

pub const LANG_ES: LangStrings = LangStrings {
	usage: "['Libro de cuentas']",
	options: OPTIONS_ES,
	ver: ": versión",
	ver_desc: ": Libro contable en línea de comandos, versión",
	term_size: MSG_TERM_112_ES,
	err_arguments: "Error: Demasiados argumentos: Solo un archivo 'LivreComptable', por favor.",
	err_chdir: "Error al cambiar el directorio a",
	err_is_dir: "es un directorio",
	header: HEADER_ES,
	new_db: "no existe.\n\n¿Creando un nuevo libro ?",
	ouverture: "Abriendo el libro «{1}» del directorio «{2}»",
};

pub const LANG_EN: LangStrings = LangStrings {
	usage: "['Accounting book']",
	options: OPTIONS_EN,
	ver: ": version",
	ver_desc: ": Accounting book in command line interface, version",
	term_size: MSG_TERM_112_EN,
	err_arguments: "Error: Too many arguments: Only one 'LivreComptable' file please.",
	err_chdir: "Error changing directory to",
	err_is_dir: "is a directory",
	header: HEADER_EN,
	new_db: "does not exist.\n\nCreating a new book ?",
	ouverture: "Opening book «{1}» from directory «{2}»",
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
