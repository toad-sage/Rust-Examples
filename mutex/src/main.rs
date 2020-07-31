// use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 1..11 {
        let counter = Arc::clone(&counter);
        let handler = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlers.push(handler);
    }

    for handle in handlers {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// this will not work as well because Rc is not safe for
// sharing memory acreoss thread

// fn third() {
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 1..10 {
//         let counter = Rc::clone(&counter);
//         let handle = std::thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result = {}", *counter.lock().unwrap());
// }

//this will not be allowed since counter variable is moved

// fn second() {
//     let counter = Mutex::new(0);
//     let mut handles = vec![];

//     for _ in 1..10 {
//         let handle = std::thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result = {}", *counter.lock().unwrap());
// }

fn first() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:#?}", m);
}
