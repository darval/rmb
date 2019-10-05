
Rust Message Bus - a generic publish/subscribe crate - work in progress - not complete yet
# msgbus
`msgbus` provides the ability to publish messages on a bus and subscribe to a bus to recieve all the messages 
published on that bus.  To support scalability, when you first register to be able to publish on that bus, you indicate what 
kind of bandwidth you will require.  The overall message bus is managed by a Message Manager(msgmgr) and that msgmgr will be configured 
with various transport capabilities.

# Theory of Operation
A message bus is a general communication mechanism which has a many-to-many relationship between publishers (many clients can publish on a given bus)
and subscribers (many clients can subscribe to a given bus). In an implementation where we can have many buses, we can have dedicated buses for one-to-one,
one-to-many and many-to-one relationships as needed.

In this particular implementation, we can have many buses, and a further enhancement has been added to support scalability.  In the
simplest implementation, the publishers and subscribers are threads in a shared application.  Communication between them is considered to be high 
bandwidth as it can be implemented as shared/copied messages.  In a slightly scaled up implementation, the publishers and subscribers may exist in 
separate applicaitons on the some processor. This medium bandwith implementation can be implemented as shared memory between those applications or other
local mechanisms.  A lower bandwith implementation may have those puclishers and subscribers existing on different connected processors.

The enhancement to support this is called the `Transport` and is presented as a trait of which we provide several different examples.  The clients 
(pubblishers or subscribers) don't choose the transport, but rather the bandwidth they require.  If during later development, the application must be split
across mulitple processes, or multiple processors, the clients require almost no refactoring as they are independent from the transport.
 
 # Publishing
Publishing is a non-blocking call to the `msgbus` designated by the `rmb::Bus`.  This is a simple `u32` which you define the means for your specific 
application. What you send is a structure with the trait of `rmb::Msg`.  The msg will be put on the bus, whether there is any subscribers or not.

# Subscribing
When you subscribe to the a particular bus, your handler will be called for all msgs received from that point forward.  The handler may be a function
or closure which you passed to the subscribe call.  The handler will be called in the thread context that the MsgMgr creates (not your thread context). 
 
# Simple Example

```
use msgbus::{msgmgr,rmb,transport::internal};
use std::fmt;

fn main() {
    struct MyMsg {
        s: String,
    }
    impl rmb::Msg for MyMsg {
    }
    impl fmt::Display for MyMsg {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.s)
        }
    }
    fn handler(_bus: rmb::Bus, msg: &dyn rmb::Msg)-> Result<String, String> {
        println!("{}", msg); 
        Ok(msg.to_string())
    }

    let t = internal::TransportInternal::new();
    let mut mm = msgmgr::MsgMgr::new(vec![(0..10,&t)]);
    let mut mb = rmb::Rmb::new(&mut mm);
    mb.init().unwrap();
    let hello = MyMsg { s: "Hello".to_string() };
    let bus = 1;
    mb.subscribe(bus, handler).unwrap();
    mb.publish(bus, &hello).unwrap();
}
```
