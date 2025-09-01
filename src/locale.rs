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

const  TYPE_COMPTE_FR: &str =
"Type de compte|Compte courant|Compte épargne|Carte de crédit|Votre choix";
const  TYPE_CATEGORIE_FR: &str =
"Type de catégorie|En entrée ex.: Paye|En Sortie ex.: Nourriture|Votre choix";

const TYPE_COMPTE_ES: &str =
"Tipo de cuenta|Cuenta corriente|Cuenta de ahorros|Tarjeta de crédito|Su elección";
const TYPE_CATEGORIE_ES: &str =
"Tipo de categoría|Ingreso ej.: Sueldo|Gasto ej.: Comida|Su elección";

const TYPE_COMPTE_EN: &str =
"Account type|Checking account|Savings account|Credit card|Your choice";
const TYPE_CATEGORIE_EN: &str =
"Category type|Income e.g.: Salary|Expense e.g.: Food|Your choice";


#[derive(Debug)]
pub struct LangStrings {
	pub usage: &'static str,
	pub options: &'static str,
	pub ver: &'static str,
	pub ver_desc: &'static str,
	pub term_size: &'static str,
	pub err_date: &'static str,
	pub err_arguments: &'static str,
	pub err_chdir: &'static str,
	pub err_is_dir: &'static str,
	pub err_echec: &'static str,
	pub err_not_sqlite3: &'static str,
	pub err_read_file: &'static str,
	pub err_open_base: &'static str,
	pub err_create_table: &'static str,
	pub exit_no_compte: &'static str,
	pub exit_no_categorie: &'static str,
	pub err_ecriure_db: &'static str,
	pub header: &'static str,
	pub head_nouv_compte: &'static str,
	pub nom_compte: &'static str,
	pub ref_compte: &'static str,
	pub type_compte: &'static str,
	pub depart_compte: &'static str,
	pub head_nouv_categorie: &'static str,
	pub nom_categorie: &'static str,
	pub type_categorie: &'static str,
	pub new_db: &'static str,
	pub ouverture: &'static str,
}

pub const LANG_FR: LangStrings = LangStrings {
	usage: "['Livre de compte']",
	options: OPTIONS_FR,
	ver: ": version",
	ver_desc: ": Livre comptable en ligne de commande, version",
	term_size: MSG_TERM_112_FR,
	err_date: "Erreur Sysytème: Pas de date\nEntrer une date au format AAAA-MM-JJ.\n",
	err_arguments: "Erreur: Trop d'argument: un seul fichier LivreComptable s.v.p.",
	err_chdir: "Erreur changement de répertoire vers",
	err_is_dir: "est un répertoire",
	err_echec: "Échec:",
	err_not_sqlite3: "Le fichier n'est pas une base SQLite.",
	err_read_file: "Erreur lors de la lecture du fichier:",
	err_open_base: "Erreur lors de l'ouverture de la base",
	err_create_table: "Erreur lors de la création de tables essentielles.",
	exit_no_compte: "Ne peux pas marcher si aucun compte n'est créé",
	exit_no_categorie: "Ne peux pas marcher si aucune catégorie n'est créé",
	err_ecriure_db: "Erreur en écriture vers la base de données",
	header: HEADER_FR,
	head_nouv_compte: "Création de nouveau(x) compte(s) aujourd'hui le",
	nom_compte:    "        Nom du compte       : ",
	ref_compte:    "        # de référence      : ",
	type_compte: TYPE_COMPTE_FR,
	depart_compte: "        Montant de départ   : ",
	head_nouv_categorie: "Création de nouvelle(s) catégorie(s)",
	nom_categorie: "        Nom de catégorie    : ",
	type_categorie: TYPE_CATEGORIE_FR,
	new_db: "n'existe pas.\n\nCréation d'un nouveau livre ?",
	ouverture: "Ouverture du livre «{1}» à partir du répertoire «{2}»",
};

pub const LANG_ES: LangStrings = LangStrings {
	usage: "['Libro de cuentas']",
	// usage: "['Libro contable']",
	options: OPTIONS_ES,
	ver: ": versión",
	ver_desc: ": Libro contable por línea de comandos, versión",
	term_size: MSG_TERM_112_ES,
	err_date: "Error del sistema: No hay fecha\nIngrese una fecha en formato AAAA-MM-DD.\n",
	err_arguments: "Error: Demasiados argumentos: Solo un archivo 'LivreComptable', por favor.",
	err_chdir: "Error al cambiar al directorio",
	err_is_dir: "es un directorio",
	err_echec: "Fallo:",
	err_not_sqlite3: "El archivo no es una base de datos SQLite.",
	err_read_file: "Error al leer el archivo:",
	err_open_base: "Error al abrir la base de datos",
	err_create_table: "Error al crear las tablas esenciales.",
	exit_no_compte: "No se puede continuar si no se ha creado ninguna cuenta",
	exit_no_categorie: "No se puede continuar si no se ha creado ninguna categoría",
	err_ecriure_db: "Error al escribir en la base de datos",
	header: HEADER_ES,
	head_nouv_compte: "Creación de nueva(s) cuenta(s) hoy en",
	nom_compte:    "        Nombre de cuenta    : ",
	ref_compte:    "        Nº de referencia    : ",
	type_compte: TYPE_COMPTE_ES,
	depart_compte: "        Monto inicial       : ",
	head_nouv_categorie: "Creación de nueva(s) categoría(s)",
	nom_categorie: "        Nombre de categoría : ",
	type_categorie: TYPE_CATEGORIE_ES,
	new_db: "no existe.\n\n¿Creando un nuevo libro?",
	ouverture: "Abriendo el libro «{1}» desde el directorio «{2}»",
};

pub const LANG_EN: LangStrings = LangStrings {
	usage: "['Accounting book']",
	options: OPTIONS_EN,
	ver: ": version",
	ver_desc: ": Command-line accounting book, version",
	term_size: MSG_TERM_112_EN,
	err_date: "System Error: No date provided\nPlease enter a date in the format YYYY-MM-DD.\n",
	err_arguments: "Error: Too many arguments: Only one 'LivreComptable' file please.",
	err_chdir: "Error changing directory to",
	err_is_dir: "is a directory",
	err_echec: "Failure:",
	err_not_sqlite3: "The file is not a SQLite database.",
	err_read_file: "Error reading the file:",
	err_open_base: "Error opening the database",
	err_create_table: "Error creating essential tables.",
	exit_no_compte: "Cannot proceed if no account is created",
	exit_no_categorie: "Cannot proceed if no category is created",
	err_ecriure_db: "Error writing to database",
	header: HEADER_EN,
	head_nouv_compte: "Creating new account(s) today on",
	nom_compte:    "        Account name        : ",
	ref_compte:    "        Reference #         : ",
	type_compte: TYPE_COMPTE_EN,
	depart_compte: "        Starting amount     : ",
	head_nouv_categorie: "Creating new category(ies)",
	nom_categorie: "        Category name       : ",
	type_categorie: TYPE_CATEGORIE_EN,
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
