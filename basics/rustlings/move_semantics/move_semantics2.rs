/*
 * FileName:        move_semantics2
 * Author:          8ucchiman
 * CreatedDate:     2023-06-20 23:34:32
 * LastModified:    2023-01-23 14:15:07 +0900
 * Reference:       https://lazyren.github.io/studylog/rustlings-move-semantics#move_semantics2rs
 * Description:     ---
 */

/*
So, `vec0` is passed into the `fill_vec` function as an argument. In Rust,
when an argument is passed to a function and it's not explicitly returned,
you can't use the original variable anymore. We call this "moving" a variable.
Variables that are moved into a function (or block scope) and aren't explicitly
returned get "dropped" at the end of that function. This is also what happens here.
There's a few ways to fix this, try them all if you want:
1. Make another, separate version of the data that's in `vec0` and pass that
   to `fill_vec` instead.
2. Make `fill_vec` borrow its argument instead of taking ownership of it,
   and then copy the data within the function in order to return an owned
   `Vec<i32>`
3. Make `fill_vec` *mutably* borrow a reference to its argument (which will need to be
   mutable), modify it directly, then not return anything. Then you can get rid
   of `vec1` entirely -- note that this will change what gets printed by the
   first `println!`
*/


// 1. make anotgher, separate version of the data that's in vec0 and pass that to fill_vec instead.
fn main() {
    let vec0 = Vec::<i32>::new();
    let mut vec1 = fill_vec(vec0.clone());

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}


// 2. Make `fill_vec` borrow its argument instead of taking ownership of it, and then copy the data within the function in order to return an owned `Vec<i32>`
fn main() {
    let vec0 = Vec::<i32>::new();
    let mut vec1 = fill_vec_with_reference(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec_with_reference(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}


// 3. Make `fill_vec` *mutably* borrow a reference to its argument (which will need to be
//   mutable), modify it directly, then not return anything. Then you can get rid
//   of `vec1` entirely -- note that this will change what gets printed by the
//   first `println!`
fn main() {
    let mut vec0: Vec<i32> = Vec::new();
    let mut vec1 = fill_vec_with_mutable_reference(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec_with_mutable_reference(vec: &mut Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.clone()
}
