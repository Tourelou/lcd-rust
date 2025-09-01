// ouvre_livre.rs

use std::fs::remove_file;

use crate::lc_libs::{self, Categorie, lc_utils::cent_2_string};
use crate::parse::VarsApp;

use super::LivreComptable;

impl LivreComptable {
	pub fn open_db(var_app: &VarsApp, nouveau: bool) -> Result<(), String> {
		let mut lc = LivreComptable::new(var_app.livre_name.as_ref().unwrap())
			.map_err(|e| format!("{} : {}", var_app.locale.err_open_base, e))?;

		if nouveau {
			println!("----------------------------------------------------------");
			println!("{} {}", var_app.locale.head_nouv_compte ,var_app.date.derniere_entree);
			println!("----------------------------------------------------------");

			loop {
				match lc.ajoute_compte(var_app) {
					Some(c) => lc.COMPTES.push(c),
					None => break,
				}
				println!("----------")
			}
			if lc.COMPTES.is_empty() {
				drop(lc);
				let _ = remove_file(var_app.livre_name.as_ref().unwrap());
				return Err(format!("{}", var_app.locale.exit_no_compte));
			}
			else {
				lc.bd.exec("CREATE TABLE IF NOT EXISTS Master (
						Nom TEXT, Ref TEXT, Type TEXT, Départ INT, Présent INT )",
				).map_err(|e| format!("{} : {}", var_app.locale.err_create_table, e))?;

				for c in &lc.COMPTES {
					let sql_string = format!(
						"INSERT INTO Master (Nom, Ref, Type, Départ, Présent) VALUES ('{}', '{}', '{}', {}, {})",
						c.nom, c.cmpt_ref, c.cmpt_type, c.depart, c.present);
					lc.bd.exec(&sql_string).map_err(|e| format!("{} : {}",	
													var_app.locale.err_ecriure_db, e))?;
				}									
			}	

			println!("------------------------------------");
			println!("{}", var_app.locale.head_nouv_categorie);
			println!("------------------------------------");

			loop {
				match lc.ajoute_categorie(var_app) {
					Some(c) => lc.CATEGORIES.push(c),
					None => break,
				}
				println!("----------")
			}
			if lc.CATEGORIES.is_empty() {
				drop(lc);
				let _ = remove_file(var_app.livre_name.as_ref().unwrap());
				return Err(format!("{}", var_app.locale.exit_no_categorie));
			}
			else {
				lc.bd.exec("CREATE TABLE IF NOT EXISTS Catégories (
						Nom TEXT, Utilisé INT, Type TEXT )",
					).map_err(|e| format!("{} : {}", var_app.locale.err_create_table, e))?;

					for (i, c) in lc.COMPTES.iter().enumerate() {
						let compte_cat: Categorie = Categorie { nom: c.nom.clone(), utilise: 1000,
																cat_type: c.cmpt_type.clone() };
						lc.CATEGORIES.insert(i, compte_cat);

						let sql_string = format!(
							"INSERT INTO Catégories (Nom, Utilisé, Type) VALUES ('{}', 1000, '{}')",
							c.nom, c.cmpt_type);
						lc.bd.exec(&sql_string).map_err(|e| format!("{} : {}",	
														var_app.locale.err_ecriure_db, e))?;
					}

					for c in &lc.CATEGORIES {
						let sql_string = format!(
							"INSERT INTO Catégories (Nom, Utilisé, Type) VALUES ('{}', 0, '{}')",
							c.nom, c.cat_type);
						lc.bd.exec(&sql_string).map_err(|e| format!("{} : {}",	
														var_app.locale.err_ecriure_db, e))?;
					}
			}

//	##################
lc.bd.exec(
	"CREATE TABLE IF NOT EXISTS Transactions (
		Date TEXT, Description TEXT, Type TEXT, Compte TEXT, Catégorie TEXT, Montant INT )",
	).map_err(|e| format!("{} : {}", var_app.locale.err_create_table, e))?;
	
	lc.bd.exec(
		"CREATE TABLE IF NOT EXISTS Favorites (
			Date TEXT, Description TEXT, Type TEXT, Compte TEXT, Catégorie TEXT, Montant INT )",
		).map_err(|e| format!("{} : {}", var_app.locale.err_create_table, e))?;
	}
//	###############################################################################################
		else {	// On transfere les données de la base existante
//	------- MASTER -------
			match lc.bd.query("SELECT * FROM Master") {
				Ok(rows) => {
					for row in rows {
						let master = lc_libs::Compte {
							nom: row.get("Nom").cloned().unwrap_or_default(),
							cmpt_ref: row.get("Ref").cloned().unwrap_or_default(),
							cmpt_type: row.get("Type").cloned().unwrap_or_default(),
							depart: row.get("Départ").and_then(|v| v.parse()
														.ok()).unwrap_or(0),
							present: row.get("Présent").and_then(|v| v.parse()
														.ok()).unwrap_or(0),
						};

						lc.COMPTES.push(master);
					}
				},
				Err(e) => eprintln!("{e}")
			};
//	------- CATÉGORIES -------
			match lc.bd.query("SELECT * FROM Catégories") {
				Ok(rows) => {
					for row in rows {
						let cat = lc_libs::Categorie {
							nom: row.get("Nom").cloned().unwrap_or_default(),
							utilise: row.get("Utilisé").and_then(|v| v.parse()
														.ok()).unwrap_or(0),
							cat_type: row.get("Type").cloned().unwrap_or_default(),
						};
		
						lc.CATEGORIES.push(cat);
					}
				},
				Err(e) => eprintln!("{e}")
			};
//	------- TRANACTIONS -------
			match lc.bd.query("SELECT * FROM Transactions") {
				Ok(rows) => {
					for row in rows {
						let transac = lc_libs::Transaction {
							date: row.get("Date").cloned().unwrap_or_default(),
							description: row.get("Description").cloned().unwrap_or_default(),
							t_type: row.get("Type").cloned().unwrap_or_default(),
							compte: row.get("Compte").cloned().unwrap_or_default(),
							categorie: row.get("Catégorie").cloned().unwrap_or_default(),
							montant: row.get("Montant").and_then(|v| v.parse()
														.ok()).unwrap_or(0),
						};

						lc.TRANSACTIONS.push(transac);
					}
				},
				Err(e) => eprintln!("{e}")
			};
//	------- FAVORITES -------
			match lc.bd.query("SELECT * FROM Favorites") {
				Ok(rows) => {
					for row in rows {
						let transac = lc_libs::Transaction {
							date: row.get("Date").cloned().unwrap_or_default(),
							description: row.get("Description").cloned().unwrap_or_default(),
							t_type: row.get("Type").cloned().unwrap_or_default(),
							compte: row.get("Compte").cloned().unwrap_or_default(),
							categorie: row.get("Catégorie").cloned().unwrap_or_default(),
							montant: row.get("Montant").and_then(|v| v.parse()
														.ok()).unwrap_or(0),
						};

					lc.FAVORITES.push(transac);
					}
				},
				Err(e) => eprintln!("{e}")
			};
		}
		for cmpt in &lc.COMPTES {
//			let d  = (cmpt.depart as f32 /100.0).to_string();
//			let p = (cmpt.present as f32 /100.0).to_string();
			println!("Nom: {} - Ref: {} - Type: {} - Départ: ${} - Présent: ${}",
					cmpt.nom, cmpt.cmpt_ref, cmpt.cmpt_type, cent_2_string(cmpt.depart), cent_2_string(cmpt.present));
//					format!("{}", d.replace(".", ",")), format!("{}", p.replace(".", ",")));
		}

		for c in &lc.CATEGORIES {
			println!("Nom: {} - Utilisé: {} - Type: {}", c.nom, c.utilise, c.cat_type);
		}

		// for t in &lc.TRANSACTIONS {
		// 	println!("Date: {} - Description: {} - Type: {} - Compte: {} - Catégorie: {} - Montant: {}",
		// 				t.date, t.description, t.t_type, t.compte, t.categorie, t.montant);
		// }

		// for f in &lc.FAVORITES {
		// 	println!("Date: {} - Description: {} - Type: {} - Compte: {} - Catégorie: {} - Montant: {}",
		// 				f.date, f.description, f.t_type, f.compte, f.categorie, f.montant);
		// }

		Ok(())
	}
}
