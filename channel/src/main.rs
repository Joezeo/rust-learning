use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = channel();

    let join_handler = thread::spawn(move || {
        while let Ok(n) = rx.recv() {
            println!("Received {}", n);
        }
    });

    for i in 0..100 {
        tx.send(i).unwrap();
    }

    thread::sleep(Duration::from_secs(10));

    drop(tx);

    join_handler.join().unwrap();
}
