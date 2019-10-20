use super::rmb;
use super::msgmgr;
pub mod internal;
pub mod local;
pub mod network;

#[derive(Debug,PartialEq)]
pub enum Bandwidth {
    Low,
    Medium,
    High,
}

pub trait Transport<'a>: Send + Sync {
    fn name(&self) -> &'static str;
    fn init(&mut self) -> Result<String, String>;
    fn is_inited(&self) -> bool;
    fn bandwidth(&self) -> &Bandwidth;
    fn register(&self, buses: &std::ops::Range<rmb::Bus>, handler: fn(&'a mut msgmgr::MsgMgr<'a>, rmb::Bus, Box<dyn rmb::Msg + 'static>)-> Result<String, String>) -> Result<String, String>;
    fn publish(&self, ch: rmb::Bus, msg: &dyn rmb::Msg) -> Result<String, String>;
}

#[cfg(test)]
mod tests {
    use crate::transport::{Transport, Bandwidth, local, internal, network};

    #[test]
    fn test_init() {
        let t = local::TransportLocal::new();
        t.init().unwrap();
    }
    #[test]
    fn get_local_name() {
        let t = local::TransportLocal::new();
        assert_eq!(t.name(), "local");
        assert_eq!(*t.bandwidth(), Bandwidth::Medium)
    }
    #[test]
    fn get_internal_name() {
        let t = internal::TransportInternal::new();
        assert_eq!(t.name(), "internal");
        assert_eq!(*t.bandwidth(), Bandwidth::High)
    }
    #[test]
    fn get_network_name() {
        let t = network::TransportNetwork::new();
        assert_eq!(t.name(), "network");
        assert_eq!(*t.bandwidth(), Bandwidth::Low)

    }
}