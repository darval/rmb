use crate::rmb;
use crate::transport::{Transport,Bandwidth};

pub struct TransportLocal {
    name: &'static str,
    bw: Bandwidth,
}

impl TransportLocal {
    pub fn new() -> TransportLocal {
        TransportLocal {
            name: "local",
            bw: Bandwidth::Medium
        }
    }

    pub fn init(&self) -> Result<String, String> {
        Ok("Success".to_string())
    }

}

impl<'a> Transport for TransportLocal   {
    fn name(&self) -> &'static str {
        &self.name
    }
    fn bandwidth(&self) -> &Bandwidth {
        &self.bw
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
    use crate::transport::local;

    #[test]
    fn test_init() {
        let t = local::TransportLocal::new();
        t.init().unwrap();
    }
}