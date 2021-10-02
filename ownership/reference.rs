fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    mod_string(&mut s2);
    println!("Modified string is {}", s2);
}

// borrowing - can't modify s in function body, don't take ownership
fn calculate_length(s: &String) -> usize {
    s.len()
}

// but we can declare a reference mutable to modify
// can only have one mutable reference to one piece of data in a scope
fn mod_string(s: &mut String) {
   s.push_str(", world");
}