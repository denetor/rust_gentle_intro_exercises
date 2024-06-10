use std::fmt;

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String
}

impl Person {

    // costruttore, lo chiamo con Person::new()
    fn new(first: &str, name: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: name.to_string()
        }
    }

    // metodo, perchè ha argomento &self
    fn full_name(&self) -> String {
        format!("{} {}",self.first_name, self.last_name)
    }

    // metodo che modifica self
    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string();
    }

    // questo ritorna una tupla
    // siccome l'argomento non è (&self), ne prende possesso (ne fa una move) e poi l'istanza di Person non è più valida
    fn to_tuple(self) -> (String,String) {
        (self.first_name, self.last_name)
    }
}


fn main() {
    let mut p = Person::new("John","Smith");

    println!("{:?}", p);

    p.set_first_name("Jane");

    println!("{:?}", p);

    println!("{:?}", p.to_tuple());
    // p has now moved.

}
// Person { first_name: "John", last_name: "Smith" }
// Person { first_name: "Jane", last_name: "Smith" }
// ("Jane", "Smith")