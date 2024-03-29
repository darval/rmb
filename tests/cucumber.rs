use cucumber::{after, before, cucumber};
use msgbus::{rmb,msgmgr,transport::internal};
pub mod feature_getting_started_steps;
pub mod feature_publishing_steps;

pub struct MyWorld {
    // You can use this struct for mutable context in scenarios.
    s: String,
    b: Box<rmb::Rmb>,
}

impl<'a> cucumber::World for MyWorld {}
impl<'a> std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // This function is called every time a new scenario is started
        MyWorld {
            s: "a default string".to_string(),
            b: Box::new(rmb::Rmb::new(msgmgr::MsgMgr::new(vec!((0..10, &internal::TransportInternal::new()))))),
        }
    }
}

// Declares a before handler function named `a_before_fn`
before!(a_before_fn => |_scenario| {

});

// Declares an after handler function named `an_after_fn`
after!(an_after_fn => |_scenario| {

});

// A setup function to be called before everything else
fn setup() {}

cucumber! {
    features: "./tests/features", // Path to our feature files
    world: ::MyWorld, // The world needs to be the same for steps and the main cucumber call
    steps: &[
        feature_getting_started_steps::steps, // the `steps!` macro creates a `steps` function in a module
        feature_publishing_steps::steps,
    ],
    setup: setup, // Optional; called once before everything
    before: &[
        a_before_fn // Optional; called before each scenario
    ],
    after: &[
        an_after_fn // Optional; called after each scenario
    ]
}
