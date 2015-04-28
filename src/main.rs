extern crate lxc;
use lxc::*;

fn main() {
  println!("lxc version: {}", lxc::version());
  let container = LxcContainer::new("test",
                                    "/home/arccha/programowanie/rust-lxc/test.txt");
  match container {
    Some(c) => println!("Created lxc container struct."),
    None => println!("Fail")
  }
  
}