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
        // The `assert`s in the following thread are very likely to fail.
        // The lock for `RESOURCE` is released after finishing `set_resource`
        // thus the `value` in `RESOURCE` may be cahnged before calling
        // `get_resource()`.
        handles.push(thread::spawn(move || {
            set_resource(Resource { value: i });
            thread::sleep(Duration::from_millis(SLEEP_TIME));
            assert_eq!(i, unsafe { (*get_resource()).value });
        }));
    }

    for i in 6..10 {
        // The `assert`s in the following thread are always correct since
        // they are operated within the same critical sections.
        handles.push(thread::spawn(move || {
            // It's ok to share mutable borrow among threads.
            let mut guard = take_control().lock().unwrap();
            (*guard).value = i;
            thread::sleep(Duration::from_millis(SLEEP_TIME));
            assert_eq!(i, (*guard).value);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
