use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::error::Error;

// with this return type we can use ? in main()
fn main() -> Result<(), Box<dyn Error>> {
    //panic!("Hello, world!");
    let fname = "hello.txt";

    // returns Result enum
    let f = File::open(fname);

    // unpack the result
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // if file not found, create it
            ErrorKind::NotFound => match File::create(fname) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            // otherwise, panic
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // unwrap will panic if fails, expect does the same, but allows message specification
    let f = File::open(fname).unwrap();
    let f = File::open(fname).expect(format!("Failed to open {}", fname).as_str());

    // return value
    Ok(())
}

// propogate error up
fn read_username_from_file1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// does same thing as above
// ? operator can only be used when return type of function is Result or Option
// or implementer of std::ops::Try
fn read_username_from_file2(fname: &str) -> Result<String, io::Error> {
    // error type returned must have from() from From trait
    let mut f = File::open(fname)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    // could also chain:
    // File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// finally, we can just do this to get the same as the above two
fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}