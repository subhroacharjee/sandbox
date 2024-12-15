use std::{
    thread::{self},
    time::Duration,
};

use aloy_engine::{
    core::logger::init_logger, event_system::engine_events::application_events::ApplicationEvents,
};

mod handlers;

fn main() {
    let mut engine = aloy_engine::core::runner::applications::Application::default();
    // init_logger();

    handlers::add_handlers_for_example_event(&mut engine);
    handlers::add_handler_for_event_with_data(&mut engine);
    handlers::add_event_with_out_data();

    println!(":::::::::::::::::::Before run is called, we call dispatch");
    engine.dispatch(&ApplicationEvents::ExampleEvent);
    thread::sleep(Duration::from_secs(1));
    println!(":::::::::::::::::::Before run is called, dispatch has ended");

    handlers::add_events_with_data();

    engine.run();
}
