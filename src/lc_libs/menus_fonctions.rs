// menus_fonctions.rs

use crate::parse::VarsApp;
use crate::LivreComptable;
// use crate::lc_libs::wrapper_sqlite3::Connection::query;
use crate::lc_libs::Readline;

pub fn passeur(menu_no: usize, app_vars: &VarsApp, livre: &mut LivreComptable) -> bool {
	println!("\n----------");
	match menu_no {
		2 => {	println!("Création d'une nouveau compte.");			// Ajouter un compte
				match livre.ajoute_compte(app_vars) {
					Some(c) => {
						let sql_string = format!(			// D'abord dans la bd
			"INSERT INTO Master (Nom, Ref, Type, Départ, Présent) VALUES ('{}', '{}', '{}', {}, {})",
								c.nom, c.cmpt_ref, c.cmpt_type, c.depart, c.present);
						match livre.bd.exec(&sql_string) {
							Ok(()) => livre.COMPTES.push(c),		// Et si tout va bien
							Err(_) => return false,
						}
					},
					None => {println!("Création de compte avortée.")},
				}
			},
		5 => {	println!("Création d'une nouvelle catégorie.");		// Ajouter une catégorie
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
					None => {println!("Création de catégorie avortée.")},
				}
			},
		12 => {
				let mut rl = Readline::new();
				let nom = match rl.read_line("Entrez une requête SQL: ", false) {
					Some(line) => line,
					None => return true,
				};
				if nom == "" { return true; }
				println!("---------- Réponse ----------");

				match livre.bd.query(nom.as_str()) {
					Ok(rows) => {
						for row in rows {
							for (key, val) in row {
								println!("{key}: {val}");
							}
							println!("----------");
						}
					},
					Err(e) => eprintln!("{e}")
				};
			},
		_ => {println!("Fonction pas encore implémentée.")},
	}
	return true;
}
