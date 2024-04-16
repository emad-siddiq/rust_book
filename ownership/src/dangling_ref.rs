// Dangling References
// Error: E0106: missing lifetime specifier
// Rust doesn't allow dangling references

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
} 
