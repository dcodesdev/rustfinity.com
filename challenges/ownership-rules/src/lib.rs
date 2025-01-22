pub fn calculate_and_modify() -> (String, usize) {
    let mut s = String::from("hello");
    let length = s.len();

    s.push_str(", world");

    let s2 = &s;
    println!("{}", s2);

    (s, length)
}
