use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Devinez le nombre !");

    let nombre_secret: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Veuillez entrer un nombre compris entre 0 et 100.");
        let mut supposition: String = String::new();

        io::stdin()
            .read_line(&mut supposition)
            .expect("Échec de la lecture de l'entrée utilisateur");

        // let suppostion et le supposition.trim ne sont pas les mêmes ! Attention
        let supposition: u32 = supposition
            .trim()
            .parse()
            .expect("Veuillez entrer un nombre !");

        //  ici, on affiche le supposition: u32
        println!("Votre nombre : {}", supposition);

        // ici, on match le supposition: u32
        match supposition.cmp(&nombre_secret) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break;
            }
        }
    }
}
