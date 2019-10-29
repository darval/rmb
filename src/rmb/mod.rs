use std::fmt::Display;
use std::any::Any;
use std::sync::mpsc;

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
    outgoing: mpsc::Sender<msgmgr::RmbMsg>,
    inited: bool,
}

impl<'a> Rmb {
    pub fn new(msgmgr: msgmgr::MsgMgr) -> Rmb {  
        let (s,_) = mpsc::channel();
        Rmb { msgmgr, outgoing: s, inited: false }
    }
    pub fn init(&mut self) -> Result<String, String> {
        self.msgmgr.init().unwrap();
        self.inited = true;
        Ok("Success".to_string()) 
    }
    pub fn run(&'static mut self) -> Result<String, String> {
        if self.inited {
            if let Ok(outgoing) = msgmgr::MsgMgr::run(&self.msgmgr.transports, &self.msgmgr.subscribers) {
                self.outgoing = outgoing;
                Ok("Success".to_string())
            } else {
                Err("Failed to run".to_string())
            }
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
            if bus == msgmgr::CONTROLBUS {
                return Err("Bus 0 (ControlBus) is for internal use only".to_string());
            }
            let msg = msgmgr::RmbMsg { bus, msg };
            self.outgoing.send(msg).unwrap();
            Ok("Success".to_string())
        } else {
            Err("Not Inited".to_string())
        }
    }

    pub fn subscribe(&mut self, bus: Bus, f: fn(Bus)-> Result<String, String>) -> Result<String, String> {
        if self.inited {
            let m = Box::new(msgmgr::SubscribeMsg { b: bus, f: f });
            let sm = msgmgr::RmbMsg {
                bus: msgmgr::CONTROLBUS,
                msg: m,
            };
            self.outgoing.send(sm).unwrap();
            Ok("Success".to_string())
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
}
