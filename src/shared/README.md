
## Shared Ownership in Rust

Shared ownership allows multiple parts of the code to own and access a resource simultaneously. Rust provides different types of smart pointers to manage shared ownership, ensuring that resources are properly cleaned up when no longer needed.

### Key Concepts:
- **Rc<T>:** Allows multiple owners in a single-threaded context. It keeps track of the number of owners and deallocates the resource when there are no more owners.
- **Arc<T>:** Similar to `Rc<T>`, but can be used in multi-threaded contexts. It uses atomic operations to manage the reference count safely across threads.

### Example:
```rust
use std::rc::Rc;

let data = Rc::new(vec![1, 2, 3]);
let data1 = Rc::clone(&data);
println!("Data: {:?}", data1);
```
    