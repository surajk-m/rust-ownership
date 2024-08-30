
# Advanced Concurrency Patterns in Rust

Rust’s ownership and borrowing model shines in concurrent programming, providing strong compile-time guarantees against data races. This section explores advanced patterns like worker pools and message passing.

## Key Concepts:
- **Channels:** Rust provides `mpsc` channels for message passing between threads. `mpsc` stands for "multiple producer, single consumer," but you can clone the `Sender` to have multiple producers.
- **Worker Pool:** A common concurrency pattern where a pool of worker threads processes tasks from a shared queue.

### Example:
```rust
use std::sync::mpsc::channel;
use std::thread;

let (tx, rx) = channel();
for i in 0..4 {
    let tx = tx.clone();
    thread::spawn(move || {
        tx.send(i).unwrap();
    });
}

for _ in 0..4 {
    println!("Got: {}", rx.recv().unwrap());
}
```

### Producer-Consumer Pattern

### Overview
In this pattern, one or more threads (producers) generate data that is consumed by one or more other threads (consumers). This is a common scenario in real-world applications, like processing tasks from a queue.

### Example:
```rust
let data = Arc::new(Mutex::new(vec![]));
let (tx, rx) = channel();

let producer = thread::spawn(move || {
    for i in 0..5 {
        let mut data = data.lock().unwrap();
        data.push(i);
        tx.send(()).unwrap();
    }
});

let consumer = thread::spawn(move || {
    for _ in 0..5 {
        rx.recv().unwrap();
        let data = data.lock().unwrap();
        println!("Consumer got: {:?}", data);
    }
});

producer.join().unwrap();
consumer.join().unwrap();
```

### Interior Mutability in Rust

Rust typically enforces strict ownership and mutability rules. However, there are scenarios where you might need to mutate data through a shared reference. Rust allows this via a pattern known as **interior mutability**.

### `Cell<T>`
- A `Cell<T>` allows you to mutate a value even if the `Cell` itself is immutable.
- It’s best suited for Copy types (like integers), as it doesn't offer reference-based borrowing.

### Example:
```rust
let data = Cell::new(10);
data.set(20);
println!("Cell value: {}", data.get());
```

In this example, even though `data` is immutable, the value inside it can be changed because of the `Cell` wrapper.

### `RefCell<T>`
- A `RefCell<T>` allows you to enforce borrowing rules at runtime instead of compile-time.
- It enables you to have multiple mutable borrows or immutable borrows, but only one of each kind at any given time.

### Example:
```rust
let data = RefCell::new(vec![1, 2, 3]);
data.borrow_mut().push(4);
println!("RefCell value: {:?}", data.borrow());
```

Here, `RefCell` allows mutable access to the data even when it’s part of a shared reference.

### Atomic Types
- Atomic types (`AtomicUsize`, `AtomicBool`, etc.) allow safe concurrent mutation across threads.
- They are crucial in multi-threaded contexts where you need to perform atomic operations.

### Example:
```rust
let atomic = AtomicUsize::new(5);
atomic.fetch_add(10, Ordering::SeqCst);
println!("AtomicUsize value: {}", atomic.load(Ordering::SeqCst));
```

In this example, the `AtomicUsize` provides a way to safely modify a value across threads without a lock.

