use async_std::sync::{channel, Receiver, Sender};
use async_std::task;

fn main() {
    let (tx1, rx1): (Sender<u64>, Receiver<u64>) = channel(64);
    let h = task::spawn(async move {
        let mut r: u64 = 0;
        while let Some(x) = rx1.recv().await {
            r ^= x + 1;
        }
        r
    });
    let (tx2, rx2): (Sender<u64>, Receiver<u64>) = channel(64);
    task::spawn(async move {
        while let Some(x) = rx2.recv().await {
            tx1.send(x - 2).await;
        }
    });

    task::spawn(async move {
        for i in 0..100_000_000_u64 {
            tx2.send(i).await;
        }
    });
    println!("{}", task::block_on(h));
}
