use std::{sync::mpsc, thread, time::Duration};

fn main() {
    // closures_with_thread();
    // spawn_thread();
    // message_passing();
    // message_passing_multiple_values();
    message_passing_multiple_producers();
}

#[allow(dead_code)]
fn closures_with_thread() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

#[allow(dead_code)]
fn spawn_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

#[allow(dead_code)]
fn message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("hi");
        tx.send(msg).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("got {received}");
}

#[allow(dead_code)]
fn message_passing_multiple_values() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("got {received}");
    }
}

fn message_passing_multiple_producers() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("got {received}");
    }
}
