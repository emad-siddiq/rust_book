fn main() {
   let user1 = User {
        active: true, 
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
   println!("User, {}, is active: {}", user1.username, user1.active);
}

struct User {
    active: bool,
    username: String, 
    email: String,
    sign_in_count: u64,
}
