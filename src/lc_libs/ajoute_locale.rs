// ajoute_locale.rs

const  TYPE_COMPTE_FR: &str =
"Type de compte|Compte courant|Compte épargne|Carte de crédit|Votre choix";
const  TYPE_CATEGORIE_FR: &str =
"Type de catégorie|En entrée ex.: Paye|En Sortie ex.: Nourriture|Votre choix";

const TYPE_COMPTE_ES: &str =
"Tipo de cuenta|Cuenta corriente|Cuenta de ahorros|Tarjeta de crédito|Su elección";
const TYPE_CATEGORIE_ES: &str =
"Tipo de categoría|Ingreso ej.: Sueldo|Gasto ej.: Comida|Su elección";

const TYPE_COMPTE_EN: &str =
"Account type|Checking account|Savings account|Credit card|Your choice";
const TYPE_CATEGORIE_EN: &str =
"Category type|Income e.g.: Salary|Expense e.g.: Food|Your choice";


#[derive(Debug)]
pub struct AjouteString {
	pub nom_compte: &'static str,			// ajoute.rs
	pub ref_compte: &'static str,			// ajoute.rs
	pub type_compte: &'static str,			// ajoute.rs
	pub depart_compte: &'static str,		// ajoute.rs
	pub nom_categorie: &'static str,		// ajoute.rs
	pub type_categorie: &'static str,		// ajoute.rs
}

pub const LANG_FR: AjouteString = AjouteString {
	nom_compte:    "        Nom du compte       : ",
	ref_compte:    "        # de référence      : ",
	type_compte: TYPE_COMPTE_FR,
	depart_compte: "        Montant de départ   : ",
	nom_categorie: "        Nom de catégorie    : ",
	type_categorie: TYPE_CATEGORIE_FR,
};

pub const LANG_ES: AjouteString = AjouteString {
	nom_compte:    "        Nombre de cuenta    : ",
	ref_compte:    "        Nº de referencia    : ",
	type_compte: TYPE_COMPTE_ES,
	depart_compte: "        Monto inicial       : ",
	nom_categorie: "        Nombre de categoría : ",
	type_categorie: TYPE_CATEGORIE_ES,
};

pub const LANG_EN: AjouteString = AjouteString {
	nom_compte:    "        Account name        : ",
	ref_compte:    "        Reference #         : ",
	type_compte: TYPE_COMPTE_EN,
	depart_compte: "        Starting amount     : ",
	nom_categorie: "        Category name       : ",
	type_categorie: TYPE_CATEGORIE_EN,
};
