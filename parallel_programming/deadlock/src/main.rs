/*
 * FileName:        main
 * Author:          8ucchiman
 * CreatedDate:     2023-09-21 14:42:31
 * LastModified:    2023-01-23 14:15:07 +0900
 * Reference:       8ucchiman.jp
 * Description:     ---
 */


use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;


fn main() -> Result<(), Box<dyn Error>> {
    let c0 = Arc::new(Mutex::new(()));
    let c1 = Arc::new(Mutex::new(()));

    let c0_p0 = c0.clone();
    let c1_p0 = c1.clone();

    // philosopher 1
    let p0 = thread::spawn(move || {
        for _ in 0..100000 {
            let _n1 = c0_p0.lock().unwrap();
            let _n2 = c1_p0.lock().unwrap();
            println!("0: eating");
        }
    });

    // philosopher 2
    let p1 = thread::spawn(move || {
        for _ in 0..100000 {
            let _n1 = c1.lock().unwrap();
            let _n2 = c0.lock().unwrap();
            println!("1: eating");
        }
    });

    p0.join().unwrap();
    p1.join().unwrap();

    Ok(())
}
