use std::cell::{Cell, RefCell};

// Example with Cell<T>
pub fn cell_example() {
    let data = Cell::new(10);
    data.set(20);
    println!("Cell value: {}", data.get());
}

// Example with RefCell<T>
pub fn refcell_example() {
    let data = RefCell::new(vec![1, 2, 3]);
    data.borrow_mut().push(4);
    println!("RefCell value: {:?}", data.borrow());
}

// Example with Atomic types
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn atomic_example() {
    let atomic = AtomicUsize::new(5);
    atomic.fetch_add(10, Ordering::SeqCst);
    println!("AtomicUsize value: {}", atomic.load(Ordering::SeqCst));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell() {
        cell_example();
    }

    #[test]
    fn test_refcell() {
        refcell_example();
    }

    #[test]
    fn test_atomic() {
        atomic_example();
    }
}
