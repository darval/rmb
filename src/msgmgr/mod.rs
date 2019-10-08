use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Mutex};
use std::thread;
use super::transport;
use super::rmb;

pub struct MsgMgr<'a> {
    inited: bool,
    transports: Mutex<Vec<(std::ops::Range<u32>,&'a (dyn transport::Transport + 'a))>>,
    _self_tx: Sender<Box<dyn rmb::Msg+'a>>,
    _thread_tx: Sender<Box<dyn rmb::Msg+'a>>,
    _self_rx: Receiver<Box<dyn rmb::Msg+'a>>,
    _thread_rx: Receiver<Box<dyn rmb::Msg+'a>>,
}

impl<'a> MsgMgr<'a> {
    pub fn new(transports: Vec<(std::ops::Range<rmb::Bus>,&'a (dyn transport::Transport + 'a))>) -> MsgMgr {  
        let (st, tr): (Sender<Box<dyn rmb::Msg +'a>>, Receiver<Box<dyn rmb::Msg +'a>>) = mpsc::channel();
        let (tt, sr): (Sender<Box<dyn rmb::Msg +'a>>, Receiver<Box<dyn rmb::Msg +'a>>) = mpsc::channel();
        let t = Mutex::new(transports);
        MsgMgr { 
            transports: t, 
            inited: false,
            _self_tx: st,
            _thread_rx: tr,
            _thread_tx: tt,
            _self_rx: sr,
        } 
    }
    pub fn init(&mut self) -> Result<String, String> {
        let t = self.transports.lock().unwrap();
        if t.is_empty() {
            return Err("MsgMgr has no transports defined".to_string());
        }
        for (channel_range,transport) in t.iter() {
            transport.register(channel_range, handle_msg).unwrap();
        }
        self.inited = true;
        Ok("Success".to_string()) 
    }
    pub fn run(incoming: Receiver<Box<dyn rmb::Msg>>, 
                _outgoing: Sender<Box<dyn rmb::Msg>>,
                _transports: Mutex<Vec<(std::ops::Range<u32>,&'a (dyn transport::Transport + 'a))>>) -> Result<String, String> {
        thread::spawn(move|| {
            loop {
                let _msg = incoming.recv().unwrap();
            }
        });
        Ok("Success".to_string()) 
    }
    pub fn get_transport_names(&self) -> Result<Vec<String>, String> {
        if self.inited == false {
            return Err("Not Inited".to_string());
        }
        let mut v: Vec<String> = Vec::new();
        let tr = self.transports.lock().unwrap();
        for t in tr.iter() {
            v.push(t.1.name().to_string());
        }
        Ok(v)
    }
    pub fn publish(&mut self, _bus: rmb::Bus, _msg: &'a dyn rmb::Msg) -> Result<String, String> {
        // if self.inited {
            // self.transport.publish(ch, msg)
        // } else {
            Ok("Not Implemented".to_string())
        // }
    }


    pub fn subscribe(&mut self, _bus: rmb::Bus, _f: fn(rmb::Bus, &dyn rmb::Msg)-> Result<String, String>) -> Result<String, String> {
        // if self.inited {
            // self.transport.subscribe(ch, f)
        // } else {
            Ok("Not Implemented".to_string())
        // }
    }

}

fn handle_msg(_bus: rmb::Bus, _msg: &dyn rmb::Msg) -> Result<String, String> {
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use crate::msgmgr;
    use crate::transport::{local,internal};

    #[test]
   fn test_init_success() {
        let t = internal::TransportInternal::new();
        let mut t = msgmgr::MsgMgr::new(vec![(0..10,&t)]);
        t.init().unwrap();
    }
    #[test]
   fn test_init_no_transport() {
        let mut t = msgmgr::MsgMgr::new(vec![]);
        let e = t.init();
        assert_eq!(e, Err("MsgMgr has no transports defined".to_string()));
    }
    #[test]
    fn get_transport_names() {
        let it = internal::TransportInternal::new();
        let lt = local::TransportLocal::new();
        let mut mm = msgmgr::MsgMgr::new(vec![(0..10,&it), (11..20, &lt)]);
        mm.init().unwrap();
        let names = mm.get_transport_names().unwrap();
        assert_eq!(names.len(), 2);
        assert_eq!(names[0], "internal".to_string());
        assert_eq!(names[1], "local".to_string());
    }
}
