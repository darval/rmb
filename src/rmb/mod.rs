use std::fmt::Display;
use std::any::Any;
use super::msgmgr;

pub type Bus = u32;

pub trait Msg: Send + Sync + Display + MsgClone + Any + 'static {
    fn as_any(&self) -> &dyn Any;
}

pub trait MsgClone {
    fn clone_box(&self) -> Box<dyn Msg>;
}

impl<T: 'static + Msg + Clone> MsgClone for T {
    fn clone_box(&self) -> Box<dyn Msg> {
        Box::new(self.clone())
    }
}

impl<T: Send + Sync + Display + Clone + Any> Msg for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<'a> Clone for Box<dyn Msg + 'a> {
    fn clone(&self) -> Box<dyn Msg + 'a> {
        self.clone_box()
    }
}

pub struct Rmb {
    msgmgr: msgmgr::MsgMgr,
    inited: bool,
}

impl<'a> Rmb {
    pub fn new(msgmgr: msgmgr::MsgMgr) -> Rmb {  Rmb { msgmgr, inited: false }    }
    pub fn init(&mut self) -> Result<String, String> {
        self.msgmgr.init().unwrap();
        self.inited = true;
        Ok("Success".to_string()) 
    }
    pub fn run(&self) -> Result<String, String> {
        if self.inited {
            msgmgr::MsgMgr::run(self.msgmgr.thread_rx, self.msgmgr.self_tx, self.msgmgr.transports, self.msgmgr.subscribers )
        } else {
           Err("Not Inited".to_string())
        }
    }
    pub fn get_transport_names(&self) -> Result<Vec<String>, String> {
        if self.inited {
            Ok(self.msgmgr.get_transport_names().unwrap())
        } else {
            Err("Not Inited".to_string())
        }
    }

    pub fn publish(&mut self, bus: Bus, msg: Box<dyn Msg + 'a>) -> Result<String, String> {
        if self.inited {
            self.msgmgr.publish(bus, msg)
        } else {
            Err("Not Inited".to_string())
        }
    }

    pub fn subscribe(&mut self, bus: Bus, f: fn(Bus)-> Result<String, String>) -> Result<String, String> {
        if self.inited {
            self.msgmgr.subscribe(bus, f)
        } else {
            Err("Not Inited".to_string())
        }
    }
    pub fn get_pending_count(&self, bus: Bus) -> usize {
        if self.inited {
            self.msgmgr.get_pending_count(bus)
        } else {
            0
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
        let mm = msgmgr::MsgMgr::new(vec![(0..10,&t)]);
        let mut r = Rmb::new(mm);
        r.init().unwrap();
        r.subscribe(1,|_, _|{Ok("".to_string())}).unwrap();
    }
   #[test]
   #[ignore]
   #[should_panic(expected = "Not Registered")]
    fn test_subscribe_unregistered() {
        let t = local::TransportLocal::new();
        let mm = msgmgr::MsgMgr::new(vec![(0..10,&t)]);
        let mut r = Rmb::new(mm);
        r.init().unwrap();
        r.subscribe(1,|_, _|{Ok("".to_string())}).unwrap();
    }
}
