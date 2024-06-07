

// values non è un array, ma è uno slice, grande come l'array, e va passato per riferimento
// & non si legge "puntatore a", ma "borrow"
fn sum(values: &[i32]) -> i32 {
    let mut total = 0;
    for i in 0..values.len() {
        total += values[i];
    }
    return total;
}


fn main() {
    // array di 5 elementi
    let arr = [33,21,43,17,11];
    println!("Array: {:?}", arr);
    println!("Somma di {} elementi: {}", arr.len(), sum(&arr));

    // estrae due slice dell'array
    let slice1 = &arr[2..];
    let slice2 = &arr[1..3];
    println!("slice1 [2..]: {:?}", slice1);
    println!("slice2 [1..3]: {:?}", slice2);
}