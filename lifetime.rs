fn main() {
    let ptr = {
        let s = "Hello world!".to_string();
        println!("{}", s);
        &s
    };
    println!("{}", ptr);
}
