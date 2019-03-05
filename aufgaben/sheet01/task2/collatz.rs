/// Aufgabe 1.2: Collatz
/// Schreibt ein Programm, das für die Zahl 27 alle Schritte
/// des Collatz-Algorithmus ausgibt.
/// Die Ausgabe soll in der Form x -> y geschehen,
/// wobei x ein Zähler ist, welcher den aktuellen Schritt anzeigt,
/// und y für den aktuellen Wert der Eingabezahl steht.
fn main() {
    let mut number = 27;
    let mut count = 0;

    while number != 1 {
        // Foliensatz 1, Seite 16 :)
        number = if number % 2 == 0 {
            number / 2
        } else {
            number * 3 + 1
        };

        println!("{} -> {}", count, number);
        
        count += 1;
    }
}

