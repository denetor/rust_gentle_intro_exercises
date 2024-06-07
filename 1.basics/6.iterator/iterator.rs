fn main() {
    // iteratore basico (notare il mut) (disabilito anche gli warning x variabile non usata)
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    let mut iter = 0..3;

    // uso .iter() per iterare in un array
    let arr = [10, 20, 30];
    for i in arr.iter() {
        println!("{}", i);
    }

    // slices will be converted implicitly to iterators...
    let slice = &arr;
    for i in slice {
        println!("{}", i);
    }

    // gli iteatori hanno dei metodi, ad esempio .sum()
    let sum: i32  = (0..5).sum();
    println!("iterator sum is {}", sum);

    // gli array possono essere trasformati in iteratori, poi li posso sommare
    // nota che il tipo è necessario, rustc non può inferirlo
    let sum: i64 = [10, 20, 30].iter().sum();
    println!("array sum is {}", sum);
}