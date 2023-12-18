/*
 * FileName:        03_01
 * Author:          8ucchiman
 * CreatedDate:     2023-09-22 18:23:47
 * LastModified:    2023-01-23 14:15:07 +0900
 * Reference:       8ucchiman.jp
 * Description:     ---
 */



#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}


use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    println!("hoge");
    Ok(())
}
