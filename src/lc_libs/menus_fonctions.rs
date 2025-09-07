// menus_fonctions.rs

use crate::{lc_libs::Categorie, parse::VarsApp};
// use crate::LivreComptable;
// use crate::lc_libs::wrapper_sqlite3::Connection::query;

use super::LivreComptable;

pub struct MenuFonctionsStrings {
	pub create_compte: &'static str,
	pub compte_abort: &'static str,
	pub create_categorie: &'static str,
	pub categorie_abort: &'static str,
	pub sql_input: &'static str,
	pub reponse_sql: &'static str,
}

pub const LANG_FR: MenuFonctionsStrings = MenuFonctionsStrings {
	create_compte: "Création d'une nouveau compte.",
	compte_abort: "Création de compte avortée.",
	create_categorie: "Création d'une nouvelle catégorie.",
	categorie_abort: "Création de catégorie avortée.",
	sql_input: "Entrez une requête SQL: ",
	reponse_sql: "---------- Réponse ----------",
};

pub const LANG_ES: MenuFonctionsStrings = MenuFonctionsStrings {
	create_compte: "Creando una nueva cuenta.",
	compte_abort: "Creación de cuenta cancelada.",
	create_categorie: "Creación de una nueva categoría.",
	categorie_abort: "Creación de categoría  cancelada.",
	sql_input: "Introduzca una consulta SQL: ",
	reponse_sql: "---------- Respuesta ----------",
};

pub const LANG_EN: MenuFonctionsStrings = MenuFonctionsStrings {
	create_compte: "Creating a new account.",
	compte_abort: "Account creation aborted.",
	create_categorie: "Creating a new category.",
	categorie_abort: "Category creation aborted.",
	sql_input: "Enter an SQL query: ",
	reponse_sql: "---------- Answer ----------",
};

// ###########################################################################

pub fn passeur(menu_no: usize, app_vars: &VarsApp, livre: &mut LivreComptable) -> bool {

	let language = match app_vars.loc_string.as_str() {
		"fr" => LANG_FR,
		"es" => LANG_ES,
		_ => LANG_EN,
	};
	println!("\n--------------------------------------------------------");
	match menu_no {
		1 => livre.imp_comptes(&livre.COMPTES),

		2 => { println!("{}", language.create_compte);			// Ajouter un compte
				match livre.ajoute_compte(app_vars) {
					Some(c) => {
						let sql_string = format!(			// D'abord dans la bd
					"INSERT INTO Master (Nom, Ref, Type, Départ, Présent) VALUES ('{}', '{}', '{}', {}, {})",
								c.nom, c.cmpt_ref, c.cmpt_type, c.depart, c.present);
						match livre.bd.exec(&sql_string) {
							Ok(()) => {
								livre.COMPTES.push(c.clone());

								let sql_string = format!(
					"INSERT INTO Catégories (Nom, Utilisé, Type) VALUES ('{}', 1000, '{}')",c.nom, c.cmpt_type);
								match livre.bd.exec(&sql_string){
									Ok(()) => livre.CATEGORIES
												.push(Categorie {nom: c.nom, utilise: 1000, cat_type: c.cmpt_type}),
									Err(_) => return false,
								}
							},		// Et si tout va bien
							Err(_) => return false,
						}
					},
					None => {println!("{}", language.compte_abort)},
				}
			},

		3 => { return livre.supp_compte(&app_vars); },

		4 => livre.imp_categories(&livre.CATEGORIES),

		5 => { println!("{}", language.create_categorie);		// Ajouter une catégorie
				match livre.ajoute_categorie(app_vars) {
					Some(c) => {
						let sql_string = format!(			// D'abord dans la bd
					"INSERT INTO Catégories (Nom, Utilisé, Type) VALUES ('{}', 0, '{}')",
													c.nom, c.cat_type);
						match livre.bd.exec(&sql_string) {
							Ok(()) => livre.CATEGORIES.push(c),		// Et si tout va bien
							Err(_) => return false,
						}
					},
					None => {println!("{}", language.categorie_abort)},
				}
			},

		6 => { return livre.supp_categorie(&app_vars); },

		12 => { let nom = match livre.run_context("full-access",
															language.sql_input,
														true) {
					Some(line) => line,
					None => return true,
				};
				if nom == "" { return true; }
				println!("{}", language.reponse_sql);

				match livre.bd.query(nom.as_str()) {
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
					Err(e) => eprintln!("{e}")
				};
			},
		_ => {println!("Fonction pas encore implémentée.")},
	}
	return true;
}
