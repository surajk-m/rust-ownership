use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

pub fn arc_atomic_example() {
    let counter = Arc::new(AtomicUsize::new(0));

    let handles: Vec<_> = (0..10)
        .map(|_| {
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

pub fn arc_mutex_example() {
    let data = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
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

pub fn arc_rwlock_example() {
    let data = Arc::new(RwLock::new(0));

    {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let mut data = data.write().unwrap();
            *data += 1;
        })
        .join()
        .unwrap();
    }

    {
        let data = data.read().unwrap();
        println!("Final rwlock value: {}", *data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arc_atomic() {
        arc_atomic_example();
    }

    #[test]
    fn test_arc_mutex() {
        arc_mutex_example();
    }

    #[test]
    fn test_arc_rwlock() {
        arc_rwlock_example();
    }
}
