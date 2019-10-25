use crate::MyWorld;
use std::fmt;
use cucumber::steps;
use msgbus::{
    rmb,
    msgmgr,
    transport::{internal, local, network, Transport},
};

#[derive(Clone)]
struct MyMsg {
    s: String,
}
impl fmt::Display for MyMsg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.s)
    }
}

// Any type that implements cucumber::World + Default can be the world
steps!(MyWorld<'static> => {
        given regex r"^an already inited (.*) bus$" |world, name, _step| {
           let mut t: Box<dyn Transport + 'static> = match name[1].as_str() {
                "internal" => Box::new(internal::TransportInternal::new()),
                "local" => Box::new(local::TransportLocal::new()),
                "network" => Box::new(network::TransportNetwork::new()),
                _ => panic!("Unknown transport type")
            };
            t.init().unwrap();
            assert_eq!(t.name(), name[1]);
            assert_eq!(t.is_inited(), true);
            let mut mm = msgmgr::MsgMgr::new(vec![(0..10, t)]);
            mm.init().unwrap();
            assert_eq!(mm.is_inited(), true);
            world.b = rmb::Rmb::new(mm);
            world.b.init().unwrap();
            msgmgr::MsgMgr::run().unwrap();
        };

        when regex r"I publish (\d+) message\(s\)" (usize) | world, number, _step | {
            let hello = MyMsg { s: "Hello".to_string() };
            let mut number = number;
            while number > 0 {
                world.b.publish(1, Box::new(hello.clone())).unwrap();
                number = number - 1;
            }
        };

        then regex r"^the pending message count should increment by (\d+)$" (usize) | world, number, _step | {
            assert_eq!(number, world.b.get_pending_count(1));
        };
    //     then regex r"^we can (.*) rules with regex$" |_world, matches, _step| {
    //         // And access them as an array
    //         assert_eq!(matches[1], "implement");
    //     };

        // then regex r"^we can also match (\d+) (.+) types$" (usize, String) |_world, num, word, _step| {
        //     // `num` will be of type usize, `word` of type String
        //     assert_eq!(num, 42);
        //     assert_eq!(word, "olika");
        // };

    //     then "we can use data tables to provide more parameters" |_world, step| {
    //         let table = step.table().unwrap().clone();

    //         assert_eq!(table.header, vec!["key", "value"]);

    //         let expected_keys = table.rows.iter().map(|row| row[0].to_owned()).collect::<Vec<_>>();
    //         let expected_values = table.rows.iter().map(|row| row[1].to_owned()).collect::<Vec<_>>();

    //         assert_eq!(expected_keys, vec!["a", "b"]);
    //         assert_eq!(expected_values, vec!["fizz", "buzz"]);
    //     };
    });
