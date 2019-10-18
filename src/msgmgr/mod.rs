use std::sync::mpsc::{Sender, Receiver};
use std::sync::{mpsc, Mutex};
use std::thread;
use std::fmt;
use super::transport;
use super::rmb;

#[derive(Clone)]
pub struct RmbMsg<'a> {
    bus: rmb::Bus,
    msg: Box<dyn rmb::Msg + 'a>,
}

const CONTROLBUS: rmb::Bus = 0;

#[derive(Debug,Clone)]
enum ControlMsg {
    Publish,
    Subscribe,
}

impl fmt::Display for ControlMsg {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{:?}", self)
    }
}
impl rmb::Msg for ControlMsg {}

pub struct MsgMgr<'a> {
    inited: bool,
    transports: Mutex<Vec<(std::ops::Range<u32>,Box<dyn transport::Transport<'a> + 'a>)>>,
    que: Mutex<queue::Queue<RmbMsg<'a>>>,
    self_tx: Sender<RmbMsg<'a>>,
    _thread_tx: Sender<RmbMsg<'a>>,
    _self_rx: Receiver<RmbMsg<'a>>,
    _thread_rx: Receiver<RmbMsg<'a>>,
}

impl<'a> MsgMgr<'a> {
    pub fn new(transports: Vec<(std::ops::Range<rmb::Bus>,Box<dyn transport::Transport<'a> + 'a>)>) -> MsgMgr<'a> {  
        let (st, tr): (Sender<RmbMsg<'a>>, Receiver<RmbMsg<'a>>) = mpsc::channel();
        let (tt, sr): (Sender<RmbMsg<'a>>, Receiver<RmbMsg<'a>>) = mpsc::channel();
        let t = Mutex::new(transports);
        MsgMgr { 
            transports: t,
            que: Mutex::new(queue::Queue::new()), 
            inited: false,
            self_tx: st,
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
            transport.register(channel_range, MsgMgr::handle_msg).unwrap();
        }
        self.inited = true;
        Ok("Success".to_string()) 
    }
    pub fn is_inited(&self) -> bool {
        self.inited
    }
    pub fn run(incoming: Receiver<RmbMsg<'static>>, 
                _outgoing: Sender<RmbMsg<'static>>,
                transports: Mutex<Vec<(std::ops::Range<u32>,&'static (dyn transport::Transport + 'static))>>) -> Result<String, String> {
        thread::spawn(move|| {
            loop {
                let msg = incoming.recv().unwrap(); // incoming msg from this thread
                if msg.bus == CONTROLBUS {

                } else {
                    let transports = transports.lock().unwrap();
                    for t in (*transports).iter() {
                        if t.0.contains(&msg.bus) { // if this transport support this bus
                            t.1.publish(msg.bus, &*msg.msg).unwrap();
                        }

                    }
                }
            }
        });
        Ok("Success".to_string()) 
    }
    pub fn get_transport_names(&self) -> Result<Vec<String>, String> {
        if !self.inited {
            return Err("Not Inited".to_string());
        }
        let mut v: Vec<String> = Vec::new();
        let tr = self.transports.lock().unwrap();
        for t in tr.iter() {
            v.push(t.1.name().to_string());
        }
        Ok(v)
    }
    pub fn publish(&mut self, bus: rmb::Bus, msg: Box<dyn rmb::Msg + 'a> ) -> Result<String, String> {
        if self.is_inited() {
            if bus == CONTROLBUS {
                return Err("Bus 0 (ControlBus) is for internal use only".to_string());
            }
            let msg = RmbMsg { bus, msg }; 
            self.self_tx.send(msg).unwrap();
            Ok("Success".to_string())
        } else {
            Err("Not Inited".to_string())
        }
    }


    pub fn subscribe(&mut self, _bus: rmb::Bus, _f: fn(rmb::Bus, Box<dyn rmb::Msg + 'a>)-> Result<String, String>) -> Result<String, String> {
        // if self.inited {
            // self.transport.subscribe(ch, f)
        // } else {
            Ok("Not Implemented".to_string())
        // }
    }

    fn handle_msg(&mut self, bus: rmb::Bus, msg: Box<dyn rmb::Msg + 'a>) -> Result<String, String> {
        let mut q = self.que.lock().unwrap();
        let msg = RmbMsg { bus, msg };
        q.queue(msg).unwrap();
        Ok("".to_string())
    }

}
#[cfg(test)]
mod tests {
    use crate::msgmgr;
    use crate::transport::{local,internal};

    #[test]
   fn test_init_success() {
        let t = Box::new(local::TransportLocal::new());
        let mut mm = msgmgr::MsgMgr::new(vec![(0..10,t)]);
        mm.init().unwrap();
    }
    #[test]
   fn test_init_no_transport() {
        let mut t = msgmgr::MsgMgr::new(vec![]);
        let e = t.init();
        assert_eq!(e, Err("MsgMgr has no transports defined".to_string()));
    }
    #[test]
    fn get_transport_names() {
        let it = Box::new(internal::TransportInternal::new());
        let lt = Box::new(local::TransportLocal::new());
        let mut mm = msgmgr::MsgMgr::new(vec![(0..10,it), (11..20, lt)]);
        mm.init().unwrap();
        let names = mm.get_transport_names().unwrap();
        assert_eq!(names.len(), 2);
        assert_eq!(names[0], "internal".to_string());
        assert_eq!(names[1], "local".to_string());
    }
}
