use std::{f32::consts::E, io};
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Indovina il numero!\n");
    let numero_segreto = rand::thread_rng().gen_range(1..=100);
    println!("Perfavore introduci il tuo numero!\n");
    let mut indovina = String::new();
    io::stdin()
        .read_line(&mut indovina)
        .expect("Errore nella lettura");
    let indovina: u32 = indovina.trim().parse().expect("Errore: Perfavore inserisci un numero!");
    println!("Il numero generato è: {numero_segreto}\n");
    println!("Il tuo numero è: {}\n",indovina);
    match indovina.cmp(&numero_segreto) {
        Ordering::Less => println!("il numero è troppo piccolo"),
        Ordering::Greater => println!("il numero è troppo grande"),
        Ordering::Equal => println!("il numero è esatto "),
        
    }
}
