

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);

    // slice
    let hello = &s[0..5];
    println!("{}", hello);
    let world = &s[6..11];
    println!("{}", world);

    let hello = first_word2(&s);
    println!("{}", hello);

    let w2 = first_word2(&s);
    println!("w2 {}", w2);
    
    s.clear();

}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
