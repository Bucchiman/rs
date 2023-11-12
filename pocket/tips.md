<!--
 FileName:      tips
 Author:        8ucchiman
 CreatedDate:   2023-11-12 15:01:49
 LastModified:  2023-01-25 10:56:12 +0900
 Reference:     8ucchiman.jp Description:   ---
-->

<!-- ------------------------------ -->
# length of array
```rs
fn main () {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let length = numbers.len();
    println!("{}", length);
}

```
- [Reference](https://www.choge-blog.com/programming/rustarraygetlength/)
<!-- ------------------------------ -->
# io::stdin().read_line(&mut guess).expect("Failed to read line");

```rs
    use std::io;
    fn main() {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    }
```
- [reference](https://doc.rust-jp.rs/book-ja-pdf/book.pdf)

<!-- ------------------------------ -->
# rand::thread_rng().gen_range(1..101);


```rs
use rand::Rng;

fn main () {
    let secret_number = rand::thread_rng().gen_range(0..101);
    print!("{}", secret_number);
}
```

- 2023-11-12 18時10分20秒
- [reference](https://doc.rust-jp.rs/book-ja-pdf/book.pdf)
<!-- ------------------------------ -->
# compare number
```rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main () {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    println!("The secret number is: {}", secret_number);
    
    println!("Please input your guess.");
    
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

```

- 2023-11-12 18時28分45秒
- [reference](https://doc.rust-jp.rs/book-ja-pdf/book.pdf)

<!-- ------------------------------ -->
# string2integer

```rs
    use std::io;

    fn main () {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
                    .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }
    }
```
- 2023-11-12 18時28分33秒
- [reference](https://doc.rust-jp.rs/book-ja-pdf/book.pdf)

<!-- ------------------------------ -->
# シャドーイング
- 前に定義した変数と同じ名前の変数を新しく宣言でき、新しい変数は、前の変数を覆い隠す

```rs
fn main () {
    let x = 4;
    let x = x + 1;

    let spaces = "     ";
    let spaces = spaces.len();    // ok!!

    let mut spaces = "    ";
    spaces = spaces.len();        // ng!!
}
```

<!-- ------------------------------ -->
# data type
- rust: 静的型付き言語

- [reference](https://doc.rust-jp.rs/book-ja-pdf/book.pdf)
<!-- ------------------------------ -->
# move
```rs
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, worl!", s1);      // error
```

```rs
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}", s1);
```

- [reference p74](https://doc.rust-jp.rs/book-ja-pdf/book.pdf)
