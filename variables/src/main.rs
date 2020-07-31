struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername@123"),
        active: true,
        sign_in_count: 1,
    };
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!(
        "{}{}{}{}",
        user1.email, user1.sign_in_count, user1.active, user1.username
    );
    let user2: User = build_user("myemail.com", "heremail.com");
    let origin = Point(0, 0, 0);
    let Point(x, y, z) = origin;
    println!("{} {} {}", x, y, z);
}

fn build_user(email: &str, username: &str) -> User {
    User {
        email: String::from(email),
        username: String::from(username),
        active: true,
        sign_in_count: 1,
    }
}
