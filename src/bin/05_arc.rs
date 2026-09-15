use std::rc::Rc;
// Rc stands for Reference Counted, which is a smart pointer that keeps track of the number of references to a value.
// We'll use it here for an example without Arc
use std::sync::Arc;
use std::thread;
// Arc stands for Atomic Reference Counted, which is a thread-safe version of Rc. 
// allows multiple threads to share ownership of a value, and it keeps track of the number of references to that value in a thread-safe way.


fn main() {
    rc_ownership_sharing();
    println!("--------------------");
    threading_with_arc_ownership();
    println!("--------------------");
}

// //without Arc we can use Rc:
fn rc_ownership_sharing() {
    // shared ownership of a vector using Rc. Both `data` and `clone` point to the same vector, and the reference count is 2.
    let data = Rc::new(vec![1, 2, 3]);
    // it's not Vec that we clone but it's just a second Rc that points to the same Vec.
    let clone = Rc::clone(&data);
    
    // strong_count() returns the number of strong references to the value, which is 2 in this case.
    println!("Compteur Rc: {}", Rc::strong_count(&data)); // 2
    println!("Data: {:?}", clone);

    // Drop disposes of a value, here it drops the Rc inside clone. Not the Vec since data still points at it.
    drop(clone);
    println!("Compteur Rc après drop de clone: {}", Rc::strong_count(&data)); // 1
    // the issue with Rc is that it is not thread-safe, so if we try to use it in a multi-threaded context, we will get a compile-time error.
}   




// // not working version with Rc and threads, because Rc is not thread-safe:
// fn main() {
//     // we put data in an Rc, which is a reference-counted smart pointer that allows multiple ownership of the same data.
//     let data = Rc::new(vec![1, 2, 3]);
//     let mut handles = vec![];

//     for i in 0..3 {
//         // here we clone the Rc, which increases the reference count, and move it into the thread.
//         let data_clone = Rc::clone(&data);
//         // but by writing "move" here, we are trying to move the Rc into the thread, which is not allowed because Rc is not thread-safe.
//         // As here we have multiple threads (3) trying to access the same Rc, it will cause a compile-time error.
//         let handle = thread::spawn(move || {
//             println!("Thread {} voit: {:?}", i, data_clone);
//         });
//         handles.push(handle);
//     }
// }




//with Arc :
fn threading_with_arc_ownership() {
    // we do the same thing here with Arc, so we're adding data to a counter again but this time it is thread-safe.
    let data = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];

    for i in 0..3 {
        // We clone the Arc, which increases the reference count, and move it into the thread.
        let data_clone = Arc::clone(&data);
        // This time "move" doesn't give us an error because Arc is thread-safe, so we can safely move it into the thread.
        // it works as follows : each Arc::clone
        let handle = thread::spawn(move || {
            println!("Thread {} voit: {:?}", i, data_clone);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // The main thread can still access the vector, the reference count is 1, and the vector is still valid.
    println!("Main voit: {:?}", data);
    // When data goes out of scope, the reference count will be 0, and the vector will be dropped.
}