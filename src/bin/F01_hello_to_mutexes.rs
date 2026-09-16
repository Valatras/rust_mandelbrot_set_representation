use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;

fn main(){
    println!("hello ! \nlet's do a dumb thread exercise with a vec! and a mutex");
    let counter = Arc::new(Mutex::new(String::from("shared data privatised by mister mustex")));
    let arrow_lengths = Arc::new (vec![20, 25, 30]);
    let mut handles = vec![];
    //thread here :

    println!("starting...");
    for i in 0..3 {
        let cloned = Arc::clone(&arrow_lengths);
        let keep_mutex_for_yourself = Arc::clone(&counter);
        let processlenghts = thread::spawn( move || {
        
            
            // To test if future threads keep working while the first waits :
            if i ==0 {
                sleep(Duration::new(5, 0));
            }
            println!("{cloned:?} ");

            println!("Hi ! I'm the thread number {i:?} and this is me accessing and writing my name in the mutex : ");
            // add your number to the mutexed string here...
            let mut get_access_to_the_mutex_string = keep_mutex_for_yourself.lock().unwrap();
            get_access_to_the_mutex_string.push_str(&format!(" {i} was here ! "));

            println!("{get_access_to_the_mutex_string}");
        }
        );
        handles.push(processlenghts);
    }


    println!("waiting for the threads to finish.");

    // to wait each thread before the main thread finishes
    for handle in handles{
        handle.join().unwrap();
    }
    
    println!("done.");


}