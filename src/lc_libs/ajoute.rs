// ajoute.rs

use super::LivreComptable;
use super::ajoute_locale;
use super::lc_utils::string_2_cent;
use super::{Categorie, Compte};

use std::io::{self, Write};


fn get_index(max_len: usize) -> usize {
	// Demande à l'usager quelle ligne traiter.
	let mut input = String::new();

	// 1. Lire la saisie
	io::stdin()
		.read_line(&mut input)
		.expect(format!("{}", "err_keyboard").as_str());

	// 2. Tenter de convertir en nombre (usize est idéal pour les index)
	match input.trim().parse::<usize>() {
		Ok(index) => {
			if index > max_len { 
				println!("{}", "err_index_too_big");
				return 0;
			}
			else { return index; }
		}
		Err(_) => { return 0; }
	}
}



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
	let mut reponse: usize; // Lire un seul octet
	if index == 0 {			// index 0 pour les paramètres d'un compte
		loop {
			reponse = get_index(3);
			match reponse {
				0 => {
					println!("Ok, exit");
					return None;
				}
				1 => {
					println!("{:30}{}","", elements[1]);
					return Some(String::from("Courant"));
				}
				2 => {
					println!("{:30}{}","", elements[2]);
					return Some(String::from("Épargne"));
				}
				3 => {
					println!("{:30}{}","", elements[3]);
					return Some(String::from("Crédit"));
				}
				_ => { println!("Ici peut-être"); }
			}
		}
	}
	if index == 1 {			// index 0 pour les paramètres d'une catégorie
		loop {
			reponse = get_index(3);
			match reponse {
				0 => {
					println!("Ok, exit");
					return None;
				}
				1 => {
					println!("{:30}{}","", elements[1]);
					return Some(String::from("IN"));
				}
				2 => {
					println!("{:30}{}","", elements[2]);
					return Some(String::from("OUT"));
				}
				_ => { println!("Ici peut-être"); }
			}
		}
	}
	None
}

impl LivreComptable {
	pub fn ajoute_compte(&mut self) -> Option<Compte> {
		let language = match self.abrev_langue.as_str() {
			"fr" => ajoute_locale::LANG_FR,
			"es" => ajoute_locale::LANG_ES,
			_ => ajoute_locale::LANG_EN,
		};

//		let mut rl = Readline::new();
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

		Some(reponse)

	}

	pub fn ajoute_categorie(&mut self) -> Option<Categorie> {
		let language = match self.abrev_langue.as_str() {
			"fr" => ajoute_locale::LANG_FR,
			"es" => ajoute_locale::LANG_ES,
			_ => ajoute_locale::LANG_EN,
		};

//		let mut rl = Readline::new();
		let nom = match self.run_context("sans-histoire",
										format!("{}", language.nom_categorie).as_str(),
										false) {
			Some(line) => line,
			None => {
				println!("Cat1 sans nom");
				return None},
		};
		if nom == "" {
			println!("Cat2 sans nom");
			return None; }

		let cat_type = match get_menu(language.type_categorie, 1) {
			Some(line) => line,
			None => return None,
		};

		let reponse = Categorie {
			nom,
			utilise: 0,
			cat_type,
		};

		Some(reponse)

	}
}
