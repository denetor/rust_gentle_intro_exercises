fn main() {
    for i in 1..5 {
        assert!(i > 0);
        if i % 2 == 0 {
            println!("Hello, even {}", i);
        } else {
            println!("Hello, odd {}", i);
        }
    }

    for i in 1..5 {
        let even_odd = if i % 2 == 0 { "even" } else { "odd" };
        println!("Hello, {} {}", even_odd, i);
    }
}