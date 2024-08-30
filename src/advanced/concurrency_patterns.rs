use std::sync::mpsc::{channel, sync_channel};
use std::sync::{Arc, Mutex};
use std::thread;

// Producer-Consumer using Mutex and Arc
pub fn producer_consumer_example() {
    let data = Arc::new(Mutex::new(vec![]));
    let (tx, rx) = channel();
    let data_clone = Arc::clone(&data);

    let producer = thread::spawn(move || {
        for i in 0..5 {
            let mut data = data_clone.lock().unwrap();
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
}

// Worker Pool example
pub fn worker_pool_example() {
    // Create a synchronous channel with capacity 1.
    let (tx, rx) = sync_channel(1);

    let mut handles = vec![];

    // Create a channel for each worker
    let mut worker_txs = vec![];
    for _ in 0..4 {
        let (worker_tx, worker_rx) = channel();
        worker_txs.push(worker_tx);

        // Each worker thread receives jobs from its own channel
        let handle = thread::spawn(move || {
            while let Ok(job) = worker_rx.recv() {
                println!("Worker received job: {}", job);
            }
        });

        handles.push(handle);
    }

    // Dispatcher thread that distributes jobs to workers
    let dispatcher = thread::spawn(move || {
        let mut i = 0;
        while let Ok(job) = rx.recv() {
            // Distribute the job to the workers in a round-robin fashion
            worker_txs[i].send(job).unwrap();
            i = (i + 1) % worker_txs.len();
        }
    });

    // Sending jobs to the channel
    for job in 1..=8 {
        // Send jobs to the dispatcher
        tx.send(job).unwrap();
    }

    // Close the channel, signaling no more jobs
    drop(tx);

    // Wait for the dispatcher to finish
    dispatcher.join().unwrap();

    // Wait for all worker threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_producer_consumer() {
        producer_consumer_example();
    }

    #[test]
    fn test_worker_pool() {
        worker_pool_example();
    }
}
