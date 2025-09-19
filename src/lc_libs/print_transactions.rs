// print_transactions.rs

use crate::lc_libs::Transaction;
use crate::lc_libs::lc_utils::{cent_2_string, split_lignes};
use super::LivreComptable;

pub const TRANS_TYPE_FR: [&str; 6] = [
	"Dépôt",
	"Débit",
	"Crédit",
	"Achat",
	"Virement",
	"Paiement",
];

pub const TRANS_TYPE_ES: [&str; 6] = [
	"Depósito",
	"Débito",
	"Crédito",
	"Compra",
	"Transferencia",
	"Pago",
];

pub const TRANS_TYPE_EN: [&str; 6] = [
	"Deposit",
	"Debit",
	"Credit",
	"Purchase",
	"Transfer",
	"Payment",
];

pub struct TransactionStrings {
	trans_type: &'static [&'static str],
	de_str: &'static str,
	vers_str: &'static str,
}

pub const LANG_FR: TransactionStrings = TransactionStrings {
	trans_type: &TRANS_TYPE_FR,
	de_str: "De  :",
	vers_str: "Vers:",
};

pub const LANG_ES: TransactionStrings = TransactionStrings {
	trans_type: &TRANS_TYPE_ES,
	de_str: "De  :",
	vers_str: "Para:",
};

pub const LANG_EN: TransactionStrings = TransactionStrings {
	trans_type: &TRANS_TYPE_EN,
	de_str: "From:",
	vers_str: "To  :",
};


#[allow(non_snake_case)]
impl LivreComptable {
	pub fn print1Transaction(&mut self, t: &Transaction) {
		self.TMP_TRANSACTIONS.clear();
		self.TMP_TRANSACTIONS.push(t.clone());
		self.printTransactions(&self.TMP_TRANSACTIONS, false);
	}

	pub fn printTransactions(&self, trans: &Vec<Transaction>, count: bool) {
		if trans.len() > 0 {
		let language = match self.abrev_langue.as_str() {
			"fr" => LANG_FR,
			"es" => LANG_ES,
			_ => LANG_EN,
		};
		let bottom = trans.len();
			if count {
				for (i, t) in trans.iter().enumerate() {
					let (l1, l2) = split_lignes(&t.description, 30);
					let new_type = match t.t_type.as_str() {
						"Dépôt" => language.trans_type[0],
						"Débit" => language.trans_type[1],
						"Crédit" => language.trans_type[2],
						"Achat" => language.trans_type[3],
						"Virement" => language.trans_type[4],
						"Paiement" => language.trans_type[5],
						_ => "- - - - - -",
					};
					if i == 0 {
						println!("╭────┬────────────────────────────────┬─────────────┬─────────────────────────────────────────╮");
					}
					else {
						println!("╭────┤────────────────────────────────┼─────────────┼─────────────────────────────────────────│");
					}
					println!("│ {:>2.2} │ {:<30.30} │ {:<11.11} │ {} {:<33.33} │",
								i + 1, l1, new_type, language.de_str, t.compte);
					println!("╰────┤ {:<30.30} │ {}{:>10.10} │ {} {:<33.33} │",
													l2, self.symbole_monaie, cent_2_string(t.montant), language.vers_str, t.categorie);
					if i + 1 == bottom {
						println!("     ╰────────────────────────────────┴─────────────┴─────────────────────────────────────────╯");
					}
				}
			}
			else {
				for (i, t) in trans.iter().enumerate() {
					let (l1, l2) = split_lignes(&t.description, 30);
					let new_type = match t.t_type.as_str() {
						"Dépôt" => language.trans_type[0],
						"Débit" => language.trans_type[1],
						"Crédit" => language.trans_type[2],
						"Achat" => language.trans_type[3],
						"Virement" => language.trans_type[4],
						"Paiement" => language.trans_type[5],
						_ => "- - - - - -",
					};
					if i == 0 {
						println!("╭────────────┬────────────────────────────────┬─────────────┬─────────────────────────────────────────╮");
					}
					else {
						println!("╭────────────┤────────────────────────────────┼─────────────┼─────────────────────────────────────────│");
					}
					println!("│ {:<10.10} │ {:<30.30} │ {:<11.11} │ {} {:<33.33} │",
								t.date, l1, new_type, language.de_str, t.compte);
					println!("╰────────────┤ {:<30.30} │ {}{:>10.10} │ {} {:<33.33} │",
													l2, self.symbole_monaie, cent_2_string(t.montant), language.vers_str, t.categorie);
					if i + 1 == bottom {
						println!("             ╰────────────────────────────────┴─────────────┴─────────────────────────────────────────╯");
					}
				}
			}
		}
	}
}
