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


use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    fileIO();
    Ok(())
}
