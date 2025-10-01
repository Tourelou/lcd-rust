// nouvelle_transaction.rs

use std::io::{self, Write};

use super::LivreComptable;
use super::nouv_trans_locale;
use crate::lc_libs::Transaction;
use crate::lc_libs::lc_utils::string_2_cent;
use crate::lc_libs::lc_utils::get_choix;
use crate::parse::VarsApp;

impl LivreComptable {
	pub fn new_trans(&mut self, app_var: &mut VarsApp, favorite: bool) -> bool {
		let language = match self.abrev_langue.as_str() {
			"fr" => nouv_trans_locale::LANG_FR,
			"es" => nouv_trans_locale::LANG_ES,
			_ => nouv_trans_locale::LANG_EN,
		};

		let mut copie = false;
		let mut t = Transaction { date: String::new(),
										description: String::new(),
										t_type: String::new(),
										compte: String::new(),
										categorie: String::new(),
										montant: 0,
		};

// ####################  Date  ####################

		if favorite { t.date = String::from("----------")}
		else {
			print!("{}", language.question_date);
			print!("{}", app_var.date.derniere_entree);
			io::stdout().flush().unwrap();

			match self.run_context("sans-histoire", " ? ", false).unwrap_or_default().as_str() {
				"" => println!("{}", language.date_acceptee),
				"0" => {
					println!("{}", language.trans_annulee);
					return true;
				},
				reponse if !app_var.date.set_check_date(reponse) => {
					println!("{}", language.date_pas_de_sens);
					return true;
				},
				_ => (),
			}
			t.date = app_var.date.derniere_entree.clone();
			println!("----------");
		}
// #################  Description  #################

		let prompt = if favorite { language.desc_favorite }
		else { language.desc_transaction };
		print!("{}", prompt);
		io::stdout().flush().unwrap();

		let reponse = self.run_context("sans-histoire", "", false).unwrap_or_default();
		if reponse.is_empty() || reponse == "0" {
			println!("{}", language.trans_annulee);
			return true;
		}
		if !favorite && reponse == "@" {
			println!("----------");
			if self.FAVORITES.is_empty() {
				println!("{}", language.no_favorite);
				println!("{}", language.trans_annulee);
				return true;
			}
			println!("{}", language.choisir_favorite);

			self.printTransactions(&self.FAVORITES, true);
			print!("{}", language.zero_ou_enter);
			io::stdout().flush().unwrap();

			let choix = match get_choix() {
				Ok(n) if n > 0 && (n as usize) <= self.FAVORITES.len() => n - 1,
				_ => {
					println!("{}", language.trans_annulee);
					return true;
				}
			};
			println!("----------");
			let fav = &self.FAVORITES[choix as usize];
			t.description = fav.description.clone();
			t.t_type = fav.t_type.clone();
			t.compte = fav.compte.clone();
			t.categorie = fav.categorie.clone();
			t.montant = fav.montant;
			copie = true;
		}
		else {
			t.description = reponse;
			println!("----------");

// ####################  Type  ####################

			t.t_type = match self.print_trans_type() {
				Ok(r) => r,
				Err(()) => {
					println!("{}", language.trans_annulee);
					return true;
				},
			};
			println!("----------");

	// ####################  Compte  ####################

			println!("{}", language.choisir_compte);
			self.imp_comptes(&self.TMP_COMPTES);
			print!("{}", language.zero_ou_enter);
			io::stdout().flush().unwrap();

			let choix = match get_choix() {
				Ok(n) if n > 0 && (n as usize) <= self.TMP_COMPTES.len() => n - 1,
				_ => {
					println!("{}", language.trans_annulee);
					return true;
				}
			};
			t.compte = self.TMP_COMPTES[choix as usize].nom.clone();
			println!("----------");

	// ###################  Catégorie  ###################

			println!("{}", language.choisir_categorie);
			self.imp_categories(&self.TMP_CATEGORIES);
			print!("{}", language.zero_ou_enter);
			io::stdout().flush().unwrap();

			let choix = match get_choix() {
				Ok(n) if n > 0 && (n as usize) <= self.TMP_CATEGORIES.len() => n - 1,
				_ => {
					println!("{}", language.trans_annulee);
					return true;
				}
			};
			t.categorie = self.TMP_CATEGORIES[choix as usize].nom.clone();
			println!("----------");
		}
// ###################  Montant  ###################

		if !copie || (copie && t.montant == 0) {
			loop {
				let reponse = self.run_context("sans-histoire",
								language.montant_de, false).unwrap_or_default();

				if favorite && reponse == "0" { break; } // Un simple break t.montant déjà à 0.

				if reponse == "" || reponse == "0" {
					println!("{}", language.trans_annulee);
					return true;
				}

				t.montant = match string_2_cent(&reponse) {
					Some(v) => v,
					None => {
						println!("{}", language.mauvais_montant);
						continue; // ou return false; selon la logique que tu veux
					},
				};
				println!("----------");
				break;
			};
		}
		// Pour éviter une erreur SQL quand la description contient une apostrophe.
		let description_db = t.description.replace("'", "''");
		if favorite {
			let requete_sql = 
			format!("INSERT INTO Favorites(Date, Description, Type, Compte, Catégorie, Montant) VALUES('{}', '{}', '{}', '{}', '{}', {})",
											t.date, description_db, t.t_type, t.compte, t.categorie, t.montant);
			match self.bd.exec(requete_sql.as_str()){
				Ok(_) => self.FAVORITES.push(t.clone()),
				Err(e) => {
					eprint!("{e} : ");
					return false;	// Si pas capable d'écrire dans la bd = erreur grave
				}
			};
		}
		else {
			let requete_sql = 
			format!("INSERT INTO Transactions(Date, Description, Type, Compte, Catégorie, Montant) VALUES('{}', '{}', '{}', '{}', '{}', {})",
											t.date, description_db, t.t_type, t.compte, t.categorie, t.montant);
			match self.bd.exec(requete_sql.as_str()){
				Ok(_) => self.TRANSACTIONS.push(t.clone()),
				Err(e) => {
					eprint!("{e} : ");
					return false;	// Si pas capable d'écrire dans la bd = erreur grave
				}
			};
	//	###########  Calcul des montants en bd et en mémoire  ###########

			let mut i_compte = self.COMPTES.iter().position(|c| c.nom == t.compte).unwrap();
			let i_cat = self.CATEGORIES.iter().position(|c| c.nom == t.compte).unwrap();

			if t.t_type == "Dépôt" || t.t_type == "Crédit" {
				if self.COMPTES[i_compte].cmpt_type == "Crédit" {
					if !self.calcul_soustrait_compte(i_compte, t.montant) { return false; }
				}
				else {
					if !self.calcul_ajoute_compte(i_compte, t.montant) { return false;}
				}
			}
			else if t.t_type == "Débit" || t.t_type == "Achat" {
				if self.COMPTES[i_compte].cmpt_type == "Crédit" {
					if !self.calcul_ajoute_compte(i_compte, t.montant) { return false; }
				}
				else {
					if !self.calcul_soustrait_compte(i_compte, t.montant) { return false; }
				}
			}
			else if t.t_type == "Virement" {
				if !self.calcul_soustrait_compte(i_compte, t.montant) { return false; }
				i_compte = self.COMPTES.iter().position(|c| c.nom == t.categorie).unwrap();
				if !self.calcul_ajoute_compte(i_compte, t.montant) { return false; }
			}
			else if t.t_type == "Paiement" {
				if !self.calcul_soustrait_compte(i_compte, t.montant) { return false; }
				i_compte = self.COMPTES.iter().position(|c| c.nom == t.categorie).unwrap();
				if !self.calcul_soustrait_compte(i_compte, t.montant) { return false; }
			}
			let sql_plus_un = format!("UPDATE Catégories SET Utilisé = Utilisé + 1 WHERE Nom = '{}'",
													self.CATEGORIES[i_cat].nom);
			match self.bd.exec(&sql_plus_un) {
				Ok(_) => {
					self.CATEGORIES[i_cat].utilise += 1 ;
				}
				Err(e) => {
					eprint!("{e} : ");
					return false;
				}
			}
			self.print1Transaction(&t);
			self.imp_comptes(&self.COMPTES);
		}
		return true;
	}

	fn calcul_ajoute_compte(&mut self, i_compte: usize, montant: i64) -> bool {
		let requete_sql_plus = format!("UPDATE Master SET Présent = Présent + {} WHERE Nom = '{}'"
													, montant, self.COMPTES[i_compte].nom);
		match self.bd.exec(&requete_sql_plus) {
			Ok(_) => {
				self.COMPTES[i_compte].present += montant;
				return true;
			},
			Err(e) => {
				eprint!("{e} : ");
				return false;
			}
		}
	}

	fn calcul_soustrait_compte(&mut self, i_compte: usize, montant: i64) -> bool {
		let requete_sql_moins = format!("UPDATE Master SET Présent = Présent - {} WHERE Nom = '{}'",
													montant, self.COMPTES[i_compte].nom);
		match self.bd.exec(&requete_sql_moins) {
			Ok(_) => {
				self.COMPTES[i_compte].present -= montant;
				return true;
			},
			Err(e) => {
				eprint!("{e} : ");
				return false;
			}
		}
	}
}
