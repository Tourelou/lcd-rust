// print_categories.rs

use crate::lc_libs::Categorie;

use super::LivreComptable;

impl LivreComptable {
	pub fn imp_categories(&self, categorie: &Vec<Categorie>) {
		let nbre_categories = categorie.len();
		let nbre_par_ligne = 3;
		let nbre_rangee = nbre_categories / nbre_par_ligne;
		let nbre_reste = nbre_categories % nbre_par_ligne;
		let mut seq = 0;

		// ############################## closure d'impression ########################
		let mut pt = |iterateur: usize| {
			for _ in 0..iterateur {
				print!("╭────┬───────────────────────────╮");
			}
			println!("");
			for _ in 0..iterateur {
				let c = &categorie[seq];
				print!("│ {:>2} │ {:<25.25} │", seq + 1, c.nom);
				seq += 1;
			}
			println!("");
			for _ in 0..iterateur {
				print!("╰────┴───────────────────────────╯")
			}
			println!("");
		};
		// ############################################################################

		for _ in 0..nbre_rangee {
			pt(nbre_par_ligne);
		}
		if nbre_reste > 0 { pt(nbre_reste) }

	}
}