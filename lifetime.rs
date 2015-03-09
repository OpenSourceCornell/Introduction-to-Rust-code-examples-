fn main() {
    let ptr = {
        let s = "Hello world!".to_string();  // lifetime of s starts here
        println!("{}", s);
        &s
        // lifetime of s ends here
    };
    println!("{}", *ptr);  // &s is used here
}
