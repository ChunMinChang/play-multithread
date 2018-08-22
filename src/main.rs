extern crate play_multithread;
use play_multithread::resource_controller::*;
use std::thread;
use std::time::Duration;

fn main() {
    const SLEEP_TIME: u64 = 10;
    let mut handles = vec![];

    // The `assert`s in the following threads are very likely to fail.
    // The locks for the static `RESOURCE` are released after calling
    // `set_resource` thus the `value` in `RESOURCE` may be cahnged
    // before calling `get_resource()`.
    for i in 1..5 {
        handles.push(thread::spawn(move || {
            set_resource(Resource { value: i });
            thread::sleep(Duration::from_millis(SLEEP_TIME));
            assert_eq!(i, unsafe { (*get_resource()).value });
        }));
    }

    // The `assert`s in the following threads are always correct since all the
    // operations to the static `RESOURCE` are in the same critical sections.
    for i in 6..10 {
        handles.push(thread::spawn(move || {
            // It's ok to share mutable borrow among threads.
            let mut guard = take_control().write().unwrap();
            (*guard).value = i;
            thread::sleep(Duration::from_millis(SLEEP_TIME));
            assert_eq!(i, (*guard).value);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
