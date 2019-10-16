use cucumber::{cucumber, before, after};
use msgbus::msgmgr;

pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    s: String,
    msgmgr: msgmgr::MsgMgr<'static>,
}

impl<'a> cucumber::World for MyWorld {}
impl<'a> std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a new scenario is started
        MyWorld { 
            s: "a default string".to_string(),
            msgmgr: msgmgr::MsgMgr::new(vec![])
        }
    }
}

mod feature_getting_started_steps {
    
use cucumber::steps;
use msgbus::transport::{Transport, internal, local, network};

    // Any type that implements cucumber::World + Default can be the world
    steps!(super::MyWorld => {
        given "nothing" |world, _step| {
            world.s = "Some string".to_string();
            // Set up your context in given steps
        };

        when regex "^I choose a (.*) transport$" |world, _, _step| {
            // Take actions
            let new_string = format!("{}.", &world.s);
            world.s = new_string;
        };

        then regex "^init the (.*) transport$" |world, name, _step| {
            // Check that the outcomes to be observed have occurred
            assert_eq!(world.s, "Some string.");
            let mut t: Box<dyn Transport + 'static> = match name[1].as_str() {
                "internal" => Box::new(internal::TransportInternal::new()),
                "local" => Box::new(local::TransportLocal::new()),
                "network" => Box::new(network::TransportNetwork::new()),
                _ => panic!("Unknown transport type")
            };
            t.init().unwrap();
            assert_eq!(t.name(), name[1]);
        };

        given regex "^an inited (.*) transport$" |world, name, _step| {
           let mut t: Box<dyn Transport + 'static> = match name[1].as_str() {
                "internal" => Box::new(internal::TransportInternal::new()),
                "local" => Box::new(local::TransportLocal::new()),
                "network" => Box::new(network::TransportNetwork::new()),
                _ => panic!("Unknown transport type")
            };
            t.init().unwrap();
            assert_eq!(t.name(), name[1]);
            assert_eq!(t.is_inited(), true);
        };

        then regex "init the (.*) message manager" |world, name, _step| {
           let mut t: Box<dyn Transport + 'static> = match name[1].as_str() {
                "internal" => Box::new(internal::TransportInternal::new()),
                "local" => Box::new(local::TransportLocal::new()),
                "network" => Box::new(network::TransportNetwork::new()),
                _ => panic!("Unknown transport type")
            };
            t.init().unwrap();
            assert_eq!(t.name(), name[1]);
            assert_eq!(t.is_inited(), true);
            world.msgmgr = crate::msgmgr::MsgMgr::new(vec![(0..10, t)]);
            world.msgmgr.init().unwrap();
            assert_eq!(world.msgmgr.is_inited(), true);
        };

        given regex "an inited (.*) msgmgr" |world, name, _step| {
           let mut t: Box<dyn Transport + 'static> = match name[1].as_str() {
                "internal" => Box::new(internal::TransportInternal::new()),
                "local" => Box::new(local::TransportLocal::new()),
                "network" => Box::new(network::TransportNetwork::new()),
                _ => panic!("Unknown transport type")
            };
            t.init().unwrap();
            assert_eq!(t.name(), name[1]);
            assert_eq!(t.is_inited(), true);
            world.msgmgr = crate::msgmgr::MsgMgr::new(vec![(0..10, t)]);
            world.msgmgr.init().unwrap();
            assert_eq!(world.msgmgr.is_inited(), true);
        };
    //     then regex r"^we can (.*) rules with regex$" |_world, matches, _step| {
    //         // And access them as an array
    //         assert_eq!(matches[1], "implement");
    //     };

    //     then regex r"^we can also match (\d+) (.+) types$" (usize, String) |_world, num, word, _step| {
    //         // `num` will be of type usize, `word` of type String
    //         assert_eq!(num, 42);
    //         assert_eq!(word, "olika");
    //     };

    //     then "we can use data tables to provide more parameters" |_world, step| {
    //         let table = step.table().unwrap().clone();

    //         assert_eq!(table.header, vec!["key", "value"]);

    //         let expected_keys = table.rows.iter().map(|row| row[0].to_owned()).collect::<Vec<_>>();
    //         let expected_values = table.rows.iter().map(|row| row[1].to_owned()).collect::<Vec<_>>();

    //         assert_eq!(expected_keys, vec!["a", "b"]);
    //         assert_eq!(expected_values, vec!["fizz", "buzz"]);
    //     };
    });
}

// Declares a before handler function named `a_before_fn`
before!(a_before_fn => |_scenario| {

});

// Declares an after handler function named `an_after_fn`
after!(an_after_fn => |_scenario| {

});

// A setup function to be called before everything else
fn setup() {
    
}

cucumber! {
    features: "./features", // Path to our feature files
    world: ::MyWorld, // The world needs to be the same for steps and the main cucumber call
    steps: &[
        feature_getting_started_steps::steps // the `steps!` macro creates a `steps` function in a module
    ],
    setup: setup, // Optional; called once before everything
    before: &[
        a_before_fn // Optional; called before each scenario
    ], 
    after: &[
        an_after_fn // Optional; called after each scenario
    ] 
}