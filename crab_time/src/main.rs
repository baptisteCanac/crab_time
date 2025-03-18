use std::io;
use std::process::Command;
use std::time::{Instant, Duration};
use std::fs::OpenOptions;
use std::io::Write;

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

    let mut i: i32 = 0;
    while i < nombre_tests {
        // Démarre le chrono
        let start = Instant::now();

        // Recréer `Command` à chaque itération
        let _ = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .expect("Echec de la commande, veuillez vérifier votre input");

        // Arrête le chrono
        let duration = start.elapsed();

        i += 1;
        total_execution.push(duration);
    }

    // Affiche les durées d'exécution
    println!("{:?}", total_execution);

    let mut data_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("data.txt")
        .expect("cannot open file");
    
    for duration in &total_execution{
        let duration_str = format!("{:?}\n", duration);
        data_file
            .write_all(duration_str.as_bytes()) // Convertir en &[u8]
            .expect("write failed");
    }
}