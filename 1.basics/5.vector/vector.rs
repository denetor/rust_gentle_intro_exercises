// i vector sono simili agli array, ma possono avere dimensione variabile.
// sono più simili agli slice, in realtà
// vanno sempre dichiarati mutabili

fn main() {
    // creazione normale di un vettore
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0];  // will panic if out-of-range
    let maybe_first = v.get(0);

    println!("v is {:?}", v);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);
    // v is [10, 20, 30]
    // first is 10
    // maybe_first is Some(10)

    // posso creare e inizializzare un vettore anche con la macro vec!
    let mut v1 = vec![10, 20, 30, 40];
    // con .pop() rimuovo l'ultimo elemento
    v1.pop();

    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);

    // i due Vec sono uguali
    assert_eq!(v1, v2);

    // esistono molti altri metodi dei vector .clone(), .insert(), .clear(), ...
    v2.extend(0..2);
    assert_eq!(v2, &[10, 20, 30, 0, 1]);
}
