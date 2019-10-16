use std::fmt::Display;
use super::msgmgr;

pub type Bus = u32;

pub trait Msg: Send + Sync + Display {

}

pub struct Rmb<'a> {
    msgmgr: msgmgr::MsgMgr<'a>,
    inited: bool,
}

impl<'a> Rmb<'a> {
    pub fn new(msgmgr: msgmgr::MsgMgr) -> Rmb {  Rmb { msgmgr, inited: false }    }
    pub fn init(&mut self) -> Result<String, String> {
        self.msgmgr.init().unwrap();
        self.inited = true;
        Ok("Success".to_string()) 
    }

    pub fn get_transport_names(&self) -> Result<Vec<String>, String> {
        if self.inited {
            Ok(self.msgmgr.get_transport_names().unwrap())
        } else {
            Err("Not Inited".to_string())
        }
    }

    pub fn publish(&mut self, bus: Bus, msg: &'a dyn Msg) -> Result<String, String> {
        if self.inited {
            self.msgmgr.publish(bus, msg)
        } else {
            Err("Not Inited".to_string())
        }
    }

    pub fn subscribe(&mut self, bus: Bus, f: fn(Bus, &dyn Msg)-> Result<String, String>) -> Result<String, String> {
        if self.inited {
            self.msgmgr.subscribe(bus, f)
        } else {
            Err("Not Inited".to_string())
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::transport::*;
    use crate::rmb::*;
    //
    // Test to see that we are registered before we call the transport subscribe
    //
    #[test]
    #[ignore]
    fn test_subscribe_registered() {
        let t = Box::new(local::TransportLocal::new());
        let mm = msgmgr::MsgMgr::new(vec![(0..10,t)]);
        let mut r = Rmb::new(mm);
        r.init().unwrap();
        r.subscribe(1,|_, _|{Ok("".to_string())}).unwrap();
    }
   #[test]
   #[ignore]
   #[should_panic(expected = "Not Registered")]
    fn test_subscribe_unregistered() {
        let t = Box::new(local::TransportLocal::new());
        let mm = msgmgr::MsgMgr::new(vec![(0..10,t)]);
        let mut r = Rmb::new(mm);
        r.init().unwrap();
        r.subscribe(1,|_, _|{Ok("".to_string())}).unwrap();
    }
}
