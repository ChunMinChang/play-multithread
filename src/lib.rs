#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

pub mod resource_controller {
    use std::sync::RwLock;

    pub struct Resource {
        pub value: u32,
    }

    lazy_static! {
        static ref RESOURCE: RwLock<Resource> = RwLock::new(Resource { value: 0 });
    }

    pub fn get_value() -> u32 {
        RESOURCE.read().unwrap().value
    }

    pub fn set_value(value: u32) {
        let mut guard = RESOURCE.write().unwrap();
        guard.value = value;
    }

    pub fn get_resource() -> *const Resource {
        let guard = RESOURCE.read().unwrap();
        &*guard
    }

    pub fn set_resource(resource: Resource) {
        let mut guard = RESOURCE.write().unwrap();
        *guard = resource;
    }

    pub fn take_control<'a>() -> &'a RwLock<Resource> {
        &RESOURCE
    }

    #[cfg(test)]
    mod tester {
        use super::*;
        use std::thread;
        use std::time::Duration;
        use test::Bencher;
        const SLEEP_TIME: u64 = 10;

        // This test will lead to a deadlock or a panic for PoisonError.
        // TODO: Explain what is `PoisonError`.
        #[test]
        #[ignore]
        // $ cargo test -- --ignored
        fn deadlock_hold_read_lock_then_write() {
            // May fail to get `_guard` since having a PoisonError.
            let _guard = take_control().read().unwrap();
            // Lead to a deadlock if program goes here.
            set_value(66);
        }

        #[test]
        // Commenting `#[ignore]` will lead to a PoisonError.
        #[ignore]
        // $ cargo test -- --ignored
        #[should_panic(expected = "rwlock read lock would result in deadlock")]
        fn deadlock_hold_write_lock_then_read() {
            let _guard = take_control().write().unwrap();
            get_value();
        }

        // This test will pass since all the operations to the static
        // `RESOURCE` are in the same critical section.
        #[test]
        fn test_hold_write_lock_then_write_and_read() {
            // The scope of `guard` is a critical section, so no other threads
            // can read or write `RESOURCE` once `guard` is created.
            let mut guard = take_control().write().unwrap();
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
        fn bench_read_write_with_rwlock(b: &mut Bencher) {
            b.iter(|| {
                let mut handles = vec![];
                for _i in 0..100 {
                    handles.push(thread::spawn(move || {
                        get_value();
                    }));
                }

                for handle in handles {
                    handle.join().unwrap();
                }
            });
        }
    }
}
