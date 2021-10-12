use std::sync::mpsc;
use std::thread;

fn main() {
    // tx -> transmitter
    // rx -> receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
