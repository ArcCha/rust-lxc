extern crate liblxc;
use liblxc::*;

fn main() {
  println!("lxc version: {}", liblxc::version());
  let container = LxcContainer::new("apicontainer",
                                    "");
  match container {
    Ok(c) => {
      println!("Created lxc container object.");
      if c.is_defined() { println!("Already defined"); } else { println!("Not yet defined"); }
      // if c.rename("test2") { println!("Renamed"); } else { println!("Rename failed"); }
      println!("State: {}", c.state());
      println!("Start: {}", c.start(0, Vec::new()));
      println!("PID: {}", c.init_pid());
      println!("State: {}", c.state());
      println!("Is running: {}", c.is_running());
      println!("Freeze: {}", c.freeze());
      println!("State: {}", c.state());
      println!("Unfreeze: {}", c.unfreeze());
      println!("State: {}", c.state());
      println!("Stop: {}", c.stop());
      println!("State: {}", c.state());
    },
    Err(e) => println!("{}", e)
  }
  
}