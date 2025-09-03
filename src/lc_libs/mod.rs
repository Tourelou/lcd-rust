// mod.rs

use std::collections::HashMap;
use std::env;

use crate::lc_libs::wrapper_sqlite3::Connection;
use crate::lc_libs::wrapper_readline::Readline;

#[allow(dead_code)]
#[derive(Debug)]
	pub struct Compte {
		pub nom: String,
		pub cmpt_ref: String,
		pub cmpt_type: String,	// Courant, Épargne, Crédit
		pub depart: i64,	// En cents
		pub present: i64,	// En cents
	}
	/*
	╭────┬──────────────────────────────────┬──────────────╮╭────┬──────────────────────────────────┬──────────────╮
	│  1 │ Compte #1                        │ $    2240,98 ││  2 │ Compte #2                        │ $    3490,88 │
	╰────┤ No de référence 1                │ $    2240,98 │╰────┤ No de référence 2.               │ $    3490,88 │
	     ╰──────────────────────────────────┴──────────────╯     ╰──────────────────────────────────┴──────────────╯
	*/

#[allow(dead_code)]
#[derive(Debug)]
	pub struct Categorie {
		pub nom: String,
		pub utilise: u16,
		pub cat_type: String,	// IN, OUT, Courant, Épargne, Crédit
	}
	/*
	╭────┬───────────────────────────╮╭────┬───────────────────────────╮╭────┬───────────────────────────╮
	│  1 │ Compte #1                 ││  2 │ Compte #2                 ││  3 │ Carte de crédit #1        │
	╰────┴───────────────────────────╯╰────┴───────────────────────────╯╰────┴───────────────────────────╯
	*/
#[allow(dead_code)]
	pub struct Transaction {
		pub date: String,	// Sous forme: 2024-03-24
		pub description: String,
		pub t_type: String,	// Dépôt, Débit, Crédit, Achat, Virement, Paiement
		pub compte: String,
		pub categorie: String,	// Nom de la catégorie
		pub montant: i64,	// En cents
	}
	/*
	╭────────────┬────────────────────────────────┬─────────────┬─────────────────────────────────────────╮
	│ 2024-05-31 │ Hypothèque                     │ Débit       │   De: Compte #1                         │
	╰────────────┤                                │ $    234,56 │ Vers: Hypothèque                        │
				 ╰────────────────────────────────┴─────────────┴─────────────────────────────────────────╯
	*/

#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct LivreComptable {				// ouvre_livre.rs instancie LivreComptable
	pub bd: Connection,
	pub symbole_monaie: String,
	pub readline_inst: Readline,
	pub COMPTES: Vec<Compte>,
	pub CATEGORIES: Vec<Categorie>,
	pub TRANSACTIONS: Vec<Transaction>,
	pub FAVORITES: Vec<Transaction>,
/*
	Des vecteurs temporaires dépendant du type demandé
	Important de faire clear avant usage
*/
	pub TMP_COMPTES: Vec<Compte>,
	pub TMP_CATEGORIES: Vec<Categorie>,
	pub TMP_TRANSACTIONS: Vec<Transaction>,
/*
	Type de transaction: Dépôt, Débit, Crédit, Achat, Virement, Paiement
	Les deux derniers impliquent une double transaction en une seule ligne.

	Opération :                                compte courant | Épargne | carte crédit
	----------------------------------------------------------------------------------
	Dépôt : Paye, remboursement d'impôt ...           +           N/A         N/A
	Débit : Hydro-Québec, Videotron ...               -           N/A          +
	Crédit : Remise dollar carte de crédit            +            +           -
	Achat : Achat avec une carte en magasin           -           N/A          +
	Virement : Transfert d'argent entre comptes     - / +        - / +        N/A
	Paiement : Remboursement de solde (carte)         -           N/A          -
	----------------------------------------------------------------------------------

	Les catégories sont déclarées dynamiquement + l'ajout des comptes
	pour permettre de faire des paiements et des virements.

	Dans le cas où le TYPE est Paiement ou Virement le logiciel présentera
	seulement les catégories appropriées pour ce type de transaction.
*/
//	std::vector<std::string> typeTransaction = {"Dépôt", "Débit", "Crédit", "Achat", "Virement", "Paiement"};
}
#[allow(dead_code)]
impl LivreComptable {
	pub fn new(nom_bd: &String) -> Result<Self, String> {
		Ok(LivreComptable {
			bd: Connection::open(nom_bd)?,
			symbole_monaie: get_monaie(),
			readline_inst: Readline::new(),
			COMPTES: Vec::new(),
			CATEGORIES: Vec::new(),
			TRANSACTIONS: Vec::new(),
			FAVORITES: Vec::new(),
			TMP_COMPTES: Vec::new(),
			TMP_CATEGORIES: Vec::new(),
			TMP_TRANSACTIONS: Vec::new(),
		})
	}
}

fn get_monaie() -> String {
	let locale = env::var("LC_MONETARY")
		.or_else(|_| env::var("LC_ALL"))
		.or_else(|_| env::var("LANG"))
		.unwrap_or_else(|_| "en_US".to_string());

	let cleaned = locale.split('.').next().unwrap_or("en_US");
	let code = cleaned.get(3..5).unwrap_or("US");

	let map = HashMap::from([
		("US", "$"),	// États-Unis
		("CA", "$"),	// Canada
		("FR", "€"),	// France
		("BE", "€"),	// Belgique
		("GB", "£"),	// Grande Bretagne
		("AU", "$"),	// Australie
		("NG", "₦"),	// Nigéria
		("ZA", "R"),	// Afrique du Sud
		("IN", "₹"),	// Inde
		("ES", "€"),	// Espagne
		("MX", "$"),	// Mexique
	]);

	map.get(code).map_or("?", |v| *v).to_string()
}

pub mod wrapper_sqlite3;
pub mod wrapper_readline;
pub mod ouvre_livre;
pub mod ouvre_locale;
pub mod ajoute;
pub mod ajoute_locale;
pub mod menus;
pub mod menus_fonctions;
pub mod lc_utils;

/*
### main.rs, parse.rs, locale.rs et amj_date.rs

!!!!!! lc_libs/mod.rs: Définie les structs et initialise LivreComptable.
!!!!!! wrapper_readline.rs, wrapper_sqlite3.rs: Essentiels pour interragir

### lc_libs/ouvre_livre.rs: Créé ou transfert le data de la bd vers la mémoire.

### lc_libs/ajoute.rs: Sert à ouvre_livre.rs et le menu principal pour
                       initialiser nouveaux Compte et/ou Catégorie

### lc_libs/menus.rs: Affiche le menu et via la fonction passeur()
                      réparti les commandes.

### lc_utils.rs: fonctions string_2_cent() et cent_2-string()



// lc_initLivre.cpp
✅	bool initLivre();
✅	void incantationSQL(std::string, std::string);

// lc_utils.cpp
	void twistAccent(const char *, int &);
	bool testMontant(std::string &);
✅	std::string dollars2cents(const std::string &);	// Convert dollars to cents
✅	std::string cents2dollars(const std::string &);	// Convert cents to dollars

	bool sortTransDateAsc(transaction, transaction);
	bool sortTransDateDesc(transaction, transaction); 

	bool getReponse(const int);

// lc_publish.cpp
	void publishTransaction(struct transaction&);

// lc_const_dest.cpp
✅	LivreComptable();
	~LivreComptable();	// Plus besoin

// lc_ouvre_ferme.cpp
✅	bool ouvreLivre(std::string, bool);	// Dans ouvre_livre.rs
✅	void fermeLivre();					// Plus besoin

// lc_ajoute.cpp
✅	bool ajoutCompte();		// Dans ajoute.rs
✅	bool ajoutCategorie();	// Dans ajoute.rs

// lc_supprime.cpp
	void suppCompte();
	void suppCategorie();
	void suppFavorite();
	void modifFavorite();

// lc_sommaire_mois.cpp
	void sommaireMois();

// lc_set2mem.cpp
✅	int setMaster2mem(char **);			// Dans ouvre_livre.rs
✅	int setCategories2mem(char **);		// Dans ouvre_livre.rs
✅	int setFavorites2mem(char **);		// Dans ouvre_livre.rs
✅	int setTransactions2mem(char **);	// Dans ouvre_livre.rs
✅	int setQuestion2mem(char **);		// Dans ouvre_livre.rs

// lc_questions.cpp
	void questionBD();
	void fullQuestionBD();

// lc_print_comptes.cpp
	void printComptes(std::vector<struct compte>&);
	void printAllComptes();

// lc_print_categories.cpp
	void printType();
	void printCategories(std::vector<struct categorie>&);
	void printAllCategories();

// lc_print_transactions.cpp
	void print1Transaction(struct transaction&);
	void printTransactions(std::vector<struct transaction>&, bool = false);
	void printAllFavorites();

// lc_nouvelle_trans.cpp
	bool nouvelleTransaction(bool = false);
*/
