use std::sync::{Arc, Mutex};
use std::thread;

#[allow(dead_code)]
fn sample_mutex() {
    // The mutex (In fact, the term mutex is short for
    // mutual exclusion) also known as spinlock
    // is the simplest synchronization tool that is used
    // to protect critical regions and thus prevent race conditions.
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("result: {}", *counter.lock().unwrap());
}
