pub fn calculate_and_modify() -> (String, usize) {
    let mut s = String::from("hello");
    let length = s.len();

    let s2 = &s;
    s.push_str(", world");

    println!("{}", s2);

    (s, length)
}
