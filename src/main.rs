// main.rs

mod parse;
mod locale;
mod amj_date;

mod lc_libs;
use lc_libs::LivreComptable;

use std::env;
use std::path::Path;
use std::fs::File;
use std::process::ExitCode;
use std::io::{self, Read, Write, BufRead, Result};

use parse::VarsApp;

const DEFAUT_PRGNAME: &str = "lcd";
const DEFAUT_LIVRE: &str = "LivreComptable";
const VERSION: &str = "2025-08-24";

/// Retourne la largeur du terminal en colonnes.
/// Si la détection échoue, retourne 0 silencieusement.
fn terminal_width() -> usize {
	let tty = match File::open("/dev/tty") {
		Ok(file) => file,
		Err(_) => return 0,
	};

	let output = match std::process::Command::new("stty")
		.arg("size")
		.stdin(std::process::Stdio::from(tty))
		.output()
	{
		Ok(out) => out,
		Err(_) => return 0,
	};

	let stdout = String::from_utf8_lossy(&output.stdout);
	stdout
		.trim()
		.split_whitespace()
		.nth(1)
		.and_then(|col_str| col_str.parse::<usize>().ok())
		.unwrap_or(0)
}

fn term112cols(vars_prg: &VarsApp) -> bool {
	let mut width = terminal_width();

	if width == 0 {
		// Terminal inaccessible ou erreur silencieuse : on continue sans se plaindre
	}
	else if width < 112 {
		print!("{}", vars_prg.locale.term_size);
		io::stdout().flush().unwrap();
		let _ = std::io::BufReader::new(io::stdin()).read_line(&mut String::new());

		width = terminal_width();
		if width < 112 { return false; }
	}
	return true;
}

fn is_sqlite_file(path: &String) -> Result<bool> {
	let mut file = File::open(path)?;
	let mut buffer = [0u8; 16];
	file.read_exact(&mut buffer)?;

	let expected_signature = b"SQLite format 3\0";
	Ok(&buffer == expected_signature)
}
fn main() -> ExitCode {
	// Récupère le nom et le path de l'exécutable
	let args: Vec<String> = env::args().collect();
	let exec_full_path = Path::new(&args[0]);
	
	let exec_name = exec_full_path.file_name()
	.and_then(|n| n.to_str())
	.unwrap_or(DEFAUT_PRGNAME);

	let exec_path = exec_full_path.parent()
									.and_then(|p| p.to_str())
									.unwrap_or(".");

	// Maintenant on parse et manipule le nom du livre
	let mut var_app = match parse::VarsApp::parse_args(exec_name, VERSION) {
		parse::ParseResult::Ok(v) => v,
		parse::ParseResult::ShowHelp(msg) => {
			println!("{msg}");
			return ExitCode::SUCCESS;
		}
		parse::ParseResult::ShowVersion(msg) => {
			println!("{msg}");
			return ExitCode::SUCCESS;
		}
		parse::ParseResult::Error(err_msg, usage_msg, code) => {
			eprintln!("{err_msg}");
			println!("{usage_msg}");
			return ExitCode::from(code);
		}
	};

	// Teste si nous avons l'espace nécessaire sur le terminal.
	if ! term112cols(&var_app) { return ExitCode::from(10);}

	// Si aucun nom n’a été fourni, on met la valeur par défaut
	if var_app.livre_name.is_none() {
		var_app.livre_name = Some(format!("{}.{}", DEFAUT_LIVRE, amj_date::get_annee()));
		// CD vers exec_path
		if let Err(e) = env::set_current_dir(exec_path) {
			eprintln!("{} {} : {}", var_app.locale.err_chdir, exec_path, e);
			return ExitCode::from(5);
		}
	}
	else {	// Ici, on fait avec la valeur fournie
		let livre_path = Path::new(var_app.livre_name
									.as_ref()
									.unwrap());
		let livre_str = var_app.livre_name
									.as_ref()
									.unwrap();
		if livre_path.is_dir() {
			eprintln!("«{}» {}", livre_str, var_app.locale.err_is_dir);
			return ExitCode::from(10);
		}
		if livre_str.len() < 1 { return ExitCode::from(100); }	// Gnaisage ""

		if livre_str.contains('/') || livre_str.starts_with('.') {
			// Cas avec chemin relatif ou absolu
			if let Some(parent) = livre_path.parent() {
				if let Err(e) = env::set_current_dir(parent) {
					eprintln!("{} {} : {}", var_app.locale.err_chdir, parent.display(), e);
					return ExitCode::from(15);
				}
			}
			// On garde juste le nom du fichier
			if let Some(file_name) = livre_path.file_name() {
				var_app.livre_name = Some(file_name
											.to_string_lossy()
											.into_owned());
			} else { return ExitCode::from(10); }
		}
		else {
			// Pas de chemin → on cd vers exec_path
			if let Err(e) = env::set_current_dir(exec_path) {
				eprintln!("{} {} : {}", var_app.locale.err_chdir, exec_path, e);
				return ExitCode::from(25);
			}
		}
	}
	let app_work_path = match env::current_dir() {
		Ok(path) => path,
		Err(_) => { return ExitCode::from(1); }
	};

	// À partir d'ici, on a toutes les pièces pour démarrer
	println!("{}", var_app.locale.header.replace("{1}", exec_name)
										.replace("{2}", VERSION));

	// Est-ce que opts.livre_name existe déjà ?
	let livre_name = var_app.livre_name.as_ref().unwrap();
	if std::path::Path::new(livre_name).exists() {
		println!("{}", var_app.locale.ouverture.replace("{1}", livre_name)
											.replace("{2}", app_work_path.to_str().unwrap()));
		// Ici on passe le nom à sqlite3
		match is_sqlite_file(&var_app.livre_name.clone().unwrap()) {
			Ok(true) => {
				match LivreComptable::open_db(&var_app, false) {
					Ok(_) => {
						println!("Base ouverte.");
					},
					Err(msg) => eprintln!("Échec : {}", msg),
				};
			}
			Ok(false) => println!("Le fichier n'est pas une base SQLite."),
			Err(e) => eprintln!("Erreur lors de la lecture du fichier : {}", e),
		}
	}
	else {
		print!("«{}/{}» {} ",app_work_path.display(), livre_name, var_app.locale.new_db);
		io::stdout().flush().unwrap(); // Force l'affichage immédiat

		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();

		let lettre = input.trim().chars().next();

		match lettre {
			Some('n') | Some('N') => println!("OK Bye."),
			_ => {
					println!("Création de db");
					match LivreComptable::open_db(&var_app, true) {
						Ok(_) => println!("Base créée."),
						Err(msg) => eprintln!("Échec : {}", msg),
					}
				},
		}
	}
	ExitCode::SUCCESS
}
