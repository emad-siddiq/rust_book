// Valid multiple references

fn main() {


  let mut s = String::from("hello");


  { 
      let _r1 = &mut s;
  } // r1 goes out of scope

  let r2 = &mut s;

  println!("{}", r2);
}
