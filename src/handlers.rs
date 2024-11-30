pub fn add_handler_for_event_with_data(
    engine: &mut aloy_engine::core::runner::applications::Application,
) {
    if let Some(err) = engine.on_event(
        ApplicationEvents::ExampleEventWithData(1, 2).get_name(),
        move |e| {
            if e.get_name() == ApplicationEvents::ExampleEventWithData(1, 2).get_name() {
                let data_store = e.get_data().unwrap();

                if let Some(coords) = data_store.get_ref::<Vec<i8>>() {
                    println!("::::::::::::::coords {:?}", coords);
                    if coords.len() == 2 {
                        let sum = *coords.first().unwrap() + *coords.get(1).unwrap();
                        println!("::::::::::::::sum of coords is {:}", sum);
                    }
                } else {
                    println!("{:?}", e);
                }
            }
        },
    ) {
        println!("Error of on event {:?}", err);
    }
}

pub fn add_handlers_for_example_event(
    engine: &mut aloy_engine::core::runner::applications::Application,
) {
    let e = ApplicationEvents::ExampleEvent;

    if let Some(err) = engine.on_event(e.get_name(), |e| {
        if e.get_name() == ApplicationEvents::ExampleEvent.get_name() {
            println!("Hello world!");
        }
    }) {
        println!("Error of on event {:?}", err);
    }
}

pub fn add_event_with_out_data() {
    spawn(move || {
        let q = EventQueue::initalize();
        let mut counter = 0;
        loop {
            if counter == 3 {
                break;
            }
            counter += 1;
            if let Err(err) = q.emit(Box::new(ApplicationEvents::ExampleEvent)) {
                println!("err:{:?}", err);
            }
            thread::sleep(Duration::from_secs(1));
            println!("slept for 1 s");
        }
    });
}

pub fn add_events_with_data() {
    spawn(move || {
        let q = event_queue::EventQueue::initalize();
        let mut counter = 0;

        loop {
            if counter == 10 {
                break;
            }
            counter += 1;
            if let Err(err) = q.emit(Box::new(ApplicationEvents::ExampleEventWithData(
                counter,
                counter + 3,
            ))) {
                println!("err:{:?}", err);
            }
            if counter % 2 == 0 {
                thread::sleep(Duration::from_secs(1));
                println!("slept for 1 s for event with data");
            }
        }

        if let Err(err) = q.emit(Box::new(ApplicationEvents::Exit(
            aloy_engine::core::runner::exit_handlers::ExitReason::NORMAL,
        ))) {
            println!("some error before exit {:?}", err);
        };
    });
}
