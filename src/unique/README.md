
## Static Ownership in Rust

Static ownership in Rust is the simplest form of ownership, where a single owner has exclusive access to a resource for its entire lifetime. This ensures memory safety by preventing any other part of the code from accessing or modifying the resource once ownership has been transferred.

### Key Concepts:
- **Ownership Transfer:** When ownership is moved from one variable to another, the original variable can no longer be used.
- **Memory Safety:** Rust ensures that once ownership is transferred, the previous owner cannot access the resource, preventing data races and memory corruption.

### Example: Basic Ownership (`T`)

```rust
let x = String::from("hello");
let y = x; // Ownership of the String is transferred to y
println!("{}", y); // `y` now owns the data, `x` is no longer valid
```
