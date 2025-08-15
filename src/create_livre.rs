use rusqlite::{params, Connection};

pub fn create_db(db_name: &String) {
	// Tentative d'ouverture de la base
	let conn = match Connection::open(db_name) {
		Ok(c) => {
			println!("Base ouverte avec succès.");
			c
		}
		Err(e) => {
			eprintln!("Échec d'ouverture de la base : {}", e);
			return;
		}
	};

	if let Err(e) = conn.execute(
		"CREATE TABLE IF NOT EXISTS Master (
			Nom     TEXT,
			Ref     TEXT,
			Type    TEXT,
			Départ  INT,
			Présent INT
		)",
		[],
	) {
		eprintln!("Échec de création de la table Master : {}", e);
		return;
	}

	if let Err(e) = conn.execute(
		"CREATE TABLE IF NOT EXISTS Catégories (
			Nom TEXT,
			Utilisé INT,
			Type TEXT
		)",
		[],
	) {
		eprintln!("Échec de création de la table Catégories : {}", e);
		return;
	}

	if let Err(e) = conn.execute(
		"CREATE TABLE IF NOT EXISTS Transactions (
			Date TEXT,
			Description TEXT,
			Type TEXT,
			Compte TEXT,
			Catégorie TEXT,
			Montant INT
		)",
		[],
	) {
		eprintln!("Échec de création de la table Transactions : {}", e);
		return;
	}

	if let Err(e) = conn.execute(
		"CREATE TABLE IF NOT EXISTS Favorites (
			Date TEXT,
			Description TEXT,
			Type TEXT,
			Compte TEXT,
			Catégorie TEXT,
			Montant INT
		)",
		[],
	) {
		eprintln!("Échec de création de la table Favorites : {}", e);
		return;
	}

	// Test insertion
	match conn.execute(
		"INSERT INTO Catégories (Nom, Utilisé, Type) VALUES (?1, ?2, ?3)",
		params!["Travail", 5, "Débit"],
	) {
		Ok(_) => println!("Insertion réussie pour Travail"),
		Err(e) => eprintln!("Échec d'insertion pour Travail : {}", e),
	}
	
	// Lecture des données
	let mut stmt = match conn.prepare("SELECT * FROM Catégories") {
		Ok(s) => s,
		Err(e) => {
			eprintln!("Échec de préparation de la requête SELECT : {}", e);
			return;
		}
	};

	let categories = match stmt.query_map([], |row| {
		Ok((
			row.get::<_, String>(0)?,
			row.get::<_, i32>(1)?,
			row.get::<_, String>(2)?,
		))
	}) {
		Ok(p) => p,
		Err(e) => {
			eprintln!("Échec de lecture des données : {}", e);
			return;
		}
	};

	println!("Liste des catégories :");
	for categorie in categories {
		match categorie {
			Ok((nom, utilise, types)) => println!("Nom: {}, Utilisé: {}, Type: {}", nom, utilise, types),
			Err(e) => eprintln!("Erreur lors de la lecture d'une ligne : {}", e),
		}
	}
}
