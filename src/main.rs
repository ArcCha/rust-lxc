extern crate liblxc;
use liblxc::*;

fn main() {
  println!("lxc version: {}", liblxc::version());
  let container = LxcContainer::new("test", None);
  match container {
    Ok(c) => {
      println!("Created lxc container object.");
      let bdevtype = None;
      let specs = BDevSpecs::new();
      let argv = vec!["-d", "ubuntu", "-r", "trusty", "-a", "amd64"];
      if c.create("download", bdevtype, specs, LxcCreateFlag::Verbose, Some(argv)) {
        println!("Sucess!");
      }
      else {
        println!("Fail.");
      }
      if c.is_defined() { println!("Already defined"); } else { println!("Not yet defined"); }
      // if c.rename("test_renamed") { println!("Renamed"); } else { println!("Rename failed"); }
      println!("State: {}", c.state());
      println!("Start: {}", c.start(0, None));
      println!("PID: {}", c.init_pid());
      println!("State: {}", c.state());
      println!("Is running: {}", c.is_running());

      let key = "lxc.hook.pre-start";
      let value = "hi there";
      println!("set: {}", c.set_config_item(key, value));
      println!("Get config item (Ok): {} = {:?}", key, c.get_config_item(key));
      println!("Get config item (Err): {} = {:?}", "err", c.get_config_item("err"));
      let running_key = "lxc.utsname";
      println!("Get running config item (Ok): {} = {:?}", running_key, c.get_running_config_item(running_key));
      println!("Get running config item (Err): {} = {:?}", "err", c.get_running_config_item("err"));
      let key_prefix = "lxc.network.0";
      println!("Get keys list (Ok): {} = {:?}", key_prefix, c.get_keys(key_prefix));
      println!("Get keys list (Err): {} = {:?}", "err", c.get_keys("err"));

      println!("Get interfaces: {:?}", c.get_interfaces());
      println!("Get IP addresses: {:?}", c.get_ips(None, None, 0));

      println!("Freeze: {}", c.freeze());
      println!("State: {}", c.state());
      println!("Unfreeze: {}", c.unfreeze());
      println!("State: {}", c.state());
      println!("Stop: {}", c.stop());
      println!("State: {}", c.state());

      println!("#############################");
      match c.clone(Some("test1"), None, LxcCloneFlag::Void, None, None, 0, None) {
        Ok(c1) => {
          println!("State: {}", c1.state());
          println!("")
        },
        Err(e) => println!("{}", e)
      }
    },
    Err(e) => println!("{}", e)
  }
  
}