use std::io;

fn main() {
     let mut r = 3;
     let l:i32 = loop {
         let r + 1 ;
         // Einen neuen String anlegen, in den die Eingabe gespeichert wird
         let mut x = String::new();
         println!("Bitte gib etwas ein:");

         // Eingabe aus stdin einlesen
         io::stdin()
             .read_line(&mut x)
             .expect("Fehler beim Einlesen");

         // Das Newline-Zeichen am Ende entfernen
         let x = x.trim();
         println!("Du hast eingegeben: {}", x);
         if r == 3{
             println!("shakira");
             break;
         }

     };
}
