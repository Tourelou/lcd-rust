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
	aucune_trans: &'static str,
	de_str: &'static str,
	vers_str: &'static str,
}

pub const LANG_FR: TransactionStrings = TransactionStrings {
	trans_type: &TRANS_TYPE_FR,
	aucune_trans: "Aucune transaction à afficher.",
	de_str: "De  :",
	vers_str: "Vers:",
};

pub const LANG_ES: TransactionStrings = TransactionStrings {
	trans_type: &TRANS_TYPE_ES,
	aucune_trans: "No hay transacciones para mostrar.",
	de_str: "De  :",
	vers_str: "Para:",
};

pub const LANG_EN: TransactionStrings = TransactionStrings {
	trans_type: &TRANS_TYPE_EN,
	aucune_trans: "No transactions to display.",
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

	pub fn printTransactions(&self, trans: &[Transaction], count: bool) {
		let language = match self.abrev_langue.as_str() {
			"fr" => LANG_FR,
			"es" => LANG_ES,
			_ => LANG_EN,
		};

		if trans.is_empty() {
			println!("{}", language.aucune_trans);
			return;
		}

		let bottom = trans.len();

		for (i, t) in trans.iter().enumerate() {
			let (l1, l2) = split_lignes(&t.description, 30);
			
			// On extrait la logique du type de transaction
			let new_type = match t.t_type.as_str() {
				"Dépôt"    => language.trans_type[0],
				"Débit"    => language.trans_type[1],
				"Crédit"   => language.trans_type[2],
				"Achat"    => language.trans_type[3],
				"Virement" => language.trans_type[4],
				"Paiement" => language.trans_type[5],
				_ => "- - - - - -",
			};

			// Configuration dynamique selon 'count'
			let (col1_val, sep_mid, sep_final_padding) = if count {
				(format!("{:>2.2}", i + 1), "────", "     ")
			} else {
				(format!("{:<10.10}", t.date), "────────────", "             ")
			};

			// --- AFFICHAGE ---

			// 1. Ligne du haut (différente si c'est la première ou les suivantes)
			if i == 0 {
				println!("╭{}┬────────────────────────────────┬─────────────┬─────────────────────────────────────────╮", sep_mid);
			} else {
				// Le fameux raccord ╭────┤ que tu voulais
				println!("╭{}┤────────────────────────────────┼─────────────┼─────────────────────────────────────────│", sep_mid);
			}

			// 2. Contenu ligne 1
			println!("│ {} │ {:<30.30} │ {:<11.11} │ {} {:<33.33} │",
					col1_val, l1, new_type, language.de_str, t.compte);

			// 3. Contenu ligne 2 (avec le raccord du bas ╰────┤)
			println!("╰{}┤ {:<30.30} │ {}{:>10.10} │ {} {:<33.33} │",
					sep_mid, l2, self.symbole_monaie, cent_2_string(t.montant), language.vers_str, t.categorie);

			// 4. Ligne de fermeture finale (uniquement pour la toute dernière transaction)
			if i + 1 == bottom {
				println!("{}╰────────────────────────────────┴─────────────┴─────────────────────────────────────────╯", sep_final_padding);
			}
		}
	}
}
