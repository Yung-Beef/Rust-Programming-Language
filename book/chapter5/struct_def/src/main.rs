struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        // active: user1.active,
        // username: user1.username,
        email: String::from("another@example.com"),
        // sign_in_count: user1.sign_in_count,
        ..user1 // says that the fields not explicitly defined should have the same values as they do in user1 - must come last
        // can't use user1 anymore because user1.username was MOVED since it doesn't have the Copy trait
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // no need to write username: username, since the name is the same
        email,
        sign_in_count: 1,
    }
}


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}


struct AlwaysEqual;

fn unit_struct() {
    let subject = AlwaysEqual;
}
