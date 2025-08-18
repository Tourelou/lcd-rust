// create_livre.rs

use rusqlite::{params, Connection};

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
