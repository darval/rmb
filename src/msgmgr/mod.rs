use super::transport;
use super::rmb;

pub struct MsgMgr<'a> {
    inited: bool,
    transports: Vec<(std::ops::Range<u32>,&'a (dyn transport::Transport + 'a))>,
}

impl<'a> MsgMgr<'a> {
    pub fn new(transports: Vec<(std::ops::Range<rmb::Channel>,&'a (dyn transport::Transport + 'a))>) -> MsgMgr {  MsgMgr { transports, inited: false } }
    pub fn init(&mut self) -> Result<String, String> { 
        if self.transports.is_empty() {
            return Err("MsgMgr has no transports defined".to_string());
        }
        for (channel_range,transport) in self.transports.iter() {
            transport.register(channel_range, handle_msg).unwrap();
        }
        self.inited = true;
        Ok("Success".to_string()) 
    }

    pub fn get_transport_names(&self) -> Result<Vec<String>, String> {
        if self.inited == false {
            return Err("Not Inited".to_string());
        }
        let mut v: Vec<String> = Vec::new();
        for t in self.transports.iter() {
            v.push(t.1.name().to_string());
        }
        Ok(v)
    }
    pub fn publish(&mut self, _ch: rmb::Channel, _msg: &'a dyn rmb::Msg) -> Result<String, String> {
        // if self.inited {
            // self.transport.publish(ch, msg)
        // } else {
            Err("Not Inited".to_string())
        // }
    }


    pub fn subscribe(&mut self, _ch: rmb::Channel, _f: fn(rmb::Channel, &dyn rmb::Msg)-> Result<String, String>) -> Result<String, String> {
        // if self.inited {
            // self.transport.subscribe(ch, f)
        // } else {
            Err("Not Inited".to_string())
        // }
    }

}
fn handle_msg(_ch: rmb::Channel, _msg: &dyn rmb::Msg) -> Result<String, String> {
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
