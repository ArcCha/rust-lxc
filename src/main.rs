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
      println!("PID: {}", c.init_pid());
      println!("State: {}", c.state());
      println!("Is running: {}", c.is_running());
      println!("Freeze: {}", c.freeze());
      println!("State: {}", c.state());
      println!("Unfreeze: {}", c.unfreeze());
      println!("State: {}", c.state());
   },
    None => println!("Fail")
  }
  
}