// ouvre_livre.rs

use std::fs::remove_file;

use crate::lc_libs::{self, Categorie};
use crate::lc_libs::menus;
use crate::lc_libs::ouvre_locale;
use crate::parse::VarsApp;

use super::LivreComptable;

impl LivreComptable {
	pub fn open_db(var_app: &mut VarsApp, nouveau: bool) -> Result<(), String> {
		let language = ouvre_locale::set_ouvre_lang(&var_app);
		let mut lc = LivreComptable::new(var_app.livre_name
								.as_ref()
								.unwrap())
								.map_err(|e| format!("{} : {}", language.err_open_base, e))?;
		lc.abrev_langue = var_app.loc_string.clone();
		if nouveau {
			println!("----------------------------------------------------------");
			println!("{} {}", language.head_nouv_compte ,var_app.date.derniere_entree);
			println!("----------------------------------------------------------");

			match lc.bd.exec("CREATE TABLE IF NOT EXISTS Master (
					Nom TEXT, Ref TEXT, Type TEXT, Départ INT, Présent INT )") {
				Ok(()) => {
					loop {
						match lc.ajoute_compte() {
							Some(c) => {
								let sql_string = format!(
			"INSERT INTO Master (Nom, Ref, Type, Départ, Présent) VALUES ('{}', '{}', '{}', {}, {})",
								c.nom, c.cmpt_ref, c.cmpt_type, c.depart, c.present);
								lc.bd.exec(&sql_string)
								.map_err(|e| format!("{} : {}", language.err_ecriure_db, e))?;
								lc.COMPTES.push(c)
							},
							None => break,
						}
						println!("----------")
					}
				},
				Err(e) => {
					drop(lc);
					let _ = remove_file(var_app.livre_name.as_ref().unwrap());
					return Err(format!("{} : {}", language.err_create_table, e))
				},
			}
			if lc.COMPTES.is_empty() {
				drop(lc);
				let _ = remove_file(var_app.livre_name.as_ref().unwrap());
				return Err(format!("{}", language.exit_no_compte));
			}
			println!("------------------------------------");
			println!("{}", language.head_nouv_categorie);
			println!("------------------------------------");

			match lc.bd.exec("CREATE TABLE IF NOT EXISTS Catégories (
									Nom TEXT, Utilisé INT, Type TEXT )") {
				Ok(()) => {
					for c in &lc.COMPTES {
						let compte_cat: Categorie = Categorie { nom: c.nom.clone(), utilise: 1000,
																cat_type: c.cmpt_type.clone() };
						let sql_string = format!(
							"INSERT INTO Catégories (Nom, Utilisé, Type) VALUES ('{}', 1000, '{}')",
							c.nom, c.cmpt_type);
						lc.bd.exec(&sql_string).map_err(|e| format!("{} : {}",	
														language.err_ecriure_db, e))?;
						lc.CATEGORIES.push(compte_cat);
					}
					loop {
						match lc.ajoute_categorie() {
							Some(c) => {
								let sql_string = format!(
					"INSERT INTO Catégories (Nom, Utilisé, Type) VALUES ('{}', 0, '{}')",
													c.nom, c.cat_type);
								lc.bd.exec(&sql_string)
											.map_err(|e| format!("{} : {}",
														language.err_ecriure_db, e))?;
								lc.CATEGORIES.push(c)
							},
							None => break,
						}
						println!("----------")
					}
				},
				Err(e) => {
					drop(lc);
					let _ = remove_file(var_app.livre_name.as_ref().unwrap());
					return Err(format!("{} : {}", language.err_create_table, e))
				},
			}
			if lc.CATEGORIES.is_empty() {
				drop(lc);
				let _ = remove_file(var_app.livre_name.as_ref().unwrap());
				return Err(format!("{}", language.exit_no_categorie));
			}

			//	##################
			lc.bd.exec(
				"CREATE TABLE IF NOT EXISTS Transactions (
				Date TEXT, Description TEXT, Type TEXT, Compte TEXT, Catégorie TEXT, Montant INT )",)
						.map_err(|e| format!("{} : {}", language.err_create_table, e))?;
	
			lc.bd.exec(
				"CREATE TABLE IF NOT EXISTS Favorites (
				Date TEXT, Description TEXT, Type TEXT, Compte TEXT, Catégorie TEXT, Montant INT )",)
						.map_err(|e| format!("{} : {}", language.err_create_table, e))?;

		}
		//	###############################################################################################
		else {	// On transfere les données de la base existante
//	------- MASTER -------
			match lc.bd.query("SELECT * FROM Master") {
				Ok(rows) => {
					for row in rows {
						let mut nom = String::new();
						let mut cmpt_ref = String::new();
						let mut cmpt_type = String::new();
						let mut depart = 0;
						let mut present = 0;

						for (col, val) in row {
							match col.as_str() {
								"Nom"     => nom       = val,
								"Ref"     => cmpt_ref  = val,
								"Type"    => cmpt_type = val,
								"Départ"  => depart    = val.parse().unwrap_or(0),
								"Présent" => present   = val.parse().unwrap_or(0),
								_ => {}
							}
						}
						lc.COMPTES.push(lc_libs::Compte {
							nom, cmpt_ref, cmpt_type, depart, present
						});
					}
				},
				Err(e) => eprintln!("{e}")
			};
//	------- CATÉGORIES -------
			match lc.bd.query("SELECT * FROM Catégories") {
				Ok(rows) => {
					for row in rows {
						let mut nom = String::new();
						let mut utilise = 0;
						let mut cat_type = String::new();

						for (col, val) in row {
							match col.as_str() {
								"Nom"     => nom      = val,
								"Utilisé" => utilise  = val.parse().unwrap_or(0),
								"Type"    => cat_type = val,
								_ => {}
							}
						}
						lc.CATEGORIES.push(lc_libs::Categorie {nom, utilise, cat_type});
					}
				},
				Err(e) => eprintln!("{e}")
			};
//	------- TRANACTIONS -------
			match lc.bd.query("SELECT * FROM Transactions") {
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
						lc.TRANSACTIONS.push(lc_libs::Transaction {date, description, t_type, compte, categorie, montant});
					}
				},
				Err(e) => eprintln!("{e}")
			};
//	------- FAVORITES -------
			match lc.bd.query("SELECT * FROM Favorites") {
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
						let transaction = lc_libs::Transaction { date, description,
															t_type, compte, categorie, montant };

						// --- DEBUT DE LA VALIDATION ---
						let mut erreurs = Vec::new();

						if !lc.COMPTES.iter().any(|c| c.nom == transaction.compte) {
							erreurs.push(format!("{}", language.not_in_compte.replace("{1}", &transaction.compte)));
						}
						if !lc.CATEGORIES.iter().any(|cat| cat.nom == transaction.categorie) {
							erreurs.push(format!("{}", language.not_in_categorie.replace("{1}", &transaction.categorie)));
						}

						if erreurs.is_empty() { lc.FAVORITES.push(transaction); }
						else {
							// Ça cloche : on affiche tout ce qu'on a ramassé
							eprintln!("{}", language.alerte_load_favorite);
							lc.print1Transaction(&transaction);

							for err in erreurs { eprintln!("===> {err}"); }
							eprintln!("{}", language.fav_tran_delete.replace("{1}", var_app.livre_name.as_ref().unwrap()));

							let query = format!("DELETE FROM Favorites WHERE Description = \"{}\" AND Montant = {}",
												transaction.description, transaction.montant);

							if let Err(e) = lc.bd.exec(&query.as_str()) {
								eprintln!("Erreur lors de la suppression : {e}");
							}
						}
						// --- FIN DE LA VALIDATION ---
					}
				},
				Err(e) => eprintln!("{e}")
			};
		}
		menus::affiche_menu(var_app, &mut lc);
		Ok(())
	}
}


