// lc_utils.rs

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

#[allow(unused)]
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
