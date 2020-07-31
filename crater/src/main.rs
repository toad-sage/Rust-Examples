// use crater::kinds::PrimaryColor;
// use crater::utils::mix;
use crater::mix;
use crater::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    println!("{:?}", mix(red, yellow));
}
