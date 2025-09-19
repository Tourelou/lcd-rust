// menus.rs - Gestion des menus

use std::io::{self, Read, Write};

use crate::{lc_libs::LivreComptable, parse::VarsApp};
use crate::lc_libs::menus_fonctions::passeur;

pub const OPTIONS_FR: [&str; 14] = [
	"1- Entrer une nouvelle transaction",
	"2- Afficher tous les comptes",
	"3- Ajouter un compte",
	"4- Supprimer un compte",
	"5- Afficher toutes les catégories",
	"6- Ajouter une catégorie",
	"7- Supprimer une catégorie",
	"8- Afficher toutes les favorites",
	"9- Ajouter une transaction favorite",
	"a- Modifier une transaction favorite",
	"b- Supprimer une transaction favorite",
	"c- Interroger les transactions",
	"d- Accès complet à la bd [PRO ONLY]",
	"e- Sommaire de comptes",
];

pub const OPTIONS_ES: [&str; 14] = [
	"1- Ingresar una nueva transacción",
	"2- Mostrar todas las cuentas",
	"3- Agregar una cuenta",
	"4- Eliminar una cuenta",
	"5- Mostrar todas las categorías",
	"6- Agregar una categoría",
	"7- Eliminar una categoría",
	"8- Mostrar todos los favoritos",
	"9- Agregar una transacción favorita",
	"a- Modificar una transacción favorita",
	"b- Eliminar una transacción favorita",
	"c- Consultar transacciones",
	"d- Acceso completo a la base de datos [SOLO PRO]",
	"e- Resumen de cuentas",
];

pub const OPTIONS_EN: [&str; 14] = [
	"1- Enter a new transaction",
	"2- Show all accounts",
	"3- Add an account",
	"4- Delete an account",
	"5- Show all categories",
	"6- Add a category",
	"7- Delete a category",
	"8- Show all favorites",
	"9- Add a favorite transaction",
	"a- Edit a favorite transaction",
	"b- Delete a favorite transaction",
	"c- Query transactions",
	"d- Full DB access [PRO ONLY]",
	"e- Account summary",
];

pub struct MenuStrings {
	pub options: &'static [&'static str],
	pub menu_usage: &'static str,
	pub err_majeure: &'static str,
	pub aurevoir: &'static str,
}

pub const LANG_FR: MenuStrings = MenuStrings {
	options: &OPTIONS_FR,
	menu_usage: "Choisir un item avec les flèches haut/bas puis enter ⏎\n --- « 0 » pour quitter",
	err_majeure: "Erreur majeur - Sortie du programme.",
	aurevoir: "À la prochaine ...",
};

pub const LANG_ES: MenuStrings = MenuStrings {
	options: &OPTIONS_ES,
	menu_usage: "Seleccione un elemento con las flechas arriba/abajo y luego ingrese ⏎\n --- « 0 » para salir",
	err_majeure: "Error mayor: salida del programa",
	aurevoir: "¡Hasta la próxima!...",
};

pub const LANG_EN: MenuStrings = MenuStrings {
	options: &OPTIONS_EN,
	menu_usage: "Select an item with the up/down arrows then enter ⏎\n --- « 0 » to exit",
	err_majeure: "Major Error - Program Exit.",
	aurevoir: "See you next time...",
};
// ###########################################################################

fn reposition_cursor_and_clear(lines: usize) {
	print!("\x1B[{}A", lines);
	for _ in 0..lines {
		print!("\x1B[2K\r");
		print!("\x1B[1B");
	}
	print!("\x1B[{}A", lines);
	io::stdout().flush().unwrap();
}

pub fn affiche_menu(var_app: &mut VarsApp, livre: &mut LivreComptable) {
	let mut selected = 0;

	let language = match var_app.loc_string.as_str() {
		"fr" => LANG_FR,
		"es" => LANG_ES,
		_ => LANG_EN,
	};
// ################### Définition des closures ############################
	let mode_brute_on = || {
		let _ = std::process::Command::new("stty")
										.arg("-echo")
										.arg("cbreak")
										.status();
	};
	let mode_brute_off = || {
		let _ = std::process::Command::new("stty")
										.arg("echo")
										.arg("-cbreak")
										.status();
	};
	// Closure pour afficher le menu
	let dessine_menu = |selected: usize| {
		println!("--------------------------------------------------------");
		for (i, option) in language.options.iter().enumerate() {
			if i == selected {
				println!("===> {}", option);
			} else {
				println!("     {}", option);
			}
		}
		println!("--------------------------------------------------------");
		print!("{}: ", language.menu_usage);
		io::stdout().flush().unwrap();
	};
// ########################################################################

	livre.imp_comptes(&livre.COMPTES);
	// Passage en mode brut du terminal
	mode_brute_on();
	dessine_menu(selected);
	loop {
		let mut buffer = [0; 3];
		let n = io::stdin().read(&mut buffer).unwrap();

		match &buffer[..n] {
			[27, 91, 65] => { // Flèche haut
				if selected > 0 { selected -= 1; }
				reposition_cursor_and_clear(language.options.len() + 3);
				dessine_menu(selected);
			}
			[27, 91, 66] => { // Flèche bas
				if selected < language.options.len() - 1 { selected += 1; }
				reposition_cursor_and_clear(language.options.len() + 3);
				dessine_menu(selected);
			}
			[10] | [13] => { // Entrée
				mode_brute_off();
				io::stdout().flush().unwrap();
				if !passeur(selected, var_app, livre) {
					eprintln!("{}", language.err_majeure);
					break;
				}
				mode_brute_on();
				dessine_menu(selected);
			}
			[b'0'] => {
				// Rétablissement du mode normal du terminal
				mode_brute_off();
				println!("{}", language.aurevoir);
				break;
			}	// Quitter
			[b] if (b'1'..=b'9').contains(b) => {
				mode_brute_off();
				io::stdout().flush().unwrap();
				selected = (b - 49) as usize;
				if !passeur(selected, var_app, livre) {
					eprintln!("{}", language.err_majeure);
					break;
				}
				mode_brute_on();
				dessine_menu(selected);
			}
			[l] if (b'a'..=b'e').contains(l) => {
				mode_brute_off();
				io::stdout().flush().unwrap();
				selected = (l - 88) as usize;
				if !passeur(selected, var_app, livre) {
					eprintln!("{}", language.err_majeure);
					break;
				}
				mode_brute_on();
				dessine_menu(selected);
			}
			_ => {}
		}
	}
}
