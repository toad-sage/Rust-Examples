extern "C" {
    fn abs(input: i32) -> i32;
    fn pow(a: f64, b: f64) -> f64;
}

static mut COUNTER: u32 = 0; // made a global variable

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
        println!("Power {} raised to {} -> {}", 2, 3, pow(2.0, 3.0));
    }

    add_to_count(4);
    let d;
    unsafe {
        d = COUNTER;
        println!("COUNTER: {}", COUNTER);
    }
    println!("{}", d);
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn _first_demo() {
    // a slice cannot be borrowed immutable more than once in scope,
    // even the slice are on different range

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    // this contains unsafe code, under safe abstraction
    // open split_at_mut function to see its implementation
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

use std::slice;

//
fn _splitter(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr(); // returns a rae pointer

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
