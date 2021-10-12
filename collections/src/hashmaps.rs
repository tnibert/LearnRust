use std::collections::HashMap;

// uses SipHash algorithm for security against DoS attacks
// can change algorithm

pub fn scores() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // or create using iterators and collect()
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // access values with get(), returns Option<&V>
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // can loop
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

pub fn updating() {
    // overwrite
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // insert if the entry doesn't exist
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // or_insert() returns mutable reference
    // can use to update based on value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}