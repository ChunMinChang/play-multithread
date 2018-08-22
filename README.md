# Playing Multithread in Rust

```mod resource_controller``` has a static vaiable ```RESOURCE```. It can be accessed and updated by any thread any time. Try using ```Mutex<T>``` and ```RwLock<T>``` to wrap the ```RESOURCE``` and see what it can help.

## TO-DO
- Compare *benchmarking* results between ```Mutex<T>``` and ```RwLock<T>```.
- Disable running *bench* functions when entering ```$ caego test```.