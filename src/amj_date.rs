use std::process::Command;
use std::fmt;

use crate::locale::LangStrings;

pub struct AMJDate {
	annee: u32,
	mois: u32,
	jour: u32,
	pub aujourdhui: String,
	pub derniere_entree: String,
}

impl AMJDate {
	pub fn new(l: &LangStrings) -> Option<Self> {
		// Essaye d'obtenir la date d'aujourd'hui
		let (annee, mois, jour) = match Self::get_today() {
			Some(date) => date,
			None => match Self::ask_for_date(l) {
				Some(date) => date,
				None => return None, // Si aucune date n'est disponible, on quitte avec None
			},
		};

		// Si on a une date, on construit la structure
		Some(AMJDate {
			annee,
			mois,
			jour,
			aujourdhui: format!("{:04}-{:02}-{:02}", annee, mois, jour),
			derniere_entree: format!("{:04}-{:02}-{:02}", annee, mois, jour),
		})
	}

	fn get_today() -> Option<(u32, u32, u32)> {
		let output = Command::new("date")
			.arg("+%Y-%m-%d")
			.output()
			.ok()?;

		let date_str = String::from_utf8_lossy(&output.stdout);
		let parts: Vec<&str> = date_str.trim().split('-').collect();

		if parts.len() == 3 {
			let year = parts[0].parse().ok()?;
			let month = parts[1].parse().ok()?;
			let day = parts[2].parse().ok()?;
			Some((year, month, day))
		}
		else { None }
	}

	fn ask_for_date(l: &LangStrings) -> Option<(u32, u32, u32)> {
		use std::io::{self, Write};

		println!("{}", l.err_date);
		print!("> ");
		io::stdout().flush().ok()?; // Utilise `?` pour sortir proprement en cas d'erreur

		let mut input = String::new();
		io::stdin().read_line(&mut input).ok()?; // Même chose ici

		let trimmed = input.trim();
		if !Self::test_date(trimmed) {
			return None;
		}

		let parts: Vec<&str> = trimmed.split('-').collect();
		if parts.len() != 3 {
			return None;
		}

		let y = parts[0].parse().ok()?;
		let m = parts[1].parse().ok()?;
		let d = parts[2].parse().ok()?;

		Some((y, m, d))
	}

	fn is_leap_year(year: u32) -> bool {
		(year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
	}

	fn days_per_month(year: u32) -> [u32; 12] {
		let mut ndjs = [31, 28, 31, 30, 31, 30,
									31, 31, 30, 31, 30, 31];
		if Self::is_leap_year(year) {
			ndjs[1] = 29;
		}
		ndjs
	}

	fn normalize_date_format(date: &str) -> String {
		if date.len() == 10 && date.chars().nth(4) == Some('/') && date.chars().nth(7) == Some('/') {
			date.replace("/", "-")
		}
		else { date.to_string() }
	}

	pub fn test_date(date: &str) -> bool {
		let date = Self::normalize_date_format(date);

		if date.len() != 10 {
			return false;
		}

		let sep1 = date.chars().nth(4);
		let sep2 = date.chars().nth(7);
		if !(sep1 == Some('-') && sep2 == Some('-')) {
			return false;
		}

		if !date.chars().enumerate().all(|(i, c)| {
			if i == 4 || i == 7 {
				true
			} else {
				c.is_ascii_digit()
			}
		}) {
			return false;
		}

		let a = date[0..4].parse::<u32>().unwrap_or(0);
		let m = date[5..7].parse::<usize>().unwrap_or(0);
		let j = date[8..10].parse::<u32>().unwrap_or(0);

		if a < 1970 || a > 9999 || m < 1 || m > 12 {
			return false;
		}

		let ndjs = Self::days_per_month(a);
		j >= 1 && j <= ndjs[m - 1]
	}

	pub fn set_check_date(&mut self, date: &str) -> bool {
		let mut s = match date.len() {
			10 => date.to_string(),
			5 => format!("{}-{}", &self.derniere_entree[0..4], date),
			2 => format!("{}-{}-{}", &self.derniere_entree[0..4],
									&self.derniere_entree[5..7], date),
			_ => return false,
		};

		s = Self::normalize_date_format(&s);

		if !Self::test_date(&s) {
			return false;
		}

		self.annee = s[0..4].parse().unwrap_or(0);
		self.mois = s[5..7].parse().unwrap_or(0);
		self.jour = s[8..10].parse().unwrap_or(0);
		self.derniere_entree = s;
		true
	}
}

impl fmt::Display for AMJDate {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}-{:02}-{:02}", self.annee, self.mois, self.jour)
	}
}
