#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

pub mod resource_controller {
    use std::sync::Mutex;

    pub struct Resource {
        pub value: u32,
    }

    lazy_static! {
        static ref RESOURCE: Mutex<Resource> = Mutex::new(Resource { value: 0 });
    }

    pub fn get_value() -> u32 {
        RESOURCE.lock().unwrap().value
    }

    pub fn set_value(value: u32) {
        let mut guard = RESOURCE.lock().unwrap();
        guard.value = value;
    }

    pub fn get_resource() -> *const Resource {
        let guard = RESOURCE.lock().unwrap();
        &*guard
    }

    pub fn set_resource(resource: Resource) {
        let mut guard = RESOURCE.lock().unwrap();
        *guard = resource;
    }

    pub fn take_control<'a>() -> &'a Mutex<Resource> {
        &RESOURCE
    }

    #[cfg(test)]
    mod tester {
        use super::*;
        use std::thread;
        use std::time::Duration;
        use test::Bencher;
        const SLEEP_TIME: u64 = 10;

        #[test]
        #[ignore]
        // $ cargo test -- --ignored
        fn deadlock_hold_lock_then_asking_lock_again() {
            // The scope of `_guard` is a critical section.
            let _guard = take_control().lock().unwrap();
            // `get_value()` requests for the mutex that is locked by this
            // thread itself again, so it leads to a deadlock.
            get_value();
        }

        #[test]
        fn no_deadlock() {
            // Acquire a mutex, do nothing, then release the mutex.
            get_value();
            // Acquire a mutex that is already released.
            let _guard = take_control().lock().unwrap();
            // The mutex is released after `_guard` goes out '}' below.
        }

        // This test will pass since all the operations to the static
        // `RESOURCE` are in the same critical section.
        #[test]
        fn test_hold_lock_then_write_and_read() {
            // The scope of `guard` is a critical section, so no other threads
            // can read or write `RESOURCE` once `guard` is created.
            let mut guard = take_control().lock().unwrap();
            (*guard).value = 100;
            thread::sleep(Duration::from_millis(SLEEP_TIME));
            assert_eq!(100, (*guard).value);
        }

        // The following tests are very likely to fail.
        // Before checking `get_value()` with `x`, where `x` is the value
        // passed in `set_value(x)`, the `value` in `RESOURCE` may be changed
        // once `set_value(x)` is finished. `set_value` and `get_value` are in
        // different critical section. That's why other threads may change the
        // `value` in `RESOURCE` within the time after calling `set_value()`
        // and before calling `get_value()`.
        #[test]
        fn test_write_then_read_thread1() {
            set_value(200);
            thread::sleep(Duration::from_millis(SLEEP_TIME));
            assert_eq!(200, get_value());
        }

        #[test]
        fn test_write_then_read_thread2() {
            set_resource(Resource { value: 300 });
            thread::sleep(Duration::from_millis(SLEEP_TIME));
            unsafe {
                let resource = &*get_resource();
                assert_eq!(300, resource.value);
            }
        }

        #[bench]
        fn bench_read_write_with_mutex(b: &mut Bencher) {
            b.iter(|| {
                let mut handles = vec![];
                for i in 0..9 {
                    handles.push(thread::spawn(move || {
                        if i % 2 == 0 {
                            set_value(66);
                        } else {
                            get_value();
                        }
                    }));
                }

                for handle in handles {
                    handle.join().unwrap();
                }
            });
        }
    }
}
