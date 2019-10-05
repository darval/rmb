use crate::rmb;
use crate::transport::{Transport,Bandwidth};

pub struct TransportNetwork {
    name: &'static str,
    bw: Bandwidth,
}

impl TransportNetwork {
    pub fn new() -> TransportNetwork {
        TransportNetwork {
            name: "network",
            bw: Bandwidth::Low,
        }
    }

    pub fn init(&self) -> Result<String, String> {
        Ok("Sucess".to_string())
    }

}

impl<'a> Transport for TransportNetwork   {
    fn name(&self) -> &'static str {
        &self.name
    }
    fn bandwidth(&self) -> &Bandwidth {
        &self.bw
    }

    fn register(&self, _buses: &std::ops::Range<rmb::Bus>, _handler: fn(rmb::Bus, &dyn rmb::Msg)-> Result<String, String>) -> Result<String, String> {
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