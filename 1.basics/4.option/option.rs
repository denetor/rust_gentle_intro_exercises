fn main() {
    let arr = [33,21,43,17,11];
    let slice = &arr[1..];

    // certi metodi, che potrebbero fallire, ritornano un Option
    let first = slice.get(0);
    let non_existant = slice.get(99);
    println!("first slice element {:?}", first);
    println!("non-existing slice element {:?}", non_existant);

    // per vedere se un elemento esiste o meno usiamo .is_some() e .is_none(); il valore li legge con .unwrap()
    println!("first {} {}", first.is_some(), first.is_none());
    println!("non-existing {} {}", non_existant.is_some(), non_existant.is_none());
    println!("first value {}", first.unwrap());

    // se non so se trovo un valore posso controllare is_some()
    let maybe_last = slice.get(99);
    let last = if maybe_last.is_some() {
        // dereferencing the &i32 value
        *maybe_last.unwrap()
    } else {
        -1
    };
    println!("last value {}", last);

    // oppure, più concisamente:
    let last_shorter = *slice.get(99).unwrap_or(&-1);
    println!("last value {}", last_shorter);
}