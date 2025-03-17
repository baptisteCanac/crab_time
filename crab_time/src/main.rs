use std::io;
use std::process::Command;
use std::time::{Instant, Duration};

fn ask_entry(question: &str) -> String {
    let mut input = String::new(); // variable qui stocke l'entrée utilisateur

    println!("{}", question);

    io::stdin().read_line(&mut input)
        .expect("Echec de la lecture de l'entrée");

    input.trim().to_string() // retourne en String
}

fn ask_int(question: &str) -> i32 {
    loop {
        let input = ask_entry(question);
        match input.parse::<i32>() {
            Ok(num) => return num, // Retourne l'entier si la conversion réussit
            Err(_) => println!("Veuillez entrer un nombre entier valide."),
        }
    }
}

fn main() {
    // Demande la commande à analyser et le nombre de tests
    let command: String = ask_entry("Entrez la commande à analyser: ");
    let nombre_tests: i32 = ask_int("Combien de lancement faire (Plus le nombre de test sera élevé, plus les vérifications seront précises): ");

    let mut total_execution: Vec<Duration> = Vec::new(); // Créer tableau de taille variable

    // Crée l'objet Command avant la boucle
    let mut command_exec = Command::new("sh");

    // Ajoute les arguments à l'objet Command
    command_exec.arg("-c").arg(&command);

    let mut i: i32 = 0;
    while i < nombre_tests {
        // Démarre le chrono
        let start = Instant::now();

        // Exécute la commande
        let _ = command_exec.output().expect("Echec de la commande veuillez vérifier votre input");

        // Arrête le chrono
        let duration = start.elapsed();

        // Incrémente le nombre de tests
        i += 1;

        // Ajoute la durée d'exécution dans le tableau après 50 tests
        if i > 50 {
            total_execution.push(duration);
        }
    }

    // Affiche les durées d'exécution
    println!("{:?}", total_execution);
}