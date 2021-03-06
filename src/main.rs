extern crate play_multithread;
use play_multithread::resource_controller::*;
use std::thread;
use std::time::Duration;

fn main() {
    const SLEEP_TIME: u64 = 10;
    let mut handles = vec![];

    // The following `assert` are very likely to fail since
    // methods in `resource_controller` are not thread-safe!

    for i in 1..5 {
        handles.push(thread::spawn(move || {
            set_resource(Resource { value: i });
            thread::sleep(Duration::from_millis(SLEEP_TIME));
            assert_eq!(i, get_resource().value);
        }));
    }

    for i in 6..10 {
        handles.push(thread::spawn(move || {
            // It's ok to share mutable borrow among threads.
            let resource: &mut Resource = take_control();
            resource.value = i;
            thread::sleep(Duration::from_millis(SLEEP_TIME));
            assert_eq!(i, resource.value);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
