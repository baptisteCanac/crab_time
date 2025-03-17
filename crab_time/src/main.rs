use std::io;

fn ask_entry(question: &str) -> String{
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
    /*
    Ce programme analyse une commande
    Il demande en entrée la commande à tester et le nombre de tests que l'on doit faire
    Elle renvoit le temps que la commande met pour s'executer
    */

    /*
    Demande la commande à analyser et le nombre de tests
    Le nombre de tests est codé sur 32 bits pour permettre un énorme nombre de tests
    */
    let command:String = ask_entry("Entrez la commande à analyser: ");
    let nombre_tests:i32 = ask_int("Combien de lancement faire (Plus le nombre de test sera élevé, plus les vérifications seront précises): ");
    
    println!("Commande: {}", command);
    println!("Nombre de tests: {}", nombre_tests);
}