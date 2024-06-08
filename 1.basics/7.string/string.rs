// il borrow operastor & coercizza la String in &str
fn dump(s: &str) {
    println!("str '{}'", s);
}

fn array_to_str(arr: &[i32]) -> String {
    let mut res = '['.to_string();
    for v in arr {
        res += &v.to_string();
        res.push(',');
    }
    res.pop();
    res.push(']');
    res
}

fn main() {
    let string_slice = "hello dolly";  // non è una stringa ma un string slice (&str)
    let s = string_slice.to_string();  // it's now an allocated string

    dump(string_slice);
    dump(&s);

    // Under the hood, String is basically a Vec<u8> e quindi si applicano i metodi dei vector
    let mut hello = String::new();
    hello.push('H');
    hello.push_str("ello");
    hello.push(' ');
    hello += "world!";
    hello.pop();
    println!("{}", &hello);

    // con .to_string() e .format() è possibile trasformare in stringa e formattare molti tipi di dati
    let arr = array_to_str(&[10, 20, 30]);
    let res = format!("hello {}", arr);
    println!("{}", &res);

    // posso anche fare lo slicing come sugli slice
    println!("{}", &"Hello!".to_string()[1..5]);
    // ma occhio che i caratteri unicode contano, a volte, più di 1, mai fidarsi
    println!("len of '{}' is {} bytes", "Hi! ¡Hola! привет!", "Hi! ¡Hola! привет!".to_string().len());
}