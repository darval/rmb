use super::rmb;

pub struct TransportInternal {
    name: &'static str,
}

impl TransportInternal {
    pub fn new() -> TransportInternal {
        TransportInternal {
            name: "internal"
        }
    }

    pub fn init(&self) -> Result<String, String> {
        Ok("Sucess".to_string())
    }

}

impl<'a> super::rmb::Transport for TransportInternal   {
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
    use crate::rmb::Transport;
    #[test]
    fn test_init() {
        let t = super::TransportInternal::new();
        t.init().unwrap();
    }
    #[test]
    fn get_name() {
        let t = super::TransportInternal::new();
        assert_eq!(t.name(), "internal");
    }
}