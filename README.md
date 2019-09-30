
Rust Message Bus - a generic publish/subscribe crate - work in progress - not complete yet
# msgbus
`msgbus` provides the ability to publish messages on a channel (or bus) and subscribe to channels to recieve all the messages 
published on that channel.  To support scalability, when you first register to be able to publish on that channel, you indicate what 
kind of bandwidth you will require.  The overall message bus is managed by a Message Manager(msgmgr) and that msgmgr will be configured 
with various transport capabilities.

# Theory of Operation
A message bus is a general communication mechanism which has a many-to-many relationship between publishers (many clients can publish on a given bus)
and subscribers (many clients can subscribe to a given bus). In an implementaiton where we can have many buses, we can have dedicated buses for one-to-one,
one-to-many and many-to-one relationships as needed.

In this particular implementation, we can have many buses, called `Channels`, and a further enhancement has been added to support scalability.  In the
simplest implementation, the publishers and subscribers are threads in a shared application.  Communication between them is considered to be high 
bandwidth as it can be implemented as shared/copied messages.  In a slightly scaled up implementation, the publishers and subscribers may exist in 
separate applicaitons on the some processor. This medium bandwith implementation can be implemented as shared memory between those applications or other
local mechanisms.  A lower bandwith implementation may have those puclishers and subscribers existing on different connected processors.

The enhancement to support this is called the `Transport` and is presented as a trait of which we provide several different examples.  The clients 
(pubblishers or subscribers) don't choose the transport, but rather the bandwidth they require.  If during later development, the application must be split
across mulitple processes, or multiple processors, the clients require almost no refactoring as they are independent from the transport.
 
