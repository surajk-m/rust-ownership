use std::sync::Arc;
use std::thread;

pub fn arc_example() {
    let data = Arc::new(vec![1, 2, 3]);

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                println!("Data: {:?}", data);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arc() {
        arc_example();
    }
}
