extern crate multithread;
use multithread::resource_readwriter::*;
use std::thread;
use std::time::Duration;

fn main() {
    let mut handles = vec![];
    for i in 1..10 {
        handles.push(thread::spawn(move || {
            set_resource(i);
            thread::sleep(Duration::from_millis(10));
            assert_eq!(i, get_resource().value);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
