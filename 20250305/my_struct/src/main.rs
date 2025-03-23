fn main() {
    println!("==============================================");
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{team_name}: {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    println!("==============================================");
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name.clone(), field_value);
    for (key, value) in &map {
        println!("{key}: {value}");
    }
    println!("{field_name}");

    println!("==============================================");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{scores:?}");

    println!("==============================================");
    let text = "hello world wonderful world";
    println!("{}", text);
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");

}
