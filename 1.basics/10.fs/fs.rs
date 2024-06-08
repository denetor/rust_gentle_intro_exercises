use std::env;
use std::io;
use std::fs::File;
use std::io::Read;

fn read_to_string(filename: &str) -> Result<String,io::Error> {
    let mut file = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(e) => Err(e),
    }
}

fn read_to_string_shorter(filename: &str) -> io::Result<String> {
    // il ? alla fine fa ritornare il risultato Ok() e ritorna l'errore, se si presenta
    let mut file = File::open(&filename)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

fn main() {
    let file_name = env::args().nth(1).expect("please supply a filename");

    // versione grezza
    let mut file = File::open(&file_name).expect("can't open the file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("can't read the file");
    println!("file length: {} bytes", file_content.len());

    // versione con una che gestisce meglio gli errori
    let file_content_2 = read_to_string(&file_name).expect("bad file man!");
    println!("file length 2: {} bytes", file_content_2.len());
}