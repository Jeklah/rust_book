struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
   let user1 = User {
       email: "someone@email.com",
       username: "user",
       active: true,
       sign_in_count: 1,
   }; 
}
