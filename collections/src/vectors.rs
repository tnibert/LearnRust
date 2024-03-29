pub fn vector_decls() {
    let v1: Vec<i32> = Vec::new();

    // macro to infer type
    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);
}

pub fn vector_reads() {
    let v = vec![1, 2, 3, 4, 5];

    // this will cause a panic if the element does not exist
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // get returns Option<&T>
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

pub fn vector_loop() {
    let v1 = vec![100, 32, 57];
    for i in &v1 {
        println!("{}", i);
    }

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        // dereference operator
        *i += 50;
    }
    for i in &v2 {
        println!("{}", i);
    }
}

// for multiple types, whoa
pub fn vectors_with_enums() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}