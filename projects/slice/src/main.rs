fn first_word(s: &str) -> &str {
    //&str is a slice view of a string
    //a &str can accept &String
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}


fn main() {
    let s = String::from("Hello, world!");

    let word = first_word(&s);

    println!("first word is {}", word);

}
