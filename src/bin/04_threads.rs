use std::thread;

// In total, we are going to have 3 threads running here, the main thread and 2 spawned threads.
fn main() {
    thread_lost();
    println!("--------------------");
    wait_for_thread();
    println!("--------------------");
}

fn thread_lost(){
    // spawn() creates a new thread and runs the code inside the closure
    // it also gives us a handle to the thread, which we can use to make the main thread wait for it to finish
    // but we don't use the handle here, so the main thread will not wait for the new thread to finish
    // the || is a closure, which is a function that can capture variables from its environment for the thread to use.
    thread::spawn(|| {
        println!("Thread 1!");
    });
    println!("Where did the thread go?");
}

fn wait_for_thread(){
    // here we use the handle to wait for the thread to finish before continuing
    let handle1 = thread::spawn(|| {
        println!("Thread 2!");
    });

    handle1.join().unwrap();
    println!("Oh it's here!");
}