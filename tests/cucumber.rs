use cucumber::{cucumber, before, after};
use msgbus::{msgmgr,transport::internal};

pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    s: String,
    trans: internal::TransportInternal,
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a new scenario is started
        MyWorld { 
            s: "a default string".to_string(),
            trans: internal::TransportInternal::new(),
        }
    }
}

mod feature_getting_started_steps {
    
use cucumber::steps;
//use msgbus::{msgmgr};

    // Any type that implements cucumber::World + Default can be the world
    steps!(super::MyWorld => {
        given "nothing" |world, _step| {
            world.s = "Some string".to_string();
            // Set up your context in given steps
        };

        when "I choose a transport" |world, _step| {
            // Take actions
            let new_string = format!("{}.", &world.s);
            world.s = new_string;
        };

        then "init the transport" |world, _step| {
            // Check that the outcomes to be observed have occurred
            assert_eq!(world.s, "Some string.");
            world.trans.init().unwrap();
        };

        given "an inited transport" |world, _step| {
            world.trans.init().unwrap();
            assert_eq!(world.trans.is_inited(), true);
        };

        then "init the message manager" |world, _step| {
            let mut mm = crate::msgmgr::MsgMgr::new(vec![(0..10,&world.trans)]);
            mm.init().unwrap();
            assert_eq!(mm.is_inited(), true);
        };

        then regex r"^we can (.*) rules with regex$" |_world, matches, _step| {
            // And access them as an array
            assert_eq!(matches[1], "implement");
        };

        then regex r"^we can also match (\d+) (.+) types$" (usize, String) |_world, num, word, _step| {
            // `num` will be of type usize, `word` of type String
            assert_eq!(num, 42);
            assert_eq!(word, "olika");
        };

        then "we can use data tables to provide more parameters" |_world, step| {
            let table = step.table().unwrap().clone();

            assert_eq!(table.header, vec!["key", "value"]);

            let expected_keys = table.rows.iter().map(|row| row[0].to_owned()).collect::<Vec<_>>();
            let expected_values = table.rows.iter().map(|row| row[1].to_owned()).collect::<Vec<_>>();

            assert_eq!(expected_keys, vec!["a", "b"]);
            assert_eq!(expected_values, vec!["fizz", "buzz"]);
        };
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