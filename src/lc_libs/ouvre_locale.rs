// ouvre_locale.rs

use crate::parse;

#[derive(Debug)]
pub struct OuvreString {
	pub err_open_base: &'static str,		// ouvre_livre.rs
	pub err_create_table: &'static str,		// ouvre_livre.rs
	pub exit_no_compte: &'static str,		// ouvre_livre.rs
	pub exit_no_categorie: &'static str,	// ouvre_livre.rs
	pub err_ecriure_db: &'static str,		// ouvre_livre.rs
	pub head_nouv_compte: &'static str,		// ouvre_livre.rs
	pub head_nouv_categorie: &'static str,	// ouvre_livre.rs
	pub not_in_compte: &'static str,		// ouvre_livre.rs
	pub not_in_categorie: &'static str,		// ouvre_livre.rs
	pub alerte_load_favorite: &'static str,	// ouvre_livre.rs
	pub fav_tran_delete: &'static str,		// ouvre_livre.rs
}

pub const LANG_FR: OuvreString = OuvreString {
	err_open_base: "Erreur lors de l'ouverture de la base",
	err_create_table: "Erreur lors de la création de tables essentielles.",
	exit_no_compte: "Ne peux pas marcher si aucun compte n'est créé",
	exit_no_categorie: "Ne peux pas marcher si aucune catégorie n'est créé",
	err_ecriure_db: "Erreur en écriture vers la base de données",
	head_nouv_compte: "Création de nouveau(x) compte(s) aujourd'hui le",
	head_nouv_categorie: "Création de nouvelle(s) catégorie(s)",
	not_in_compte: "'{1}' ne figure pas dans vos comptes.",
	not_in_categorie: "'{1}' ne figure pas dans vos catégories.",
	alerte_load_favorite: "------\nAlerte : Problème avec la transaction favorite suivante :",
	fav_tran_delete: "Cette transaction sera supprimée de '{1}'\n------",
};

pub const LANG_ES: OuvreString = OuvreString {
	err_open_base: "Error al abrir la base de datos",
	err_create_table: "Error al crear las tablas esenciales.",
	exit_no_compte: "No se puede continuar si no se ha creado ninguna cuenta",
	exit_no_categorie: "No se puede continuar si no se ha creado ninguna categoría",
	err_ecriure_db: "Error al escribir en la base de datos",
	head_nouv_compte: "Creación de nueva(s) cuenta(s) hoy en",
	head_nouv_categorie: "Creación de nueva(s) categoría(s)",
	not_in_compte: "'{1}' no figura en sus cuentas.",
	not_in_categorie: "'{1}' no figura en sus categorías.",
	alerte_load_favorite: "------\nAlerta: Problema con la siguiente transacción favorita:",
	fav_tran_delete: "Esta transacción será eliminada de '{1}'\n------",
};

pub const LANG_EN: OuvreString = OuvreString {
	err_open_base: "Error opening the database",
	err_create_table: "Error creating essential tables.",
	exit_no_compte: "Cannot proceed if no account is created",
	exit_no_categorie: "Cannot proceed if no category is created",
	err_ecriure_db: "Error writing to database",
	head_nouv_compte: "Creating new account(s) today on",
	head_nouv_categorie: "Creating new category(ies)",
	not_in_compte: "'{1}' is not in your accounts.",
	not_in_categorie: "'{1}' is not in your categories.",
	alerte_load_favorite: "------\nWarning: Issue with the following favorite transaction:",
	fav_tran_delete: "This transaction will be deleted from '{1}'\n------",
};

pub fn set_ouvre_lang(appvar: &parse::VarsApp) -> OuvreString {
	match appvar.loc_string.as_str() {
		"fr" => LANG_FR,
		"es" => LANG_ES,
		_ => LANG_EN,
	}
}
