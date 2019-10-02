use super::transport;
use super::rmb;

pub struct MsgMgr<'a> {
    transport: &'a (dyn transport::Transport + 'a),
    inited: bool,

}

impl<'a> MsgMgr<'a> {
    pub fn new(transport: &dyn transport::Transport) -> MsgMgr {  MsgMgr { transport, inited: false }    }
    pub fn init(&mut self) -> Result<String, String> { 
        self.inited = true;
        Ok("Success".to_string()) 
    }
    pub fn get_transport_name(&self) -> Result<String, String> {
        if self.inited {
            Ok(self.transport.name().to_string())
        } else {
            Err("Not Inited".to_string())
        }
    }
    pub fn publish(&mut self, ch: rmb::Channel, msg: &'a dyn rmb::Msg) -> Result<String, String> {
        if self.inited {
            self.transport.publish(ch, msg)
        } else {
            Err("Not Inited".to_string())
        }
    }

    pub fn subscribe(&mut self, ch: rmb::Channel, f: fn(rmb::Channel, &dyn rmb::Msg)-> Result<String, String>) -> Result<String, String> {
        if self.inited {
            self.transport.subscribe(ch, f)
        } else {
            Err("Not Inited".to_string())
        }
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
