fn main() {
    // la funzione args() ritorna un itarable con gli argomenti della riga di comando
    for arg in std::env::args() {
        println!("{}", arg);
    }

    // expect() è come un unwrap, con un messaggio se fallisce
    let _command = std::env::args().nth(1).expect("please supply a command");
    let value = std::env::args().nth(2).expect("please supply a value after the command");
    let _n: i32 = value.parse().expect("value is not an integer!");
}