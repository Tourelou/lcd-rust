// supprime.rs

use crate::lc_libs::lc_utils::get_choix;
use crate::lc_libs::{Transaction, Categorie, Compte};
use crate::lc_libs::supprime_locale::{self, SupprimeStrings};

use super::LivreComptable;
use std::io::{self, Write};

impl LivreComptable {
	pub fn supp_compte(&mut self) -> bool {

		let language = match self.abrev_langue.as_str() {
			"fr" => supprime_locale::LANG_FR,
			"es" => supprime_locale::LANG_ES,
			_ => supprime_locale::LANG_EN,
		};

		self.TMP_COMPTES.clear();	// Toujours faire un clear avant de commencer.

		for compte in &self.COMPTES {
			if compte.depart != compte.present { continue; }

			for cat in &self.CATEGORIES {
				if compte.nom == cat.nom && cat.utilise == 1000 {
					self.TMP_COMPTES.push(compte.clone());
				}
			}
		}
		if !self.TMP_COMPTES.is_empty() {

			self.imp_comptes(&self.TMP_COMPTES);
			if self.TMP_COMPTES.len() == 1 {
				print!("{}", language.question_supp_compte);
				io::stdout().flush().unwrap();

				let mut line = String::new();
				std::io::stdin().read_line(&mut line).unwrap();

				println!("----------");
				if let Some(c) = line.chars().next() {
					if c == 'n' || c == 'N' {
						println!("{}", language.pas_de_prob);
						return true;
					}
					else { return self.do_supp_compte(&self.TMP_COMPTES[0].clone(), &language); }
				} else {return true; }
			}
			else {		// TMP_COMPTES.len() > 1
				print!("{}", language.question_cmpt_numero);
				io::stdout().flush().unwrap();

				match get_choix() {
					Ok(n) => {
						println!("----------");
						if n == 0 {
							println!("{}", language.pas_de_prob);
							return true;
						}
						else if n as usize > self.TMP_COMPTES.len() {
							println!("{}", language.err_num_trop_grand);
							return true;
						}
						return self.do_supp_compte(&self.TMP_COMPTES[n as usize - 1].clone(), &language);
					}
					Err(()) => {
						println!("----------");
						println!("{}", language.err_criteres);
						return true;
					}
				}
			}
		}
		println!("{}", language.no_cmpt_2_del);
		return true;
	}

	fn do_supp_compte(&mut self, compte: &Compte, l: &SupprimeStrings) -> bool{
		match self.bd.exec(format!("DELETE FROM Master WHERE Nom = \"{}\" AND Départ = {}",
											compte.nom, compte.depart).as_str()) {
			Ok(_) => {
				println!("{}", format!("{}", l.succes_supp_cmpt).replace("{1}", &compte.nom));
				self.COMPTES.retain(|c| c != compte);

				let categorie = Categorie {	// Suppression de la
					nom: compte.nom.clone(),		// catégorie associée.
					utilise: 1000,
					cat_type: compte.cmpt_type.clone(),
				};
				return self.do_supp_categorie(&categorie, &l);
			},
			Err(e) => {
				eprint!("{e}");
				return false;
			 },
		}
	}
	// ########################################################################

	pub fn supp_categorie(&mut self) -> bool {

		let language = match self.abrev_langue.as_str() {
			"fr" => supprime_locale::LANG_FR,
			"es" => supprime_locale::LANG_ES,
			_ => supprime_locale::LANG_EN,
		};

		self.TMP_CATEGORIES.clear();	// Toujours faire un clear avant de commencer.

		for categorie in &self.CATEGORIES {
			if categorie.utilise != 0 { continue; }
			self.TMP_CATEGORIES.push(categorie.clone());
		}
		if self.TMP_CATEGORIES.is_empty() { println!("{}", language.no_cat_2_del); }
		else {
			self.imp_categories(&self.TMP_CATEGORIES);
			if self.TMP_CATEGORIES.len() == 1 {
				print!("{}", language.question_supp_cat);
				io::stdout().flush().unwrap();

				let mut line = String::new();
				std::io::stdin().read_line(&mut line).unwrap();

				println!("----------");
				if let Some(c) = line.chars().next() {
					if c == 'n' || c == 'N' {
						println!("{}", language.pas_de_prob);
						return true;
					}
					else { return self.do_supp_categorie(&self.TMP_CATEGORIES[0].clone(), &language); }
				} else { return true; }
			}
			else {		// TMP_CATEGORIES.len() > 1
				print!("{}", language.question_cat_numero);
				io::stdout().flush().unwrap();

				match get_choix() {
					Ok(n) => {
						println!("----------");
						if n == 0 {
							println!("{}", language.pas_de_prob);
							return true;
						}
						else if n as usize > self.TMP_CATEGORIES.len() {
							println!("{}", language.err_num_trop_grand);
							return true;
						}
						return self.do_supp_categorie(&self.TMP_CATEGORIES[n as usize - 1].clone(), &language);
					}
					Err(()) => {
						println!("----------");
						println!("{}", language.err_criteres);
						return true;
					}
				}
			}
		}
		return true;
	}

	fn do_supp_categorie(&mut self, categorie: &Categorie, l: &SupprimeStrings) -> bool {
		match self.bd.exec(format!("DELETE FROM Catégories WHERE Nom = \"{}\" AND Type = \"{}\"",
											categorie.nom, categorie.cat_type).as_str()) {
			Ok(_) => {
				println!("{}", format!("{}", l.succes_supp_cat).replace("{1}", &categorie.nom));
				self.CATEGORIES.retain(|c| c != categorie);
			},
			Err(e) => {
				eprint!("{e}");
				return false;
			 },
		}
		return true;
	}

	pub fn supp_favorite(&mut self) -> bool {

		let language = match self.abrev_langue.as_str() {
			"fr" => supprime_locale::LANG_FR,
			"es" => supprime_locale::LANG_ES,
			_ => supprime_locale::LANG_EN,
		};

		if self.FAVORITES.is_empty() { println!("{}", language.no_fav_2_del); }
		else {
			self.printTransactions(&self.FAVORITES, true);
			if self.FAVORITES.len() == 1 {
				print!("{}", language.question_supp_fav);
				io::stdout().flush().unwrap();

				let mut line = String::new();
				std::io::stdin().read_line(&mut line).unwrap();

				println!("----------");
				if let Some(c) = line.chars().next() {
					if c == 'n' || c == 'N' {
						println!("{}", language.pas_de_prob);
						return true;
					}
					else { return self.do_supp_favorite(&self.FAVORITES[0].clone(), &language); }
				} else { return true; }
			}
			else {		// FAVORITES.len() > 1
				print!("{}", language.question_fav_numero);
				io::stdout().flush().unwrap();

				match get_choix() {
					Ok(n) => {
						println!("----------");
						if n == 0 {
							println!("{}", language.pas_de_prob);
							return true;
						}
						else if n as usize > self.FAVORITES.len() {
							println!("{}", language.err_num_trop_grand);
							return true;
						}
						return self.do_supp_favorite(&self.FAVORITES[n as usize - 1].clone(), &language);
					}
					Err(()) => {
						println!("----------");
						println!("{}", language.err_criteres);
						return true;
					}
				}
			}
		}
		return true;
	}

	fn do_supp_favorite(&mut self, favorite: &Transaction, l: &SupprimeStrings) -> bool {
		match self.bd.exec(format!("DELETE FROM Favorites WHERE Description = \"{}\" AND Montant = {}",
											favorite.description, favorite.montant).as_str()) {
			Ok(_) => {
				println!("{}", format!("{}", l.succes_supp_fav).replace("{1}", &favorite.description));
				self.FAVORITES.retain(|c| c != favorite);
			},
			Err(e) => {
				eprint!("{e}");
				return false;
			 },
		}
		return true;
	}
}
