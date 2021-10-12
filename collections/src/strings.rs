// & BORROWS a reference - &str
// String and str are both UTF-8 encoded

pub fn create() {
    let mut s = String::new();

    let data = "initial contents";

    // method available to all types implementing Display trait
    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");
}

pub fn update() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    // push appends single character
    s.push('!');
    println!("{}", s);

    // concatenation with + is possible, but first operand will give up ownership to add()
    // better to use format!()
    // macro uses references
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}

pub fn accessing_elements() {
    // rust strings don't support indexing by integer
    // bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters)

    let hello = "Здравствуйте";
    // first four bytes (2 two byte grapheme clusters for Russian)
    let s = &hello[0..4];
    println!("{}", s);
    // the following would cause runtime panic - risky
    // &hello[0..1]

    // iterate over scalar values
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // iterate over bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // Getting grapheme clusters from strings is complex, so this functionality is not provided by the standard library
}