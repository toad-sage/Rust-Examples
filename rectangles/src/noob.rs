fn calc() {
    let rect: (u32, u32) = (30, 50);

    println!("The area of the rectangle is {} square pixels.", area(rect));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
