/// - fn definiert Funktion
/// - main ist Einstiegspunkt
fn main() {
    /// println! gibt Zeile aus
    /// ! bedeutet Macro (erstmal: vergleichbar mit Funktion)
    /// ; „beendet Anweisungen“
    println!("Hello World!");

    /// Primitive Typen:
    ///
    /// Ganzzahlen:
    ///  - i8, i16, i32, i64 (Feste Größe, mit Vorzeichen)
    ///  - u8, u16, u32, u64 (Feste Größe, nur positiv)
    ///  - isize, usize (pointer sized)
    ///
    /// Fließkomma:
    ///  - f32 (float), f64 (double)
    ///
    /// Sonstige
    ///  - bool (true, false)
    ///  - char (Unicode Skalar, 32bit)
    ///  - str (String slice)

    /// - Variablenbindings über den let Befehl
    /// - Compiler bestimmt den Typ automatisch
    /// - Variablen sind immutable by default (nicht veränderbar)
    /// - Variablennamen snake_case
    let a = 3;
    let b = 3.14;
    let c = true;

    a = 4; // error: re-assignment of immutable variable `a`

    // Mit mut Keyword als mutable deklarieren
    let mut x = 3;
    x = 4; // ok

    /// Typumwandlung/Casten mit Keyword 'as'
    let x = 3i32;
    let y = x as u16;


    /// Tupel: Heterogene, endliche Sequenz
    /// (T, U, ...)
    ///  - Länge/Arität fest zur Kompilierzeit!
    ///  Beispiele:
    ///  - (u8, bool)
    ///  - (u64, char, i8)
    ///  - (T,) // <- Tupel mit einem Element
    ///  - ()   // <- Void
    /// Zugriff mit .0, .1, usw. (oder destructure!)

 
    /// Arrays und Slides: Homogene Sequenz
    /// Länge N fest zur Kompilierzeit!
    /// Beispiele:
    ///  - [bool; 3]
    ///  - [u32; 8]
    ///  - [T; 1]  // <- Array mit einem Element
    /// Zugriff mit [0], [1], usw.
    /// [T] -> Slice: „View“ in Speicherblock, z.B. Array (später mehr)

    // Explicit type annotations with `: T` (rarely necessary!)
    let a: bool = true;
    let b: char = '水'; // Unicode :)

    let c: i32 = 3;    // also possible: u32, i8, usize, ...; often determined later
    let d: f64 = 3.14; // also possible: f32; often determined later
                       // ^^^ called {integer} and {float} in error messages

    let t: (char, bool) = ('♥', true);
    let (x, y): (char, bool) = ('♥', true); // destructuring ...
    let (u, v) = t; // this works, too
    t.0 == x;       // accessing tuple elements, both sides are the same
    t.1 == y;

    /// Ausgabe
    /// - Makro-Familie: println!(), print!(), format!(), ...
    /// - Erstes Argument: Formatstring mit Platzhaltern
    /// - Danach: Werte für Platzhalter
    println!("Kein Platzhalter hier...");

    let a = 3;
    println!("Der Wert von a ist {}", a);

    let b = true;
    println!("a ist {} und b ist {}", a, b);

    let arr = [3, 1, 4];
    println!("arr ist {}", arr);


    // Fixed size arrays, size in type (-> size fixed at compile time)
    let a: [i32; 3] = [2, 4, 6];
    let b: [char; 2] = ['☂', '♞'];

    // we can call methods on arrays, and index with []
    println!("{} and {}", a.len(), b.len()); // output: „3 and 2“
    println!("{} and {}", a[0], b[1]);       // output: „2 and ♞“

    let c: [char; 5] = ['a', 'b', 'c', 'd', 'e'];


    // Slices: size not in type, but a runtime value
    let d: &[char] = &c[1..4]; // ['b', 'c', 'd']
    let e: &[char] = &c[3..];  // ['d', 'e']
    let f: &[char] = &c[..];   // ['a', 'b', 'c', 'd', 'e']
    println!("{}; {}; {}", d.len(), e[0], f[4]);  // output: "3; d; e"

    // Methoden auch auf primitiven Typen aufrufbar (später mehr) :
    16.is_power_of_two(); // true

    /// Benannte, nutzerdefinierte Typen
    ///  - Struct
    ///  - Tuple-Struct
    ///  - Enum
    struct Point {
        x: f32,
        y: f32,
    }

    let point = Point {
        x: 5.0,
        y: 3.14,
    };
    println!("x={}, y={}", point.x, point.y);

    /// If-Else-Statement
    /// - Bedingung ohne runde Klammern
    /// - Rumpf zwingend mit geschweiften Klammern!
    ///
    /// Allgemein:
    /// - Öffnende, geschweifte Klammer sollte nicht in eigene Zeile
    /// - Schließende, geschweifte Klammer immer in eigene Zeile (außer vor else)
    if a == 4 {
        println!("If branch");
    } else if a > 10 {
        println!("Else-If branch");
    } else {
        println!("Else branch");
    }

    /// If-Else-Expression
    ///  - Nur möglich wenn Else-Zweig vorhanden
    ///  - Alle Zweige müssen den selben Typen zurückgeben
    ///  - Vorsicht mit den Semikola! Semikolon wandelt Expression in Statement:
    let a = 5;
    let b = if a >= 50 { 100 } else { 0 };
    // type of c?
    let c = if b % 2 == 0 {
        // do some work
        'w'
    } else {
        // do some other work
        's'
    };


    /// break; und continue; funktionieren wie gewohnt in allen Schleifentypen
    while a < 10 {
        a += 1;
    }

    /// For-Schleife
    for i in 1..10 {
        println!("{}", i);
    }

    let arr = [3, 27, 42];
    for elem in &arr {
        println!("{}", elem);
    }

    for adult_age in 18.. {
        // wheeeeee
    }

    loop {
        // equivalent to `while true { }`
        println!("yolo!");
    }

    /// Aufgabe 1.2: Collatz
    /// Schreibt ein Programm, das für die Zahl 27 alle Schritte
    /// des Collatz-Algorithmus ausgibt.
    /// Die Ausgabe soll in der Form x -> y geschehen,
    /// wobei x ein Zähler ist, welcher den aktuellen Schritt anzeigt,
    /// und y für den aktuellen Wert der Eingabezahl steht.
    let mut number = 27;
    let mut count = 0;

    while number != 1 {
        number = if number % 2 == 0 {
            number / 2
        } else {
            number * 3 + 1
        };

        println!("{} -> {}", count, number);
        
        count += 1;
    }
    
    /// Funktionen (siehe unten)
    foo();
    print_number(20);
    print_sum(20, 22);


    println!("3*3 = {}", square(3));
    let (double, triple) = double_triple(7);
}


/// Funktionen
/// - Name in snake_case
/// - Erst Parametername, dann –typ
/// - Freie Funktionen (kein Empfängerobjekt, wie z.B. this)
/// - Definition in anderen Funktionen möglich!
/// - Typinferenz zaubert wieder!
/// - Überladung gibt es nicht!
fn foo() {
   // does nothing
}

fn print_number(n: i64) {
    println!("A number: {}", n);
}

fn print_sum(a: i32, b: i32) {
    println!("A sum: {}", a + b);
}

fn square(n: i32) -> i32 { // returns i32
    n * n // no "return" keyword?!
}

fn is_prime(n: u64) -> bool {
    if x <= 1 {
        /// early return
        return false;
    }
    // lots of code calculating `prime: bool`
    prime
}

fn double_triple(n: i32) -> (i32, i32) { // returns tuple
    (2 * n, 3 * n)
}

// note: there is already n.abs()
// no need to write it yourself
fn absolute_value(n: i32) -> i32 {
    if n < 0 { -n } else { n }
}


