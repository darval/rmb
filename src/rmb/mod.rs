use std::fmt;

pub type Channel = u32;

pub trait Msg: fmt::Display {

}

pub trait Transport {
    fn name(&self) -> &'static str;

    fn register(&self) -> Result<String, String>;
    fn publish(&self, ch: Channel, msg: &dyn Msg) -> Result<String, String>;
    fn subscribe(&self, ch: Channel, f: fn(Channel, &dyn Msg)-> Result<String, String>) -> Result<String, String>;
}


pub struct Rmb<'a> {
    transport: &'a (dyn Transport + 'a),
    inited: bool,
    registered: bool,
}

impl<'a> Rmb<'a> {
    pub fn new(transport: &dyn Transport) -> Rmb {  Rmb { transport, inited: false, registered: false }    }
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

    pub fn register(&mut self) -> Result<String, String> {
        if self.inited {
            self.transport.register()?;
            self.registered = true;
            Ok("Success".to_string())
        } else {
            Err("Not Inited".to_string())
        }
    }
    pub fn publish(&mut self, ch: Channel, msg: &dyn Msg) -> Result<String, String> {
        if self.registered {
            self.transport.publish(ch, msg)
        } else {
            Err("Not Registered".to_string())
        }
    }

    pub fn subscribe(&mut self, ch: Channel, f: fn(Channel, &dyn Msg)-> Result<String, String>) -> Result<String, String> {
        if self.registered {
            self.transport.subscribe(ch, f)
        } else {
            Err("Not Registered".to_string())
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::local::*;
    use crate::rmb::*;
    //
    // Test to see that we are registered before we call the transport subscribe
    //
    #[test]
    fn test_subscribe_registered() {
        let t = TransportLocal::new();
        let mut r = Rmb::new(&t);
        r.init().unwrap();
        r.register().unwrap();
        r.subscribe(1,|_, _|{Ok("".to_string())}).unwrap();
    }
   #[test]
   #[should_panic(expected = "Not Registered")]
    fn test_subscribe_unregistered() {
        let t = TransportLocal::new();
        let mut r = Rmb::new(&t);
        r.init().unwrap();
        r.subscribe(1,|_, _|{Ok("".to_string())}).unwrap();
    }
}
