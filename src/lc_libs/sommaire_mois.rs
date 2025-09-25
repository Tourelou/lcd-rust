// sommaire_mois.rs

use crate::lc_libs::{lc_utils::{cent_2_string, get_choix}, Transaction};
use std::io::{self, Write};

use super::LivreComptable;

pub const MOIS_FR: [&str; 12] = [
	"janvier", "février", "mars", "avril",
	"mai", "juin", "juillet", "août",
	"septembre", "octobre", "novembre", "décembre",
];

pub const MOIS_ES: [&str; 12] = [
	"enero", "febrero", "marzo", "abril",
	"mayo", "junio", "julio", "agosto",
	"septiembre", "octubre", "noviembre", "diciembre",
];

pub const MOIS_EN: [&str; 12] = [
	"January", "February", "March", "April",
	"May", "June", "July", "August",
	"September", "October", "November", "December",
];

pub const HEADER_FR: [&str; 5] = [
	"Date", "Description", "Crédit", "Débit", "Total",
];

pub const HEADER_ES: [&str; 5] = [
	"Fecha", "Descripción", "Crédito", "Débito", "Total",
];

pub const HEADER_EN: [&str; 5] = [
	"Date", "Description", "Credit", "Debit", "Total",
];

pub struct SomMoisStrings {
	som_mois: &'static [&'static str],
	quel_compte: &'static str,
	err_num_trop_grand: &'static str,
	err_criteres: &'static str,
	aucune_transaction: &'static str,
	compte_mois: &'static str,
	marquise: &'static [&'static str],
	fin_ligne: &'static str,
}

pub const LANG_FR: SomMoisStrings = SomMoisStrings {
	som_mois: &MOIS_FR,
	quel_compte: "De quel compte désirez-vous le sommaire ? ",
	err_num_trop_grand: "Le nombre en entrée est trop grand.",
	err_criteres: "La réponse ne correspond pas aux critères.",
	aucune_transaction: "Aucune transaction pour ce compte.",
	compte_mois: "Sommaire de «{1}» pour {2}",
	marquise: &HEADER_FR,
	fin_ligne: "Totaux du mois",
};

pub const LANG_ES: SomMoisStrings = SomMoisStrings {
	som_mois: &MOIS_ES,
	quel_compte: "¿Para qué cuenta quieres el resumen? ",
	err_num_trop_grand: "El número de entrada es demasiado grande.",
	err_criteres: "La respuesta no coincide con los criterios.",
	aucune_transaction: "No hay transacciones para esta cuenta.",
	compte_mois: "Resumen de «{1}» para {2}",
	marquise: &HEADER_ES,
	fin_ligne: "Totales del mes"
};

pub const LANG_EN: SomMoisStrings = SomMoisStrings {
	som_mois: &MOIS_EN,
	quel_compte: "Which account do you want the summary for ? ",
	err_num_trop_grand: "The input number is too large.",
	err_criteres: "The answer does not match the criteria.",
	aucune_transaction: "No transactions for this account.",
	compte_mois: "Summary of «{1}» for {2}",
	marquise: &HEADER_EN,
	fin_ligne: "Monthly totals"
};

#[allow(non_snake_case)]
impl LivreComptable {
	pub fn sommaireMois(&mut self) -> bool {
		let language = match self.abrev_langue.as_str() {
			"fr" => LANG_FR,
			"es" => LANG_ES,
			_ => LANG_EN,
		};

		println!("{}", language.quel_compte);
		self.imp_comptes(&self.COMPTES);
		print!("===> ");
		io::stdout().flush().unwrap();

		let reponse = match get_choix() {
			Ok(n) => {
				if n == 0 { return true; }
				else if n as usize > self.COMPTES.len() {
					println!("{}", language.err_num_trop_grand);
					return true;
				}
				else { n }
			}
			Err(()) => {
				println!("{}", language.err_criteres);
				return true;
			}
		};
		let ce_compte = &self.COMPTES[reponse as usize - 1];
		let mut trans_du_compte: Vec<Transaction> = Vec::new();
		let req_sql = format!(
			"SELECT * FROM Transactions WHERE Compte = '{}' OR Catégorie = '{}' ORDER BY Date",
													ce_compte.nom, ce_compte.nom
		);
		match self.bd.query(&req_sql) {
			Ok(rows) => {
				for row in rows {
					let mut date = String::new();
					let mut description = String::new();
					let mut t_type = String::new();
					let mut compte = String::new();
					let mut categorie = String::new();
					let mut montant = 0;
					for (col, val) in row {
						match col.as_str() {
							"Date"        => date        = val,
							"Description" => description = val,
							"Type"        => t_type      = val,
							"Compte"      => compte      = val,
							"Catégorie"   => categorie   = val,
							"Montant"     => montant     = val.parse().unwrap_or(0),
							_ => {}
						}
					}
					trans_du_compte.push(Transaction {date, description, t_type, compte, categorie, montant});
				}
			},
			Err(e) => eprintln!("{e}")
		};

		if trans_du_compte.is_empty() {
			println!("{}", language.aucune_transaction);
			return true;
		}

		let mut solde = ce_compte.depart;
		let credit = ce_compte.cmpt_type == "Crédit";
		let selection = ce_compte.nom.clone();

		let header = format!("│ {:<10.10} │ {:<50.50} │ {:>10.10} │ {:>10.10} │ {:>10.10} │",
			language.marquise[0], language.marquise[1], language.marquise[2], language.marquise[3], language.marquise[4]);
		let footer = language.fin_ligne;

		// ########################## closure d'impression ############################

		let mut imprime_mois = |this: &Self| {
			let mut totalCredit = 0;
			let mut totalDebit = 0;
			let totalMois;

			let mois = &this.TMP_TRANSACTIONS[0].date[5..7].to_string().parse().unwrap();
			let ans = &this.TMP_TRANSACTIONS[0].date[0..4];
			println!("{} {}", format!("{}", language.compte_mois).replace("{1}", &selection)
										.replace("{2}", language.som_mois[(mois - 1) as usize]),
						ans);

			println!("╭────────────┬────────────────────────────────────────────────────┬────────────┬────────────┬────────────╮");
			println!("{header}");
			println!("╰────────────┴────────────────────────────────────────────────────┴────────────┴────────────┴────────────╯");
			for tt in &this.TMP_TRANSACTIONS {
				let mut closure_type = tt.t_type.clone();
				if tt.t_type == "Virement" || tt.t_type == "Paiement" {
					if selection == tt.categorie { closure_type = "Crédit".to_string(); }
					else { closure_type = "Débit".to_string(); }
				}
				print!("│ {} │ {:<50.50} │", tt.date, tt.description);
				if closure_type == "Crédit" || closure_type == "Dépôt" {
					totalCredit += tt.montant;
					if credit { solde -= tt.montant; }
					else { solde += tt.montant; }
					println!(" {:>10.10} │            │ {:>10.10} │", cent_2_string(tt.montant), cent_2_string(solde));
				}
				else {
					totalDebit += tt.montant;
					if credit { solde += tt.montant; }
					else { solde -= tt.montant; }
					println!("            │ {:>10.10} │ {:>10.10} │", cent_2_string(tt.montant), cent_2_string(solde));
				}
			}
			totalMois = totalCredit - totalDebit;
			println!("             ╭────────────────────────────────────────────────────┬────────────┬────────────┬────────────╮");
			println!("             │ {:<50.50} │ {:>10.10} │ {:>10.10} │ {:>10.10} │", footer,
										cent_2_string(totalCredit), cent_2_string(totalDebit), cent_2_string(totalMois));
			println!("             ╰────────────────────────────────────────────────────┴────────────┴────────────┴────────────╯");
		};
		// ############################################################################
		let mut idx_trs_cmte = 0;
		let trs_cmte_len = trans_du_compte.len();
		let mut an_mois = trans_du_compte[idx_trs_cmte].date.clone()[0..7].to_string();

		self.TMP_TRANSACTIONS.clear();
		while idx_trs_cmte < trs_cmte_len {
			if trans_du_compte[idx_trs_cmte].date.contains(&an_mois) {
				self.TMP_TRANSACTIONS.push(trans_du_compte[idx_trs_cmte].clone());
				idx_trs_cmte += 1;
			}
			else {
				imprime_mois(self);
				println!("----------");
				self.TMP_TRANSACTIONS.clear();
				an_mois = trans_du_compte[idx_trs_cmte].date.clone()[0..7].to_string();
			}
		}
		imprime_mois(self);

		return true;
	}
}
