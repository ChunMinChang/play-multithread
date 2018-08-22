# Playing Multithread in Rust

```mod resource_controller``` has a static vaiable ```RESOURCE```. It can be accessed and updated by any thread any time. Try using ```Mutex<T>``` and ```RwLock<T>``` to wrap the ```RESOURCE``` and see what it can help.

## Mutex
code is in [mutex branch][mutex].

## RwLock
code is in [rwlock branch][rwlock].

## TO-DO
- Compare *benchmarking* results between ```Mutex<T>``` and ```RwLock<T>```.
- Disable running *bench* functions when entering ```$ cargo test```.
- Understand what ```PoisonError``` is
- Try using [```get_mut```][get_mut]

[mutex]: https://github.com/ChunMinChang/play-multithread/tree/mutex "mutex tree"
[rwlock]: https://github.com/ChunMinChang/play-multithread/tree/rwlock "rwlock tree"
[get_mut]: https://doc.rust-lang.org/std/sync/struct.RwLock.html#method.get_mut "std::sync::RwLock"

