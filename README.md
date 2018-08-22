# Playing Multithread in Rust

The method of ```mod resource_controller``` in *master* brach is not *thread-safe*.
Try solving this problem by using ```Mutex<T>``` and ```RwLock<T>```.

## TO-DO
- Compare *benchmarking* results between ```Mutex<T>``` and ```RwLock<T>```.
- Disable running *bench* functions when entering ```$ caego test```.