use std::sync::Mutex;
use std::sync::mpsc;
use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thred", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thred", i);
        thread::sleep(Duration::from_millis(1));
    }
    // handle.join().unwrap();

    let v = vec![1, 2, 3];

    let hand = thread::spawn(move || println!("{:?}", v));

    message_passing();
    sharing_state();
}

fn message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hi there"),
            String::from("Hi one"),
            String::from("Hi two"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // let reveived = rx.recv().unwrap();
    // println!("Got {}", reveived);

    for received in rx {
        println!("GOT {}", received);
    }
}

fn sharing_state() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("{:?}", m);
}
