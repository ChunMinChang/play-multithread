pub mod resource_readwriter {
    pub struct Resource {
        pub value: u32,
    }

    static mut RESOURCE: Resource = Resource { value: 0 };

    pub fn get_resource<'a>() -> &'a Resource {
        unsafe { &RESOURCE }
    }

    pub fn set_resource(x: u32) {
        unsafe {
            RESOURCE.value = x;
        }
    }

    #[cfg(test)]
    mod tester {
        use super::*;
        use std::thread;
        use std::time::Duration;

        #[test]
        fn test_write_then_read_thread1() {
            set_resource(100);
            thread::sleep(Duration::from_millis(10));
            assert_eq!(100, get_resource().value);
        }

        #[test]
        fn test_write_then_read_thread2() {
            set_resource(200);
            thread::sleep(Duration::from_millis(10));
            assert_eq!(200, get_resource().value);
        }
    }
}
