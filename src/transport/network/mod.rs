use crate::rmb;
use crate::transport;

pub struct TransportNetwork {
    name: &'static str,
}

impl TransportNetwork {
    pub fn new() -> TransportNetwork {
        TransportNetwork {
            name: "network"
        }
    }

    pub fn init(&self) -> Result<String, String> {
        Ok("Sucess".to_string())
    }

}

impl<'a> transport::Transport for TransportNetwork   {
    fn name(&self) -> &'static str {
        &self.name
    }

    fn register(&self) -> Result<String, String> {
        Ok("Success".to_string())
    }
    fn publish(&self, _ch: rmb::Channel, _msg: &dyn rmb::Msg) -> Result<String, String> {
        Ok("Success".to_string())
    }

    fn subscribe(&self, _ch: rmb::Channel, _f: fn(rmb::Channel, &dyn rmb::Msg)-> Result<String, String>) -> Result<String, String> {
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