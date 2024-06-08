fn main() {
    // match può coperare con un Option()
    match std::env::args().find(|x| x == "ls") {
        Some(_idx) => {
            println!("could find the 'ls' command");
        },
        None => println!("couldn't find the 'ls' command")
    }

    // ma può anche essere usato cone uno switch
    let text = match 1 {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "fallback",
    };
    println!("text is {}", text);

    // match riconosce anche i ranger (che hanno 3 punti)
    let voto = match 6 {
        0..=5 => "insufficiente",
        6..=7 => "sufficiente",
        8..=10 => "ottimo",
        _ => "non valido",
    };
    println!("voto: {}", voto);
    
}