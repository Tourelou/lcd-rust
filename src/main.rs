// main.rs

mod parse;
mod locale;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, Write, BufRead};

use std::time::{SystemTime, UNIX_EPOCH};
use time::OffsetDateTime;

use crate::parse::Options;

const DEFAUT_PRGNAME: &str = "lcd";
const DEFAUT_LIVRE: &str = "LivreComptable";
const VERSION: &str = "2025-08-14";

fn get_annee() -> String {
	let now = SystemTime::now()
						.duration_since(UNIX_EPOCH)
						.unwrap()
						.as_secs();
	let datetime = OffsetDateTime::from_unix_timestamp(now as i64)
									.unwrap();
	datetime.year().to_string()
}

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

fn term112cols(vars_prg: &Options) {
	let mut width = terminal_width();

	if width == 0 {
		// Terminal inaccessible ou erreur silencieuse : on continue sans se plaindre
	}
	else if width < 112 {
		print!("{}", vars_prg.locale.term_size);
		io::stdout().flush().unwrap();
		let _ = std::io::BufReader::new(io::stdin()).read_line(&mut String::new());

		width = terminal_width();
		if width < 112 { std::process::exit(100); }
	}
}

fn main() {
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
	let mut opts = parse::Options::parse_args(exec_name, VERSION);

	// Teste si nous avons l'espace nécessaire sur le terminal.
	term112cols(&opts);

	// Si aucun nom n’a été fourni, on met la valeur par défaut
	if opts.livre_name.is_none() {
		opts.livre_name = Some(format!("{}.{}", DEFAUT_LIVRE, get_annee()));
		// CD vers exec_path
		if let Err(e) = env::set_current_dir(exec_path) {
			eprintln!("{} {} : {}", opts.locale.err_chdir, exec_path, e);
			std::process::exit(5);
		}
	}
	else {	// Ici, on fait avec la valeur fournie
		let livre_path = Path::new(opts.livre_name
									.as_ref()
									.unwrap());
		let livre_str = opts.livre_name
									.as_ref()
									.unwrap();
		if livre_path.is_dir() {
			eprintln!("«{}» {}", livre_str, opts.locale.err_is_dir);
			std::process::exit(10);
		}
		if livre_str.len() < 1 { std::process::exit(100); }	// Gnaisage ""

		if livre_str.contains('/') || livre_str.starts_with('.') {
			// Cas avec chemin
			if let Some(parent) = livre_path.parent() {
				if let Err(e) = env::set_current_dir(parent) {
					eprintln!("{} {} : {}", opts.locale.err_chdir, parent.display(), e);
					std::process::exit(15);
				}
			}
			// On garde juste le nom du fichier
			if let Some(file_name) = livre_path.file_name() {
				opts.livre_name = Some(file_name
											.to_string_lossy()
											.into_owned());
			} else { std::process::exit(10); }
		}
		else {
			// Pas de chemin → on cd vers exec_path
			if let Err(e) = env::set_current_dir(exec_path) {
				eprintln!("{} {} : {}", opts.locale.err_chdir, exec_path, e);
				std::process::exit(25);
			}
		}
	}
	let app_work_path = env::current_dir()
										.unwrap_or_else(|_| std::process::exit(1));
	// À partir d'ici, on a toutes les pièces pour démarrer
	println!("{}", opts.locale.header.replace("{1}", exec_name)
										.replace("{2}", VERSION));

	// Est-ce que opts.livre_name existe déjà ?
	let livre_name = opts.livre_name.as_ref().unwrap();
	if std::path::Path::new(livre_name).exists() {
		println!("{}", opts.locale.ouverture.replace("{1}", livre_name)
											.replace("{2}", app_work_path.to_str().unwrap()));
		// Ici on passe le nom à sqlite3
	}
	else {
		print!("«{}/{}» {} ",app_work_path.display(), livre_name, opts.locale.new_db);
		io::stdout().flush().unwrap(); // Force l'affichage immédiat

		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();

		let lettre = input.trim().chars().next();

		match lettre {
			Some('n') | Some('N') => println!("On arrête."),
			_ => println!("Création de db"),
		}
	}
}
