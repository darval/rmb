use crate::rmb;
use crate::transport::{Transport,Bandwidth};

pub struct TransportInternal {
    name: &'static str,
    bw: Bandwidth,
    inited: bool,
}

impl TransportInternal {
    pub fn new() -> TransportInternal {
        TransportInternal {
            name: "internal",
            bw: Bandwidth::High,
            inited: false,
        }
    }

    pub fn init(&mut self) -> Result<String, String> {
        self.inited = true;
        Ok("Sucess".to_string())
    }

    pub fn is_inited(&self) -> bool {
        self.inited
    }
}

impl<'a> Transport for TransportInternal   {
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
    use crate::transport::internal;
    #[test]
    fn test_init() {
        let mut t = internal::TransportInternal::new();
        t.init().unwrap();
    }
}