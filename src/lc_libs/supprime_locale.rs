// supprime_locale.rs

use crate::parse;

pub struct SupprimeStrings {
	pub question_supp_compte: &'static str,
	pub question_cmpt_numero: &'static str,
	pub no_cmpt_2_del: &'static str,
	pub question_supp_cat: &'static str,
	pub question_cat_numero: &'static str,
	pub no_cat_2_del: &'static str,
	pub pas_de_prob: &'static str,
	pub succes_supp_cmpt: &'static str,
	pub succes_supp_cat: &'static str,
	pub err_num_trop_grand: &'static str,
	pub err_criteres: &'static str,
}

pub const LANG_FR: SupprimeStrings = SupprimeStrings {
	question_supp_compte: "Voulez vous supprimer ce compte ? [ n/N pour annuler ] ",
	question_cmpt_numero: "Choisir le compte à supprimer par son #  [ 0 pour annuler ] : ",
	no_cmpt_2_del: "Aucun compte ne peut être supprimé.",
	question_supp_cat: "Voulez vous supprimer cete catégorie ? [ n/N pour annuler ] ",
	question_cat_numero: "Choisir la catégorie à supprimer par son #  [ 0 pour annuler ] : ",
	no_cat_2_del: "Aucun catégorie ne peut être supprimé.",
	pas_de_prob: "Pas de problème, on ne supprime rien.",
	succes_supp_cmpt: "Compte «{1}» supprimée avec succès.",
	succes_supp_cat: "Catégorie «{1}» supprimée avec succès.",
	err_num_trop_grand: "Rien n'est supprimé: Le nombre en entrée est trop grand.",
	err_criteres: "Rien n'est supprimé: La réponse ne correspond pas aux critères.",
};

pub const LANG_ES: SupprimeStrings = SupprimeStrings {
	question_supp_compte: "¿Quieres eliminar esta cuenta? [ n/N para cancelar ] ",
	question_cmpt_numero: "Seleccione la cuenta a eliminar por su # [ 0 para cancelar ]: ",
	no_cmpt_2_del: "No se puede eliminar ninguna cuenta",
	question_supp_cat: "¿Quieres eliminar esta categoría? [ n/N para cancelar ] ",
	question_cat_numero: "Seleccione la categoría a eliminar por su # [ 0 para cancelar ]: ",
	no_cat_2_del: "No se puede eliminar ninguna categoría.",
	pas_de_prob: "No hay problema, no borraremos nada.",
	succes_supp_cmpt: "La cuenta «{1}» se eliminó correctamente.",
	succes_supp_cat: "La categoría «{1}» se eliminó correctamente.",
	err_num_trop_grand: "No se elimina nada: el número de entrada es demasiado grande.",
	err_criteres: "No se elimina nada: la respuesta no coincide con los criterios.",
};

pub const LANG_EN: SupprimeStrings = SupprimeStrings {
	question_supp_compte: "Do you want to delete this account? [ n/N to cancel ] ",
	question_cmpt_numero: "Select the account to delete by its # [ 0 to cancel ]: ",
	no_cmpt_2_del: "No account can be deleted.",
	question_supp_cat: "Do you want to delete this category? [ n/N to cancel ] ",
	question_cat_numero: "Select the category to delete by its # [ 0 to cancel ]: ",
	no_cat_2_del: "No category can be deleted.",
	pas_de_prob: "No problem, we're not deleting anything.",
	succes_supp_cmpt: "Account «{1}» successfully deleted.",
	succes_supp_cat: "Category «{1}» successfully deleted.",
	err_num_trop_grand: "Nothing is deleted: The input number is too large.",
	err_criteres: "Nothing is deleted: The answer does not match the criteria.",
};

pub fn set_supprime_lang(appvar: &parse::VarsApp) -> SupprimeStrings {
	match appvar.loc_string.as_str() {
		"fr" => LANG_FR,
		"es" => LANG_ES,
		_ => LANG_EN,
	}
}
