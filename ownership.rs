fn main() {
    let mut v = vec![];  // v owns the vector
    v.push("Hello");
    let x = &v[0];  // x borrows the vector
    v.push("world");  // v tries to modify the vector
    println!("{}", x);
}
