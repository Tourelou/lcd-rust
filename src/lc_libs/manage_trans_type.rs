// manage_trans_type.rs

use super::LivreComptable;
use super::nouv_trans_locale;
use crate::lc_libs::lc_utils::get_choix;
use crate::lc_libs::lc_utils::string_2_cent;

use std::io::{self, Write};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TypeTransaction {
	Depot,
	Debit,
	Credit,
	Achat,
	Virement,
	Paiement,
}

fn get_comptes_valides() -> HashMap<TypeTransaction, Vec<&'static str>> {
	use TypeTransaction::*;
	HashMap::from([
		(Depot, vec!["Courant"]),
		(Debit, vec!["Courant", "Crédit"]),
		(Credit, vec!["Courant", "Épargne", "Crédit"]),
		(Achat, vec!["Courant", "Crédit"]),
		(Virement, vec!["Courant", "Épargne"]),
		(Paiement, vec!["Courant"]),
	])
}

fn get_categories_valides() -> HashMap<TypeTransaction, Vec<&'static str>> {
	use TypeTransaction::*;
	HashMap::from([
		(Depot, vec!["IN"]),
		(Debit, vec!["OUT"]),
		(Credit, vec!["IN", "Courant", "Épargne", "Crédit"]),
		(Achat, vec!["OUT"]),
		(Virement, vec!["Courant", "Épargne"]),
		(Paiement, vec!["Crédit"]),
	])
}

impl LivreComptable {
	pub fn print_trans_type(&mut self) -> Result<String, ()> {
		let language = match self.abrev_langue.as_str() {
			"fr" => nouv_trans_locale::LANG_FR,
			"es" => nouv_trans_locale::LANG_ES,
			_ => nouv_trans_locale::LANG_EN,
		};

		let elements: Vec<&str> = language.type_name.split('|').collect();

		println!("{}", language.choisir_type);

		println!("╭────┬───────────────────────────╮╭────┬───────────────────────────╮╭────┬───────────────────────────╮");
		println!("│  1 │ {:<25.25} ││  2 │ {:<25.25} ││  3 │ {:<25.25} │", elements[0], elements[1], elements[2]);
		println!("╰────┴───────────────────────────╯╰────┴───────────────────────────╯╰────┴───────────────────────────╯");
		println!("╭────┬───────────────────────────╮╭────┬───────────────────────────╮╭────┬───────────────────────────╮");
		println!("│  4 │ {:<25.25} ││  5 │ {:<25.25} ││  6 │ {:<25.25} │", elements[3], elements[4], elements[5]);
		println!("╰────┴───────────────────────────╯╰────┴───────────────────────────╯╰────┴───────────────────────────╯");

		print!("{}", language.zero_ou_enter);
		io::stdout().flush().unwrap();

		let choix = match get_choix() {
			Ok(n @ 1..=6) => n - 1,
			_ => return Err(()),
		};

		let types: &[(TypeTransaction, &str)] = &[
			(TypeTransaction::Depot, "Dépôt"),
			(TypeTransaction::Debit, "Débit"),
			(TypeTransaction::Credit, "Crédit"),
			(TypeTransaction::Achat, "Achat"),
			(TypeTransaction::Virement, "Virement"),
			(TypeTransaction::Paiement, "Paiement"),
		];

		let (type_choisi, label) = &types[choix as usize];
		self.filtre_compte_cat(type_choisi.clone());
		Ok(label.to_string())
	}

	fn filtre_compte_cat(&mut self, type_tx: TypeTransaction) {

		let comptes_valides = get_comptes_valides();
		let categories_valides = get_categories_valides();

		self.TMP_COMPTES.clear();
		self.TMP_CATEGORIES.clear();

		// Filtrage des comptes
		if let Some(valide_comptes) = comptes_valides.get(&type_tx) {
			for compte in self.COMPTES.iter() {
				if valide_comptes.contains(&compte.cmpt_type.as_str()) {
					self.TMP_COMPTES.push(compte.clone());
				}
			}
		}

		// Filtrage des catégories
		if let Some(valide_categories) = categories_valides.get(&type_tx) {
			for cat in self.CATEGORIES.iter() {
				if valide_categories.contains(&cat.cat_type.as_str()) {
					self.TMP_CATEGORIES.push(cat.clone());
				}
			}
		}
	}

	pub fn modif_favorite(&mut self) -> bool {
		let language = match self.abrev_langue.as_str() {
			"fr" => nouv_trans_locale::LANG_FR,
			"es" => nouv_trans_locale::LANG_ES,
			_ => nouv_trans_locale::LANG_EN,
		};

		if self.FAVORITES.is_empty() {
			println!("{}", language.no_favorite);
			return true;
		}
		println!("{}",language.choix_fav_modif);
		self.printTransactions(&self.FAVORITES, true);
		print!("{}", language.zero_ou_enter);
		io::stdout().flush().unwrap();

		let choix = match get_choix() {
			Ok(n) if n > 0 && (n as usize) <= self.FAVORITES.len() => n - 1,
			_ => { return true; }
		};

		let mut montant: i64 = 0;
		loop {
			let reponse = self.run_context("sans-histoire",
							language.montant_de, false).unwrap_or_default();

			if reponse == "0" { break; } // Un simple break montant déjà à 0.
			if reponse == "" { return true; }

			montant = match string_2_cent(&reponse) {
				Some(v) => v,
				None => {
					println!("{}", language.mauvais_montant);
					continue; // ou return false; selon la logique que tu veux
				},
			};
			println!("----------");
			break;
		};

		let fav_choisi = &mut self.FAVORITES[choix as usize];
		let sql = format!("UPDATE Favorites SET Montant = {} WHERE Description = '{}'",
															montant, fav_choisi.description);
		match self.bd.exec(&sql) {
			Ok(_) => {
				fav_choisi.montant = montant ;
			}
			Err(e) => {
				eprint!("{e} : ");
				return false;
			}
		}
		println!("{}", language.fav_modif_succes.replace("{1}", fav_choisi.description.as_str()));
		self.print1Transaction(&self.FAVORITES[choix as usize].clone());
		return true;
	}
}
