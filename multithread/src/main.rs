/*
 * FileName:        main
 * Author:          8ucchiman
 * CreatedDate:     2023-05-23 13:50:54
 * LastModified:    2023-01-23 14:15:07 +0900
 * Reference:       8ucchiman.jp
 * Description:     ---
 */


use std::thread;
use std::time::Duration;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    Ok(())
}

