use std::sync::{Arc, Mutex, RwLock};
use std::thread;

pub fn mutex_example() {
    // Use Arc to share the Mutex between threads
    let data = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            // Clone the Arc to move it into the thread
            let data = Arc::clone(&data);
            thread::spawn(move || {
                // Lock the Mutex
                let mut data = data.lock().unwrap();
                *data += 1;
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final mutex value: {}", *data.lock().unwrap());
}

pub fn rwlock_example() {
    let data = RwLock::new(0);

    {
        let mut data = data.write().unwrap();
        *data += 1;
    }

    {
        let data = data.read().unwrap();
        println!("Current rwlock value: {}", *data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutex() {
        mutex_example();
    }

    #[test]
    fn test_rwlock() {
        rwlock_example();
    }
}
