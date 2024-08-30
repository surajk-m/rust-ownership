use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

pub fn atomic_example() {
    // Use Arc to share the counter between threads
    let counter = Arc::new(AtomicUsize::new(0));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            // Clone the Arc to move it into the thread
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..1000 {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", counter.load(Ordering::SeqCst));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic() {
        atomic_example();
    }
}
