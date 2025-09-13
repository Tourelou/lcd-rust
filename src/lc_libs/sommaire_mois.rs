// sommaire_mois.rs

use crate::lc_libs::lc_utils::{cent_2_string, get_choix};
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

#[allow(unused)]
pub struct SomMoisStrings {
	som_mois: &'static [&'static str],
	quel_compte: &'static str,
	err_num_trop_grand: &'static str,
	err_criteres: &'static str,
	compte_mois: &'static str,
	marquise: &'static [&'static str],
	fin_ligne: &'static str,
}

pub const LANG_FR: SomMoisStrings = SomMoisStrings {
	som_mois: &MOIS_FR,
	quel_compte: "De quel compte désirez-vous le sommaire ? ",
	err_num_trop_grand: "Le nombre en entrée est trop grand.",
	err_criteres: "La réponse ne correspond pas aux critères.",
	compte_mois: "Sommaire de «{1}» pour le mois de {2}",
	marquise: &HEADER_FR,
	fin_ligne: "Totaux du mois",
};

pub const LANG_ES: SomMoisStrings = SomMoisStrings {
	som_mois: &MOIS_ES,
	quel_compte: "¿Para qué cuenta quieres el resumen? ",
	err_num_trop_grand: "El número de entrada es demasiado grande.",
	err_criteres: "La respuesta no coincide con los criterios.",
	compte_mois: "Resumen de «{1}» para el mes de {2}",
	marquise: &HEADER_ES,
	fin_ligne: "Totales del mes"
};

pub const LANG_EN: SomMoisStrings = SomMoisStrings {
	som_mois: &MOIS_EN,
	quel_compte: "Which account do you want the summary for ? ",
	err_num_trop_grand: "The input number is too large.",
	err_criteres: "The answer does not match the criteria.",
	compte_mois: "Summary of «{1}» for the month of {2}",
	marquise: &HEADER_EN,
	fin_ligne: "Monthly totals"
};

#[allow(non_snake_case, unused)]
impl LivreComptable {
	pub fn sommaireMois(&mut self) -> bool {
		let language = match self.abrev_langue.as_str() {
			"fr" => LANG_FR,
			"es" => LANG_ES,
			_ => LANG_EN,
		};

		self.TMP_TRANSACTIONS.clear();
		println!("{}", language.quel_compte);
		self.imp_comptes(&self.COMPTES);
		print!("===> ");
		io::stdout().flush().unwrap();

		let reponse = match get_choix() {
			Ok(n) => {
				println!("----------");
				if n == 0 { return true; }
				else if n as usize > self.COMPTES.len() {
					println!("{}", language.err_num_trop_grand);
					return true;
				}
				else { n }
			}
			Err(()) => {
				println!("----------");
				println!("{}", language.err_criteres);
				return true;
			}
		};
		let mut start = self.COMPTES[reponse as usize - 1].depart;
		let credit = self.COMPTES[reponse as usize - 1]. cmpt_type == "Crédit";
		let selection = self.COMPTES[(reponse - 1) as usize].nom.clone();
		let header = format!("│ {:<10.10} │ {:<50.50} │ {:>10.10} │ {:>10.10} │ {:>10.10} │",
			language.marquise[0], language.marquise[1], language.marquise[2], language.marquise[3], language.marquise[4]);
		let footer = language.fin_ligne;

		// ########################## closure d'impression ############################

		let mut imprime = |this: &Self| {
			let mut totalCredit = 0;
			let mut totalDebit = 0;
			let mut totalMois = 0;

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
					if credit { start -= tt.montant; }
					else { start += tt.montant; }
					println!(" {:>10.10} │            │ {:>10.10} │", cent_2_string(tt.montant), cent_2_string(start));
				}
				else {
					totalDebit += tt.montant;
					if credit { start += tt.montant; }
					else { start -= tt.montant; }
					println!("            │ {:>10.10} │ {:>10.10} │", cent_2_string(tt.montant), cent_2_string(start));
				}
			}
			totalMois = totalCredit - totalDebit;
			println!("             ╭────────────────────────────────────────────────────┬────────────┬────────────┬────────────╮");
			println!("             │ {:<50.50} │ {:>10.10} │ {:>10.10} │ {:>10.10} │", footer,
										cent_2_string(totalCredit), cent_2_string(totalDebit), cent_2_string(totalMois));
			println!("             ╰────────────────────────────────────────────────────┴────────────┴────────────┴────────────╯");
		};
		// ############################################################################

		for m in 1..13 {
			let mois = format!("-{:02.02}-", m);
			for t in &self.TRANSACTIONS {
				if t.date.contains(&mois) && (t.compte == selection || t.categorie == selection) {
					self.TMP_TRANSACTIONS.push(t.clone());
				}
			}
			if self.TMP_TRANSACTIONS.is_empty() { continue; }
			println!("{}", format!("{}", language.compte_mois).replace("{1}", &selection)
														.replace("{2}", language.som_mois[m - 1]));
			imprime(self);
			if m != 12 { println!("----------"); }
			self.TMP_TRANSACTIONS.clear();
		}
	return true;
	}
}
