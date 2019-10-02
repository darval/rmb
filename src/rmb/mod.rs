use std::fmt;

use super::msgmgr;

pub type Channel = u32;

pub trait Msg: fmt::Display {

}

pub struct Rmb<'a> {
    msgmgr: &'a mut msgmgr::MsgMgr<'a>,
    inited: bool,
}

impl<'a> Rmb<'a> {
    pub fn new(msgmgr: &'a mut msgmgr::MsgMgr<'a>) -> Rmb<'a> {  Rmb { msgmgr, inited: false }    }
    pub fn init(&mut self) -> Result<String, String> {
        self.msgmgr.init().unwrap();
        self.inited = true;
        Ok("Success".to_string()) 
    }

    pub fn get_transport_name(&self) -> Result<String, String> {
        if self.inited {
            Ok(self.msgmgr.get_transport_name().unwrap())
        } else {
            Err("Not Inited".to_string())
        }
    }

    pub fn publish(&mut self, ch: Channel, msg: &'a dyn Msg) -> Result<String, String> {
        if self.inited {
            self.msgmgr.publish(ch, msg)
        } else {
            Err("Not Inited".to_string())
        }
    }

    pub fn subscribe(&mut self, ch: Channel, f: fn(Channel, &dyn Msg)-> Result<String, String>) -> Result<String, String> {
        if self.inited {
            self.msgmgr.subscribe(ch, f)
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
        let t = local::TransportLocal::new();
        let mut mb = msgmgr::MsgMgr::new(&t);
        let mut r = Rmb::new(&mut mb);
        r.init().unwrap();
        r.subscribe(1,|_, _|{Ok("".to_string())}).unwrap();
    }
   #[test]
   #[ignore]
   #[should_panic(expected = "Not Registered")]
    fn test_subscribe_unregistered() {
        let t = local::TransportLocal::new();
        let mut mb = msgmgr::MsgMgr::new(&t);
        let mut r = Rmb::new(&mut mb);
        r.init().unwrap();
        r.subscribe(1,|_, _|{Ok("".to_string())}).unwrap();
    }
}
