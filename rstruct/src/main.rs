struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

fn main() {
    let mut user = User {
        username: String::from("someone"),
        email: String::from("someone@gmail.com"),
        sign_in_count: 0,
        active: false
    };

    user.email = String::from("changeme@gmail.com");

    let pu = &mut user;
    pu.active = true;

    let user2 = build_user(String::from("someone"), String::from("someone@gmail.com"));

    let user3 = User {
        username: String::from("someone3"),
        email: String::from("someone3@gmail.com"),
        ..user
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Color {}, {}, {}", black.0, black.1, black.2);
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 0,
        active: false
    }
}
