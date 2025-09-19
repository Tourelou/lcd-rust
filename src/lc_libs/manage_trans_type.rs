// manage_trans_type.rs

use super::LivreComptable;
use super::nouv_trans_locale;
use crate::lc_libs::lc_utils::get_choix;

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
}
