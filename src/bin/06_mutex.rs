use std::sync::{Arc, Mutex, RwLock};
// Mutex allows only one thread to access the data at a time, whether reading or writing.
// RwLock allows multiple threads to read simultaneously, but only one thread to write at a time.
use std::thread;

fn main() {
    arc_mutex_counter();
    println!("-----------------------");
    arc_rwlock_counter();
}

fn arc_mutex_counter() {
    // Arc allows multiple threads to share ownership of the same value.
    // Mutex ensures that only one thread can access the value at a time.
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for i in 0..3 {
        // Clone the Arc, not the counter itself.
        // All clones point to the same Mutex and therefore the same integer.
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // lock() gives us access to the value protected by the Mutex.
            // Only one thread can hold the lock at a time.
            let mut value = counter_clone.lock().unwrap();

            // We can safely modify the shared value.
            *value += 1;

            println!("Thread {}: counter = {}", i, value);

            // The lock is automatically released when `value` goes out of scope.
        });

        handles.push(handle);
    }

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }

    // The main thread can access the same shared value.
    let value = counter.lock().unwrap();

    println!("Main: counter = {}", value);
}


fn arc_rwlock_counter() {
    // Arc allows multiple threads to own the RwLock.
    // RwLock allows multiple readers OR one writer at a time.
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    let mut handles = vec![];

    // Spawn multiple reader threads.
    for i in 0..3 {
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            // read() gives shared read-only access.
            // Multiple threads can hold a read lock at the same time.
            let values = data_clone.read().unwrap();

            println!("Reader {}: {:?}", i, values);
        });

        handles.push(handle);
    }

    // Spawn one writer thread.
    let data_clone = Arc::clone(&data);

    let handle = thread::spawn(move || {
        // write() gives exclusive mutable access.
        // All readers and other writers must wait while we hold this lock.
        let mut values = data_clone.write().unwrap();

        values.push(4);

        println!("Writer: {:?}", values);
    });

    handles.push(handle);

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }

    // Read the final value from the main thread.
    let values = data.read().unwrap();

    println!("Main: {:?}", values);
}