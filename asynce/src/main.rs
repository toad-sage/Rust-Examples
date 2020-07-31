use futures::executor::block_on;

async fn hello_world() {
    println!("hello,world");
    std::thread::sleep(std::time::Duration::from_secs(2));
}

async fn waiter() {
    println!("Second One");
    println!("Second One");
    println!("Second One");
}

async fn async_main() {
    let f1 = hello_world();
    let f2 = waiter();

    futures::join!(f1, f2);
}

fn main() {
    // block_on blocks current thread, so using .await will not block current thread
    // let future = hello_world();
    // block_on(future);
    // println!("Second One");

    // await can only be called from async function
    block_on(async_main());
}
