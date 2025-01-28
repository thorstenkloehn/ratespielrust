use rand::Rng;
use std::io;
fn main() {

    loop {
    let mut rng = rand::rng();
    let zufallszahl: i32 = rng.random_range(1..101); // Verwendung der neuen Methode `random_range`
       
    for _ in 0..10 {
        println!("Geben Sie eine Zahl ein:");
        
        let mut eingabe = String::new();
        io::stdin().read_line(&mut eingabe).expect("Fehler beim Lesen der Eingabe");
        let ergebnis:i32 = eingabe.trim().parse().expect("Ist keine zahl");
        if ergebnis == zufallszahl {
            println!("Herzlich Glückwunsch Sie haben Gewonnen");
            break;
        } else if ergebnis < zufallszahl {
            println!("Zu klein");
        } else {
            println!("Zu Gross")
        }
    }
 
 println!("Wollen Sie Weiter Spielen");
 println!("Geben Sie j[a] oder [n]ein]  ein");
 break;

    }
} 


