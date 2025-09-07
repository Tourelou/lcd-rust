// ajoute.rs

use super::LivreComptable;
use super::ajoute_locale;
use super::lc_utils::string_2_cent;
use super::lc_utils::enable_raw_mode;
use super::lc_utils::disable_raw_mode;
use super::wrapper_readline::Readline;
use super::{Categorie, Compte};

use crate::parse::VarsApp;

use std::io::{self, Read, Write};

fn get_menu(chaine: &str, index: u8) -> Option<String>{
	// index sert à différencier si c'est pour Compte ou Catégorie
	let elements: Vec<&str> = chaine.split('|').collect();

	println!("        {:<19} : 1- {}", elements[0], elements[1]);
	for (i, item) in elements.iter().enumerate().skip(2) {
		if i == elements.len() -1 { break; }
		println!("{:<24}    : {}- {}", "", i, item);
	}
	print!("        {:<16}    : ", elements[elements.len()-1]);
	io::stdout().flush().unwrap();
	enable_raw_mode();
	let mut buffer = [0; 1]; // Lire un seul octet
	if index == 0 {			// index 0 pour les paramètres d'un compte
		loop {
			io::stdin().read_exact(&mut buffer).unwrap();
			match buffer[0] {
				b'0' => {
					disable_raw_mode();
					println!("Ok, exit");
					return None;
				}
				b'1' => {
					disable_raw_mode();
					println!("{}", elements[1]);
					return Some(String::from("Courant"));
				}
				b'2' => {
					disable_raw_mode();
					println!("{}", elements[2]);
					return Some(String::from("Épargne"));
				}
				b'3' => {
					disable_raw_mode();
					println!("{}", elements[3]);
					return Some(String::from("Crédit"));
				}
				_ => {}
			}
		}
	}
	if index == 1 {			// index 0 pour les paramètres d'une catégorie
		loop {
			io::stdin().read_exact(&mut buffer).unwrap();
			match buffer[0] {
				b'0' => {
					disable_raw_mode();
					println!("Ok, exit");
					return None;
				}
				b'1' => {
					disable_raw_mode();
					println!("{}", elements[1]);
					return Some(String::from("IN"));
				}
				b'2' => {
					disable_raw_mode();
					println!("{}", elements[2]);
					return Some(String::from("OUT"));
				}
				_ => {}
			}
		}
	}
	None
}

#[allow(unused)]
impl LivreComptable {
	pub fn ajoute_compte(&mut self, var_app: &VarsApp) -> Option<Compte> {
		let language = ajoute_locale::set_ajoute_lang(&var_app);

		let mut rl = Readline::new();
		let nom = match self.run_context("sans-histoire",
										format!("{}", language.nom_compte).as_str(),
										 false) {
			Some(line) => line,
			None => return None,
		};
		if nom == "" { return None; }

		let cmpt_ref = match self.run_context("sans-histoire",
										format!("{}", language.ref_compte).as_str(),
										false) {
			Some(line) => line,
			None => return None,
		};
		if cmpt_ref == "" { return None; }

		let cmpt_type = match get_menu(language.type_compte, 0) {
			Some(line) => line,
			None => return None,
		};

		let depart_string = match self.run_context("sans-histoire",
										format!("{}", language.depart_compte).as_str(),
										false) {
			Some(line) => line,
			None => return None,
		};
		let depart = match string_2_cent(&depart_string) {
			Some(val) => val,
			None => return None,
		};

		let reponse = Compte {
			nom,
			cmpt_ref,
			cmpt_type,
			depart: depart,
			present: depart,
		};
		println!("{:#?}", reponse);

		Some(reponse)
	}

	pub fn ajoute_categorie(&mut self, var_app: &VarsApp) -> Option<Categorie> {
		let language = ajoute_locale::set_ajoute_lang(&var_app);

		let mut rl = Readline::new();
		let nom = match self.run_context("sans-histoire",
										format!("{}", language.nom_categorie).as_str(),
										false) {
			Some(line) => line,
			None => return None,
		};
		if nom == "" { return None; }

		let cat_type = match get_menu(language.type_categorie, 1) {
			Some(line) => line,
			None => return None,
		};

		let reponse = Categorie {
			nom,
			utilise: 0,
			cat_type,
		};
		println!("{:#?}", reponse);
		Some(reponse)

	}
}
