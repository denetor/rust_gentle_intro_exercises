use std::f64::consts;

fn double(n: i32) -> i32 {
    n * 2
}

// passo per riferimento mutabile il parametro
fn double_ref(n: &mut f64) -> f64 {
    // referenzo il parametro n
    return *n * 2.0;
}


fn main() {
    let otto = double(4);
    println!("Double of 4 is {}", otto);

    let dieci: f64 = double_ref(&mut 5.0);
    println!("Double of 5 (by ref) is {}", dieci);

    let mut sei = 6.0;
    let dodici = double_ref(&mut sei);
    println!("Double of 6 (by ref) is {}", dodici);

    let r = 10.0;
    // avrei potuto non importare std::f64::consts e usare direttamente std::f64::consts::PI
    let circle_area = consts::PI * r * r;
    println!("Area of circle having radius of {} is {}", r, circle_area);
}