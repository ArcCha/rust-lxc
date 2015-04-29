extern crate lxc;
use lxc::*;

fn main() {
  println!("lxc version: {}", lxc::version());
  let container = LxcContainer::new("apicontainer",
                                    "");
  match container {
    Some(c) => {
      println!("Created lxc container object.");
      if c.is_defined() { println!("Already defined"); } else { println!("Not yet defined"); }
      // if c.rename("test2") { println!("Renamed"); } else { println!("Rename failed"); }
      println!("State: {}", c.state());
   },
    None => println!("Fail")
  }
  
}