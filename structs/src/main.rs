struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
    User { 
           email, // this shorthand works because parameter and struct field names 
           username, // are the same. longhand would be email:email or email:email_longer if named diff
           active: true,
           sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        active: user1.active, 
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // or the following is the same

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };
    
    // These are tuple structs. Even though they take the same types, they need to be correct
    // types. This is done to save verbosity and redundant code. They are useful for unique tuples.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    let subject = AlwaysEqual;
}
