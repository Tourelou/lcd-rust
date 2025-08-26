// create_livre.rs

use crate::lc_libs;
use crate::parse::VarsApp;

use super::LivreComptable;
use super::Connection;

impl LivreComptable {
	pub fn open_db(var_app: &VarsApp, nouveau: bool) -> Result<Connection, String> {

		let mut lc = LivreComptable::new(var_app.livre_name.as_ref().unwrap())
				.map_err(|e| format!("Erreur lors de l'ouverture de la base : {}", e))?;

		if nouveau {
			lc.bd.exec(
				"CREATE TABLE IF NOT EXISTS Master (
					Nom     TEXT,
					Ref     TEXT,
					Type    TEXT,
					Départ  INT,
					Présent INT
				)",
			).map_err(|e| format!("Erreur lors de la création de la table Master : {}", e))?;

			lc.bd.exec(
				"CREATE TABLE IF NOT EXISTS Catégories (
					Nom TEXT,
					Utilisé INT,
					Type TEXT
				)",
			).map_err(|e| format!("Erreur lors de la création de la table Catégories : {}", e))?;

			lc.bd.exec(
				"CREATE TABLE IF NOT EXISTS Transactions (
					Date TEXT,
					Description TEXT,
					Type TEXT,
					Compte TEXT,
					Catégorie TEXT,
					Montant INT
				)",
			).map_err(|e| format!("Erreur lors de la création de la table Transactions : {}", e))?;

			lc.bd.exec(
				"CREATE TABLE IF NOT EXISTS Favorites (
					Date TEXT,
					Description TEXT,
					Type TEXT,
					Compte TEXT,
					Catégorie TEXT,
					Montant INT
				)",
			).map_err(|e| format!("Erreur lors de la création de la table Favorites : {}", e))?;
		}
		else {	// On transfere les données de la base existante
//	------- MASTER -------
			let rows = lc.bd.query("SELECT * FROM Master")?;
			for row in rows {
				let master = lc_libs::Compte {
					nom: row.get("Nom").cloned().unwrap_or_default(),
					cmpt_ref: row.get("Ref").cloned().unwrap_or_default(),
					cmpt_type: row.get("Type").cloned().unwrap_or_default(),
					depart: row.get("Départ").and_then(|v| v.parse().ok()).unwrap_or(0),
					present: row.get("Présent").and_then(|v| v.parse().ok()).unwrap_or(0),
				};

				lc.COMPTES.push(master);
			}
			for cmpt in lc.COMPTES {
				println!("Nom: {} - Ref: {} - Type: {} - Départ: {} - Présent: {}",
							cmpt.nom, cmpt.cmpt_ref, cmpt.cmpt_type, cmpt.depart/100, cmpt.present/100);
			}
//	------- CATÉGORIES -------
			let rows = lc.bd.query("SELECT * FROM Catégories")?;
			for row in rows {
					let cat = lc_libs::Categorie {
					nom: row.get("Nom").cloned().unwrap_or_default(),
					utilise: row.get("Utilisé").and_then(|v| v.parse().ok()).unwrap_or(0),
					cat_type: row.get("Type").cloned().unwrap_or_default(),
				};

				lc.CATEGORIES.push(cat);
			}
			for c in lc.CATEGORIES {
				println!("Nom: {} - Utilisé: {} - Type: {}", c.nom, c.utilise, c.cat_type);
			}
//	------- TRANACTIONS -------
			let rows = lc.bd.query("SELECT * FROM Transactions")?;
			for row in rows {
				let transac = lc_libs::Transaction {
					date: row.get("Date").cloned().unwrap_or_default(),
					description: row.get("Description").cloned().unwrap_or_default(),
					t_type: row.get("Type").cloned().unwrap_or_default(),
					compte: row.get("Compte").cloned().unwrap_or_default(),
					categorie: row.get("Catégorie").cloned().unwrap_or_default(),
					montant: row.get("Montant").and_then(|v| v.parse().ok()).unwrap_or(0),
				};

				lc.TRANSACTIONS.push(transac);
			}
			for t in lc.TRANSACTIONS {
				println!("Date: {} - Description: {} - Type: {} - Compte: {} - Catégorie: {} - Montant: {}",
							t.date, t.description, t.t_type, t.compte, t.categorie, t.montant);
			}
//	------- FAVORITES -------
			let rows = lc.bd.query("SELECT * FROM Favorites")?;
			for row in rows {
				let fav = lc_libs::Transaction {
					date: row.get("Date").cloned().unwrap_or_default(),
					description: row.get("Description").cloned().unwrap_or_default(),
					t_type: row.get("Type").cloned().unwrap_or_default(),
					compte: row.get("Compte").cloned().unwrap_or_default(),
					categorie: row.get("Catégorie").cloned().unwrap_or_default(),
					montant: row.get("Montant").and_then(|v| v.parse().ok()).unwrap_or(0),
				};

				lc.FAV_FAVORITES.push(fav);
			}
			for f in lc.FAV_FAVORITES {
				println!("Date: {} - Description: {} - Type: {} - Compte: {} - Catégorie: {} - Montant: {}",
							f.date, f.description, f.t_type, f.compte, f.categorie, f.montant);
			}
		}
		Ok(lc.bd)
	}
}
