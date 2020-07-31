pub mod vector_print;

pub mod vector_iterate;

pub mod enum_vec;

use super::vector::vector_iterate::{iterate, mutable_iterate};
use super::vector::vector_print::{print1, print2};

pub fn vector_operation() {
    let mut v: Vec<i32> = Vec::new();
    // let v = vec![1,2,3];
    v.push(5);
    v.push(6);

    print1(&v[1]);
    print2(&v, 0);
    println!("{:?}", v.get(100));

    let mut v = vec![1, 2, 3, 4, 5];
    v[0] = 2;
    let first = &v[0];
    print1(first);
    let second = &v[1];
    // v.push(7);  due to wnership rule v[1] is borrowed hence, cannot be mutated
    println!("{}", second);
    v.sort();
    println!("{:?}", v);
    v.insert(2, 3);
    println!("{:?}", v);
    iterate(&v);
    mutable_iterate(&mut v);
    iterate(&v);

    enum_vec::creak();
}
