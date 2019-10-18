#![doc(html_playground_url = "https://play.rust-lang.org/")]

//! Work in Progress - not usable yet!
//! 
//! # msgbus
//! `msgbus` provides the ability to publish messages on a bus and subscribe to buses to recieve all the messages 
//! published on that bus.  To support scalability, when you first register to be able to publish on that bus, you indicate what 
//! kind of bandwidth you will require.  The overall message bus is managed by a Message Manager(msgmgr) and that msgmgr will be configured 
//! with various transport capabilities.
//! 
//! # Theory of Operation
//! A message bus is a general communication mechanism which has a many-to-many relationship between publishers (many clients can publish on a given bus)
//! and subscribers (many clients can subscribe to a given bus). In an implementation where we can have many buses, we can have dedicated buses for one-to-one,
//! one-to-many and many-to-one relationships as needed.
//! 
//! In this particular implementation, we can have many buses, and a further enhancement has been added to support scalability.  In the
//! simplest implementation, the publishers and subscribers are threads in a shared application.  Communication between them is considered to be high 
//! bandwidth as it can be implemented as shared/copied messages.  In a slightly scaled up implementation, the publishers and subscribers may exist in 
//! separate applicaitons on the some processor. This medium bandwith implementation can be implemented as shared memory between those applications or other
//! local mechanisms.  A lower bandwith implementation may have those puclishers and subscribers existing on different connected processors.
//! 
//! The enhancement to support this is called the `Transport` and is presented as a trait of which we provide several different examples.  The clients 
//! (pubblishers or subscribers) don't choose the transport, but rather the bandwidth they require.  If during later development, the application must be split
//! across mulitple processes, or multiple processors, the clients require almost no refactoring as they are independent from the transport.
//! 
//! # Publishing
//! Publishing is a non-blocking call to the `msgbus` designated by the `rmb::Bus`.  This is a simple `u32` which you define the means for your specific 
//! application. What you send is a structure with the trait of `rmb::Msg`.  The msg will be put on the bus, whether there is any subscribers or not.
//! 
//! # Subscribing
//! When you subscribe to the a particular bus, your handler will be called for all msgs received from that point forward.  The handler may be a function
//! or closure which you passed to the subscribe call.  The handler will be called in the thread context that the MsgMgr creates (not your thread context). 
//! 
//! # Simple Example
//! 
//! ```
//! use msgbus::{msgmgr,rmb,transport::internal};
//! use std::fmt;
//! 
//! fn main() {
//!     #[derive(Clone)]
//!     struct MyMsg {
//!         s: String,
//!     }
//!     impl rmb::Msg for MyMsg {
//!
//!     }
//!     impl fmt::Display for MyMsg {
//!         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//!             write!(f, "{}", self.s)
//!         }
//!     }
//!     fn handler(_bus: rmb::Bus, msg: Box<dyn rmb::Msg>)-> Result<String, String> {
//!         println!("{}", msg); 
//!         Ok(msg.to_string())
//!     }
//!
//!     let hello = MyMsg { s: "Hello".to_string() };
//!     let bus = 1;
//!     let t = Box::new(internal::TransportInternal::new());
//!     let mm = msgmgr::MsgMgr::new(vec![(0..10,t)]);
//!     let mut mb = rmb::Rmb::new(mm);
//!     mb.init().unwrap();
//!     mb.subscribe(bus, handler).unwrap();
//!     mb.publish(bus, Box::new(hello)).unwrap();

//! 
//! }
pub mod rmb;
pub mod msgmgr;
pub mod transport;

#[cfg(test)]
mod tests {
    use super::{rmb, msgmgr, transport::local};
    #[test]
    fn test_init() {
        let t = Box::new(local::TransportLocal::new());
        let mm = msgmgr::MsgMgr::new(vec![(0..10,t)]);
        let mut r = rmb::Rmb::new(mm);
        r.init().unwrap();
    }
    #[test]
    #[ignore]
    fn test_simple_subscribe_publish() {
        impl rmb::Msg for String {

        }
        // fn handler(_bus: rmb::Bus, msg: &dyn rmb::Msg)-> Result<String, String> {
        //     println!("{}", msg); 
        //     // assert_eq!(msg.to_string(), "Hello".to_string()); 
            // Ok(msg.to_string())
        // }

        let hello = "Hello".to_string();
        let bus = 1;
        let t = Box::new(local::TransportLocal::new());
        let mm = msgmgr::MsgMgr::new(vec![(0..10,t)]);
        let mut r = rmb::Rmb::new(mm);
        r.init().unwrap();
        // r.subscribe(bus, handler).unwrap();
        r.publish(bus, Box::new(hello)).unwrap();
    }
}
