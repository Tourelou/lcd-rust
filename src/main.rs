// main.rs

mod parse;
mod locale;

use std::env;
use std::path::Path;

use std::time::{SystemTime, UNIX_EPOCH};
use time::OffsetDateTime;

const DEFAUT_PRGNAME: &str = "lcd";
const DEFAUT_LIVRE: &str = "LivreComptable";
const VERSION: &str = "2025-08-13";

fn get_annee() -> String {
	let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
	let datetime = OffsetDateTime::from_unix_timestamp(now as i64).unwrap();
	datetime.year().to_string()
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

	println!("prg: {exec_name}, path: {exec_path}");

	// Maintenant on parse et manipule le nom du livre
	let mut opts = parse::Options::parse_args(exec_name, VERSION);

		// Si aucun nom n’a été fourni, on met la valeur par défaut
		if opts.livre_name.is_none() {
			opts.livre_name = Some(format!("{}.{}", DEFAUT_LIVRE, get_annee()));
// CD vers exec_path
		}
		else {
			println!("Grosse job de cd.")
		}
	println!("Livre: {}", opts.livre_name.unwrap());
}
