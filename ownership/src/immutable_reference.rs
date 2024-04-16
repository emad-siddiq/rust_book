// This code errors: EO596: mutably borrowing an unmutable variable
// A function can't modify a value it is borrowing through a reference
fn main() {
    let s = String::from("hello");
    
    change(&s);

}

fn change(some_string: &String) {
    some_string.push_str(", world");
}


