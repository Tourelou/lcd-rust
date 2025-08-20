// create_livre.rs

use rusqlite::{params, Connection, Result};
use super::LivreComptable;

#[derive(Debug)]
struct Categorie {
	nom: String,
	utilise: i32,
	type_cat: String,
}


impl LivreComptable {
	pub fn create_db(db_name: &String) -> Result<Connection, String> {
		// Tentative d'ouverture de la base
		let conn = Connection::open(db_name)
			.map_err(|e| format!("Erreur lors de l'ouverture de la base : {}", e))?;

		conn.execute(
			"CREATE TABLE IF NOT EXISTS Master (
				Nom     TEXT,
				Ref     TEXT,
				Type    TEXT,
				Départ  INT,
				Présent INT
			)",
			[],
		).map_err(|e| format!("Erreur lors de la création de la table Master : {}", e))?;

		conn.execute(
			"CREATE TABLE IF NOT EXISTS Catégories (
				Nom TEXT,
				Utilisé INT,
				Type TEXT
			)",
			[],
		).map_err(|e| format!("Erreur lors de la création de la table Catégories : {}", e))?;

		conn.execute(
			"CREATE TABLE IF NOT EXISTS Transactions (
				Date TEXT,
				Description TEXT,
				Type TEXT,
				Compte TEXT,
				Catégorie TEXT,
				Montant INT
			)",
			[],
		).map_err(|e| format!("Erreur lors de la création de la table Transactions : {}", e))?;

		conn.execute(
			"CREATE TABLE IF NOT EXISTS Favorites (
				Date TEXT,
				Description TEXT,
				Type TEXT,
				Compte TEXT,
				Catégorie TEXT,
				Montant INT
			)",
			[],
		).map_err(|e| format!("Erreur lors de la création de la table Favorites : {}", e))?;


		conn.execute(
			"INSERT INTO Catégories (Nom, Utilisé, Type) VALUES (?1, ?2, ?3)",
			params!["Travail", 5, "Débit"],
		).map_err(|e| format!("Erreur lors de l'insertion dans la catégorie : {}", e))?;

		Ok(conn)
	}

	pub fn read_db(conn: Connection) -> Result<bool, String>{
		let mut stmt = conn.prepare("SELECT * FROM Catégories")
								.map_err(|e| format!("Erreur lors de la préparation : {}", e))?;
		let categories = stmt.query_map([], |row| {
			Ok(Categorie {
				nom: row.get("Nom").unwrap(),
				utilise: row.get("Utilisé").unwrap(),
				type_cat: row.get("Type").unwrap(),
			})
		}).map_err(|e| format!("Erreur lors de l'interrogation de la base : {}", e))?;

		for cat in categories {
			let pr = cat.unwrap();
			println!("Nom    : {}", pr.nom);
			println!("Utilisé: {}", pr.utilise);
			println!("Type   : {}", pr.type_cat);
		}
		Ok(true)
	}
}
