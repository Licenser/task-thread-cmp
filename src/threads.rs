use crossbeam_channel::{bounded, Receiver, Sender};
use std::thread;

fn main() {
    let (tx1, rx1): (Sender<u64>, Receiver<u64>) = bounded(64);
    let h = thread::spawn(move || {
        let mut r: u64 = 0;
        for x in rx1 {
            r ^= x + 1;
        }
        r
    });
    let (tx2, rx2): (Sender<u64>, Receiver<u64>) = bounded(64);
    thread::spawn(move || {
        for x in rx2 {
            tx1.send(x - 2);
        }
    });

    thread::spawn(move || {
        for i in 0..100_000_000_u64 {
            tx2.send(i);
        }
    });
    println!("{:?}", h.join());
}
