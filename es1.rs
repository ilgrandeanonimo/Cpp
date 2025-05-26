/*
Un impianto di risalita ha tariffa base di 7€; in estate (e) viene applicata
una maggiorazione del 15%, in autunno (a) uno sconto del 50%; in inverno (i)
viene raddoppiata, in primavera (p) il costo rimane pari alla tariffa base.
Data in ingresso la stagione, calcolare e visualizzare il costo dell'impianto
*/
use std::io::{self, Write};

fn main() {
    print!("Inserisci il prezzo: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut prezzo = 7f32;
    match input.trim().to_lowercase().as_str() {
        "primavera" => {
            println!("In primavera il prezzo è rimane la tariffa base.");
        }
        "estate" => {
            prezzo = prezzo * 115.0 / 100.0;
            println!("In estate c'è un incremento del 15%.");
        }
        "autunno" => {
            prezzo /= 2.0;
            println!("In autunno c'è uno sconto del 50%.");
        }
        "inverno" => {
            prezzo *= 2.0;
            println!("In inverno il prezzo raddoppia.");
        }
        _ => println!("Quello inserito non è il nome di una stagione"),
    }
    println!("Prezzo: {prezzo}€")
}
