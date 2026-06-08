use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== THREADS ===\n");
    let mut handles = vec![];
    for id in 1..=3 {
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            println!("  Thread {} xong!", id);
            id * 10
        }));
    }
    let results: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    println!("  Kết quả: {:?}", results);

    println!("\n=== CHANNEL ===\n");
    let (tx, rx) = mpsc::channel();
    for i in 0..3 {
        let tx_c = tx.clone();
        thread::spawn(move || { tx_c.send(format!("Msg {}", i)).unwrap(); });
    }
    drop(tx);
    for msg in rx { println!("  📨 {}", msg); }

    println!("\n=== ARC + MUTEX ===\n");
    let counter = Arc::new(Mutex::new(0));
    let mut hs = vec![];
    for _ in 0..10 {
        let c = Arc::clone(&counter);
        hs.push(thread::spawn(move || { *c.lock().unwrap() += 1; }));
    }
    for h in hs { h.join().unwrap(); }
    println!("  Counter: {}", *counter.lock().unwrap());
}
