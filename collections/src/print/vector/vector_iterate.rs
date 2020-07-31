use super::vector_print::print1;

pub fn iterate(v: &Vec<i32>) {
    for i in 0..v.len() {
        print1(&v[i]);
    }
    // or
    for i in v {
        print1(i);
    }
}

pub fn mutable_iterate(v: &mut Vec<i32>) {
    for i in 0..v.len() {
        v[i] += 50;
    }
    iterate(v);
}
