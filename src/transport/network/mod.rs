use crate::rmb;
use crate::msgmgr;
use crate::transport::{Transport,Bandwidth};

pub struct TransportNetwork {
    name: &'static str,
    bw: Bandwidth,
    inited: bool,
}

impl TransportNetwork {
    pub fn new() -> TransportNetwork {
        TransportNetwork {
            name: "network",
            bw: Bandwidth::Low,
            inited: false,
        }
    }

    pub fn init(&self) -> Result<String, String> {
        Ok("Sucess".to_string())
    }

}

impl<'a> Transport<'a> for TransportNetwork   {
    fn init(&mut self) -> Result<String, String> {
        self.inited = true;
        Ok("Sucess".to_string())
    }

    fn is_inited(&self) -> bool {
        self.inited
    }
    fn name(&self) -> &'static str {
        &self.name
    }
    fn bandwidth(&self) -> &Bandwidth {
        &self.bw
    }

    fn register(&self, _buses: &std::ops::Range<rmb::Bus>, _handler: fn(&'a mut msgmgr::MsgMgr<'a>, rmb::Bus, Box<dyn rmb::Msg + 'static>)-> Result<String, String>) -> Result<String, String> {
        Ok("Success".to_string())
    }
    fn publish(&self, _bus: rmb::Bus, _msg: &dyn rmb::Msg) -> Result<String, String> {
        Ok("Success".to_string())
    }

}
#[cfg(test)]
mod tests {
    use crate::transport::network;
    #[test]
    fn test_init() {
        let t = network::TransportNetwork::new();
        t.init().unwrap();
    }
}