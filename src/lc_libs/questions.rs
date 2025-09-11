// questions.rs

use crate::{lc_libs::{	Transaction,
						lc_utils::cent_2_string
					}, parse::VarsApp};

use super::LivreComptable;

const QUEST_TRANS_FR: &str =
r#"Formulez votre question à la base de données:
----------
╭─────────────────┬───────────────────┬───────────────╮
│ Choix possibles │ Actions possibles │  Motif        │
│─────────────────┼───────────────────┼───────────────│
│ Date            │         =         │    " .. "     │
│ Description     │         !=        │    " .. "     │
│ Type            │         <         │    " .. "     │
│ Compte          │         >         │    " .. "     │
│ Catégorie       │         <=        │    " .. "     │
│ Montant         │         >=        │               │
╰─────────────────┤        LIKE       │   "% .. %"    │
                  │        GLOB       │   "* .. *"    │
                  ╰───────────────────┴───────────────╯
Pour enchaîner les filtres:   AND
----------"#;

const QUEST_TRANS_ES: &str =
r#"Formula tu consulta a la base de datos:
----------
╭───────────────────┬───────────────────┬───────────────╮
│ Posibles opciones │ Posibles acciones │  Motivo       │
│───────────────────┼───────────────────┼───────────────│
│ Date              │         =         │    " .. "     │
│ Description       │         !=        │    " .. "     │
│ Type              │         <         │    " .. "     │
│ Compte            │         >         │    " .. "     │
│ Catégorie         │         <=        │    " .. "     │
│ Montant           │         >=        │               │
╰───────────────────┤        LIKE       │   "% .. %"    │
                    │        GLOB       │   "* .. *"    │
                    ╰───────────────────┴───────────────╯
Para encadenar filtros:    AND
----------"#;

const QUEST_TRANS_EN: &str =
r#"Formulate your query to the database:
----------
╭──────────────────┬──────────────────┬───────────────╮
│ Possible choices │ Possible actions │  Pattern      │
│──────────────────┼──────────────────┼───────────────│
│ Date             │        =         │    " .. "     │
│ Description      │        !=        │    " .. "     │
│ Type             │        <         │    " .. "     │
│ Compte           │        >         │    " .. "     │
│ Catégorie        │        <=        │    " .. "     │
│ Montant          │        >=        │               │
╰──────────────────┤       LIKE       │   "% .. %"    │
                   │       GLOB       │   "* .. *"    │
                   ╰──────────────────┴───────────────╯
To chain filters:         AND
 ----------"#;


pub struct QuestionsStrings {
	pub quest_options: &'static str,
	pub quest_input: &'static str,
	pub sommaire: &'static str,
	pub sql_input: &'static str,
	pub reponse_sql: &'static str,
}

pub const LANG_FR: QuestionsStrings = QuestionsStrings {
	quest_options: QUEST_TRANS_FR,
	quest_input: "====> Votre requête: ",
	sommaire: "│ Crédit      │ Débit       │ Virement    │",
	sql_input: "Entrez une requête SQL: ",
	reponse_sql: "---------- Réponse ----------",
};

pub const LANG_ES: QuestionsStrings = QuestionsStrings {
	quest_options: QUEST_TRANS_ES,
	quest_input: "====> Tu pregunta: ",
	sommaire: "│ Crédito     │ Débito      │ Transf.     │",
	sql_input: "Introduzca una consulta SQL: ",
	reponse_sql: "---------- Respuesta ----------",
};

pub const LANG_EN: QuestionsStrings = QuestionsStrings {
	quest_options: QUEST_TRANS_EN,
	quest_input: "====> Your query: ",
	sommaire: "│ Credit      │ Debit       │ Transfer    │",
	sql_input: "Enter an SQL query: ",
	reponse_sql: "---------- Answer ----------",
};


#[allow(non_snake_case)]
impl LivreComptable {

	pub fn questionBD (&mut self, var_app: &VarsApp) -> bool {
		let language = match var_app.loc_string.as_str() {
			"fr" => LANG_FR,
			"es" => LANG_ES,
			_ => LANG_EN,
	};

		let mut total_credit = 0;
		let mut total_debit = 0;
		let mut total_virement = 0;

		if ! self.contexts.contains_key("question-Transaction") {
			println!("{}", language.quest_options);
		}
		let user_req = match self.run_context("question-Transaction",
												language.quest_input,
												true)  {
			Some(line) => line,
			None => return true,
		};
		if user_req == "" { return true; }
		let sql_req = format!("SELECT * FROM Transactions WHERE {user_req} ORDER BY Date");

		println!("--------------------------------------------------------");
		self.TMP_TRANSACTIONS.clear();
		match self.bd.query(&sql_req.as_str()) {
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
					let trans_type = t_type.clone();
					self.TMP_TRANSACTIONS.push(Transaction {date, description, t_type, compte, categorie, montant});
					match trans_type.as_str() {
						"Dépôt" | "Crédit" => total_credit += montant,
						"Achat" | "Débit" => total_debit += montant,
						"Paiement" | "Virement" => total_virement += montant,
						_ => (),
					}
				}
			}
			Err(e) => eprintln!("{e}"),
		}
		if self.TMP_TRANSACTIONS.is_empty() { return true; }
		else {
			self.printTransactions(&self.TMP_TRANSACTIONS, false);
			println!("                                                            ╭─────────────┬─────────────┬─────────────╮");
			println!("                                                            {}", language.sommaire);
			println!("╭───────────────────────────────────────────────────────────┼─────────────┼─────────────┼─────────────│");
			println!("│ Totaux:                                                   │ {}{:>10.10} │ {}{:>10.10} │ {}{:>10.10} │",
																				self.symbole_monaie, cent_2_string(total_credit),
																				self.symbole_monaie, cent_2_string(total_debit),
																				self.symbole_monaie, cent_2_string(total_virement));
			println!("╰───────────────────────────────────────────────────────────┴─────────────┴─────────────┴─────────────╯");
		}
		return  true;
	}

	pub fn fullQuestionBD(&mut self, var_app: &VarsApp) -> bool {
		let language = match var_app.loc_string.as_str() {
			"fr" => LANG_FR,
			"es" => LANG_ES,
			_ => LANG_EN,
	};

		let user_req = match self.run_context("full-access", language.sql_input, true) {
			Some(line) => line,
			None => return true,
		};
		if user_req == "" { return true; }
		println!("{}", language.reponse_sql);

		match self.bd.query(user_req.as_str()) {
			Ok(rows) => {
				for (i, row) in rows.iter().enumerate() {
					for (key, val) in row {
						println!("{key}: {val}");
					}
					if i < rows.len() - 1 {
						println!("----------");
					}
				}
			},
			Err(e) => eprintln!("{e}"),
		};
		return true;
	}
}
