pub mod resource_controller {
    pub struct Resource {
        pub value: u32,
    }

    static mut RESOURCE: Resource = Resource { value: 0 };

    pub fn get_resource<'a>() -> &'a Resource {
        unsafe { &RESOURCE }
    }

    pub fn set_resource(resource: Resource) {
        unsafe {
            RESOURCE = resource;
        }
    }

    pub fn take_control<'a>() -> &'a mut Resource {
        unsafe { &mut RESOURCE }
    }

    #[cfg(test)]
    mod tester {
        use super::*;
        use std::thread;
        use std::time::Duration;
        const SLEEP_TIME: u64 = 10;

        #[test]
        fn test_write_then_read_thread1() {
            let resource: &mut Resource = take_control();
            resource.value = 100;
            thread::sleep(Duration::from_millis(SLEEP_TIME));
            assert_eq!(100, resource.value);
        }

        #[test]
        fn test_write_then_read_thread2() {
            set_resource(Resource { value: 200 });
            thread::sleep(Duration::from_millis(SLEEP_TIME));
            assert_eq!(200, get_resource().value);
        }

        #[test]
        fn test_write_then_read_thread3() {
            set_resource(Resource { value: 300 });
            thread::sleep(Duration::from_millis(SLEEP_TIME));
            assert_eq!(300, get_resource().value);
        }
    }
}
