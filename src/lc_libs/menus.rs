use std::io::{self, Read, Write};

use crate::{lc_libs::LivreComptable, parse::VarsApp};
use crate::lc_libs::menus_fonctions::passeur;

pub struct MenuStrings {
	pub menu_usage: &'static str,
	pub options: &'static [&'static str],
}

pub const OPTIONS_FR: [&str; 14] = [
	"Entrer une nouvelle transaction",
	"Afficher tous les comptes",
	"Ajouter un compte",
	"Supprimer un compte",
	"Afficher toutes les catégories",
	"Ajouter une catégorie",
	"Supprimer une catégorie",
	"Afficher toutes les favorites",
	"Ajouter une transaction favorite",
	"Modifier une transaction favorite",
	"Supprimer une transaction favorite",
	"Interroger les transactions",
	"Accès complet à la bd [PRO ONLY]",
	"Sommaire de comptes",
];

pub const OPTIONS_ES: [&str; 14] = [
	"Ingresar una nueva transacción",
	"Mostrar todas las cuentas",
	"Agregar una cuenta",
	"Eliminar una cuenta",
	"Mostrar todas las categorías",
	"Agregar una categoría",
	"Eliminar una categoría",
	"Mostrar todos los favoritos",
	"Agregar una transacción favorita",
	"Modificar una transacción favorita",
	"Eliminar una transacción favorita",
	"Consultar transacciones",
	"Acceso completo a la base de datos [SOLO PRO]",
	"Resumen de cuentas",
];

pub const OPTIONS_EN: [&str; 14] = [
	"Enter a new transaction",
	"Show all accounts",
	"Add an account",
	"Delete an account",
	"Show all categories",
	"Add a category",
	"Delete a category",
	"Show all favorites",
	"Add a favorite transaction",
	"Edit a favorite transaction",
	"Delete a favorite transaction",
	"Query transactions",
	"Full DB access [PRO ONLY]",
	"Account summary",
];

pub const LANG_FR: MenuStrings = MenuStrings {
	menu_usage: "Choisir un item avec les flèches haut/bas puis enter ⏎\n --- « 0 » pour quitter",
	options: &OPTIONS_FR,
};

pub const LANG_ES: MenuStrings = MenuStrings {
	menu_usage: "Seleccione un elemento con las flechas arriba/abajo y luego ingrese ⏎\n --- « 0 » para salir",
	options: &OPTIONS_ES,
};

pub const LANG_EN: MenuStrings = MenuStrings {
	menu_usage: "Select an item with the up/down arrows then enter ⏎\n --- « 0 » to exit",
	options: &OPTIONS_EN,
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

pub fn affiche_menu(var_app: &VarsApp, livre: &mut LivreComptable) {
	let mut selected = 0;

	let language = match var_app.loc_string.as_str() {
		"fr" => LANG_FR,
		"es" => LANG_ES,
		_ => LANG_EN,
	};
	// Passage en mode brut du terminal
	let _ = std::process::Command::new("stty").arg("-echo").arg("cbreak").status();

	// Closure pour afficher le menu
	let dessine_menu = |selected: usize| {
		println!("--------------------------------------------------------");
		for (i, option) in language.options.iter().enumerate() {
			if i == selected {
				println!("==> {:>2}- {}", i + 1, option);
			} else {
				println!("    {:>2}- {}", i + 1, option);
			}
		}
		println!("--------------------------------------------------------");
		print!("{}: ", language.menu_usage);
		io::stdout().flush().unwrap();

	};

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
				let _ = std::process::Command::new("stty").arg("echo").arg("-cbreak").status();
//				println!("\n→ Sélection : {}", language.options[selected]);
				io::stdout().flush().unwrap();
				if !passeur(selected, &var_app, livre) {
					eprintln!("Erreur majeur - Sortie du programme");
					break;
				}
				let _ = std::process::Command::new("stty").arg("-echo").arg("cbreak").status();
				dessine_menu(selected);
//				break;
			}
			[b'0'] => { break; }	// Quitter
			
			_ => {}	// On passe tout droit
		}
	}

	// Rétablissement du mode normal du terminal
	let _ = std::process::Command::new("stty").arg("echo").arg("-cbreak").status();
}
