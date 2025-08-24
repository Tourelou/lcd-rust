// mod.rs

use  rusqlite::{self, Connection};
use crate::amj_date::get_date;

#[allow(dead_code)]
	struct Compte {
		nom: String,
		cmpt_ref: String,
		cmpt_type: String,	// Courant, Épargne, Crédit
		depart: String,
		present: String,
	}
	/*
	┌────┬──────────────────────────────────┬──────────────┐┌────┬──────────────────────────────────┬──────────────┐
	│  1 │ Compte #1                        │ $    2240,98 ││  2 │ Compte #2                        │ $    3490,88 │
	└────┤ -                                │ $    2240,98 │└────┤ -                                │ $    3490,88 │
		 └──────────────────────────────────┴──────────────┘     └──────────────────────────────────┴──────────────┘
	*/

#[allow(dead_code)]
	struct Categorie {
		nom: String,
		utilise: u16,
		cat_type: String,	// IN, OUT, Courant, Épargne, Crédit
	}
	/*
	┌────┬───────────────────────────┐┌────┬───────────────────────────┐┌────┬───────────────────────────┐
	│  1 │ Compte #1                 ││  2 │ Compte #2                 ││  3 │ Carte de crédit           │
	└────┴───────────────────────────┘└────┴───────────────────────────┘└────┴───────────────────────────┘
	*/
#[allow(dead_code)]
	struct Transaction {
		date: String,	// Sous forme: 2024-03-24
		description: String,
		t_type: String,	// Dépôt, Débit, Crédit, Achat, Virement, Paiement
		compte: String,
		catégorie: String,	// Nom de la catégorie
		montant: i64,	// En cents
	}
	/*
	┌────────────┬────────────────────────────────┬─────────────┬─────────────────────────────────────────┐
	│ 2024-05-31 │ Hypothèque                     │ Débit       │   De: Compte #2                         │
	└────────────┤                                │ $    234,56 │ Vers: Hypothèque                        │
				 └────────────────────────────────┴─────────────┴─────────────────────────────────────────┘
	*/

#[allow(dead_code)]
#[allow(non_snake_case)]
pub struct LivreComptable {
	bd: rusqlite::Connection,
	d: String,	// Date
	line_read: Option<String>,
	COMPTES: Vec<Compte>,
	CATEGORIES: Vec<Categorie>,
	TRANSACTIONS: Vec<Transaction>,
	FAV_FAVORITES: Vec<Transaction>,
/*
	Des vecteurs temporaires dépendant du type demandé
	Important de faire clear avant usage
*/
	TMP_COMPTES: Vec<Compte>,
	TMP_CATEGORIES: Vec<Categorie>,
	TMP_TRANSACTIONS: Vec<Transaction>,
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
	pub fn new(nom_bd: &String) -> Result<Self, rusqlite::Error> {
		Ok(LivreComptable {
			bd: Connection::open(nom_bd)?,
			d: get_date(),
			line_read: None,
			COMPTES: Vec::new(),
			CATEGORIES: Vec::new(),
			TRANSACTIONS: Vec::new(),
			FAV_FAVORITES: Vec::new(),
			TMP_COMPTES: Vec::new(),
			TMP_CATEGORIES: Vec::new(),
			TMP_TRANSACTIONS: Vec::new(),
		})
	}
}

pub mod create_livre;

/*
// lc_initLivre.cpp
	bool initLivre();
	void incantationSQL(std::string, std::string);

// lc_utils.cpp
	void twistAccent(const char *, int &);
	bool testMontant(std::string &);
	std::string dollars2cents(const std::string &);	// Convert dollars to cents
	std::string cents2dollars(const std::string &);	// Convert cents to dollars

// lc_publish.cpp
	void publishTransaction(struct transaction&);

// lc_const_dest.cpp
	LivreComptable();
	~LivreComptable();

// lc_ouvre_ferme.cpp
	bool ouvreLivre(std::string, bool);
	void fermeLivre();

// lc_ajoute.cpp
	bool ajoutCompte();
	bool ajoutCategorie();

// lc_supprime.cpp
	void suppCompte();
	void suppCategorie();
	void suppFavorite();
	void modifFavorite();

// lc_sommaire_mois.cpp
	void sommaireMois();

// lc_set2mem.cpp
	int setMaster2mem(char **);
	int setCategories2mem(char **);
	int setFavorites2mem(char **);
	int setTransactions2mem(char **);
	int setQuestion2mem(char **);

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

// lc_utils.cpp
bool sortTransDateAsc(transaction, transaction);
bool sortTransDateDesc(transaction, transaction); 

bool getReponse(const int);
*/
