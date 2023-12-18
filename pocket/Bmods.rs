/*
 * FileName:        Bmods
 * Author:          8ucchiman
 * CreatedDate:     2023-10-25 10:23:37
 * LastModified:    2023-01-23 14:15:07 +0900
 * Reference:       8ucchiman.jp
 * Description:     ---
 */


use std::env;
use std::fs::File;
use std::io::prelude::*;

fn fileIO () {
    let mut f = File::open("./poem.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    println!("With text:\n{}", contents);
}

//let mut f = File::open(filename).expect("file not found");



/// Input string to 
///
/// # Arguments
///
/// * `name` - A string slice that holds the name of the person
///
/// # Examples
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
/// use doc::Person;
/// let person = Person::new("name");
/// ```
/// Reference: https://doc.rust-jp.rs/book-ja-pdf/book.pdf
///
fn inputIO () {
    use std::io;
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
}

/// Returns a person with the name given them
///
/// # Arguments
///
/// *  - A string slice that holds the name of the person
///
/// # Examples
///
/// rustdoc
/// Reference: https://doc.rust-jp.rs/book-ja-pdf/book.pdf
///
fn guess_number () {
    use std::io;

    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read line");
}


//
// tips
// :: ... expect associative function
//
// e.g.
//      String::new()   "String" expects "new" function
//
//
//
//


use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // fileIO();
    Ok(())
}
