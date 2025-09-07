// print_comptes.rs

use crate::lc_libs::Compte;
use crate::lc_libs::lc_utils::cent_2_string;

use super::LivreComptable;

impl LivreComptable {
	pub fn imp_comptes(&self, compte: &Vec<Compte>) {
		let nbre_comptes = compte.len();
		let nbre_par_ligne = 2;
		let nbre_rangee = nbre_comptes / nbre_par_ligne;
		let nbre_reste = nbre_comptes % nbre_par_ligne;
		let mut seq = 0;

		// ############################## closure d'impression ########################
		let mut pt = |iterateur: usize| {
			for _ in 0..iterateur {
				print!("╭────┬──────────────────────────────────┬──────────────╮");
			}
			println!("");
			for _ in 0..iterateur {
				let c = &compte[seq];
				print!("│ {:>2} │ {:<32.32} │ {} {:>10} │",
								seq + 1, c.nom, self.symbole_monaie, cent_2_string(c.depart));
				seq += 1;
			}
			println!("");
			seq -= iterateur;
			for _ in 0..iterateur {
				let c = &compte[seq];
				print!("╰────┤ {:<32.32} │ {} {:>10} │", c.cmpt_ref,
												self.symbole_monaie, cent_2_string(c.present));
				seq += 1;
			}
			println!("");
			for _ in 0..iterateur {
				print!("     ╰──────────────────────────────────┴──────────────╯")
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
