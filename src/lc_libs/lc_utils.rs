// lc_utils.rs

// use std::process::Command;

// pub fn enable_raw_mode() {
// 	// Sauvegarde les paramètres actuels du terminal
// 	let _ = Command::new("stty")
// 						.arg("-echo")
// 						.arg("raw")
// 						.status();
// }

// pub fn disable_raw_mode() {
// 	// Restaure les paramètres du terminal
// 	let _ =	Command::new("stty")
// 						.arg("echo")
// 						.arg("-raw")
// 						.status()
// 						.expect("Échec de la désactivation du mode brut");
// }

pub fn string_2_cent(valeur: &String) -> Option<i64> {

	let neg = valeur.chars().nth(0) == Some('-');
	let s: &str;

	if neg { s = &valeur[1..]; }
	else { s = &valeur[0..]; }

	if s.is_empty() { return None; }
	let mut conv_string = s.replace(" ", "");

	if !conv_string.chars().all(|c| c.is_ascii_digit() || c == '.' || c == ',') { return None; }

	let p: Vec<_> = conv_string.match_indices(|c| c == '.' || c == ',').collect();

	if p.len() > 1 { return None; }
	if p.is_empty() { conv_string.push_str("00"); }
	else {
		let pos = p[0].0;
		if pos < conv_string.len() - 3 { return None; }
		else if pos == conv_string.len() - 1 { conv_string.push_str("00"); }
		else if pos == conv_string.len() - 2 { conv_string.push('0');}
	}
	if neg {conv_string.insert(0, '-');}

	conv_string.replace(&[',', '.'][..], "").parse::<i64>().ok()
}

pub fn cent_2_string(valeur: i64) -> String {

	let neg = valeur < 0;
	let dollars = (valeur / 100).abs();
	let cents = (valeur % 100).abs();
	let mut sortie = String::new();

	if neg { sortie.push('-'); }
	if dollars > 1000 {
		sortie.push_str(format!("{} {:03},", dollars/1000, dollars%1000).as_str());
	}
	else { sortie.push_str(format!("{},", dollars).as_str()); }
	sortie.push_str(format!("{:02}", cents).as_str());

	sortie
}

pub fn get_choix() -> Result<u8, ()> {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();

	let line = line.trim(); // Enlève les retours à la ligne et les espaces
	match line.parse::<u8>() {
		Ok(n) => Ok(n),
		Err(_) => Err(()),
	}
}

pub fn split_lignes(source: &String, cut: usize) -> (String, String) {
	// Découpe la source en mots
	let mots: Vec<&str> = source.split_whitespace().collect();
	let mut ligne1 = String::new();
	let mut ligne2 = String::new();

	// Closure pour calculer la "vraie" longueur visuelle d'un mot
	let real_len = |s: &str| -> usize {
		let mut pad = 0;
		let mut chars = s.chars().peekable();

		while let Some(c) = chars.next() {
			let code = c as u32;

			// Ajoute du padding pour les caractères Unicode larges
			if code >= 0x0080 { pad += 1; }
			if code >= 0x0800 { pad += 1; }
			if code >= 0x10000 { pad += 1; }
		}

		s.len() - pad
	};
	// ###########################################################

	let mut l1_len = cut;
	let mut l1_full = false;

	for mot in mots {
		let rl_mot = real_len(mot);

		if !l1_full {
			if rl_mot <= l1_len {
				ligne1.push_str(mot);
				l1_len -= rl_mot;

				if l1_len > 0 {
					ligne1.push(' ');
					l1_len -= 1;
				} else {
					l1_full = true;
				}
			} else {
				// Mot trop long pour ligne1 → on passe à ligne2
				l1_full = true;
				ligne2.push_str(mot);
				ligne2.push(' ');
			}
		} else {
			ligne2.push_str(mot);
			ligne2.push(' ');
		}
	}

	// Alignement final avec padding
	ligne1 = format!("{:<1$.1$}", ligne1.trim_end(), cut);
	ligne2 = format!("{:<1$.1$}", ligne2.trim_end(), cut);

	(ligne1, ligne2)
}
