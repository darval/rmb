use super::rmb;
pub mod internal;
pub mod local;
pub mod network;

pub trait Transport {
    fn name(&self) -> &'static str;

    fn register(&self) -> Result<String, String>;
    fn publish(&self, ch: rmb::Channel, msg: &dyn rmb::Msg) -> Result<String, String>;
    fn subscribe(&self, ch: rmb::Channel, f: fn(rmb::Channel, &dyn rmb::Msg)-> Result<String, String>) -> Result<String, String>;
}

#[cfg(test)]
mod tests {
    use crate::transport::{Transport, local, internal, network};

    #[test]
    fn test_init() {
        let t = local::TransportLocal::new();
        t.init().unwrap();
    }
    #[test]
    fn get_local_name() {
        let t = local::TransportLocal::new();
        assert_eq!(t.name(), "local");
    }
    #[test]
    fn get_internal_name() {
        let t = internal::TransportInternal::new();
        assert_eq!(t.name(), "internal");
    }
    #[test]
    fn get_network_name() {
        let t = network::TransportNetwork::new();
        assert_eq!(t.name(), "network");
    }
}