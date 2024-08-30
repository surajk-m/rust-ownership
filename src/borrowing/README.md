
# Borrowing and Interior Mutability in Rust

Borrowing allows multiple parts of the code to access a resource without taking ownership of it. Rust enforces strict borrowing rules to ensure memory safety, preventing data races and other concurrency issues.

## Key Concepts:
- **Immutable Borrowing:** Allows multiple references to a resource, but none of them can modify it.
- **Mutable Borrowing:** Allows a single mutable reference, which can modify the resource.
- **Interior Mutability:** A design pattern in Rust that allows mutation of data even when there are immutable references to it.

### Examples:
- **Cell<T>:** Allows interior mutability for `Copy` types.
- **RefCell<T>:** Allows interior mutability for non-`Copy` types with runtime borrow checking.

### Example:
```rust
use std::cell::RefCell;

let data = RefCell::new(vec![1, 2, 3]);
data.borrow_mut().push(4);  // Mutating the vector inside RefCell
println!("{:?}", data.borrow());  // Borrowing the vector immutably
```
    