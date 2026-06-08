# Chương 8: Lập Trình Đồng Thời (Concurrency)

## 8.1 Threads

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Xin chào từ thread con!");
    });
    handle.join().unwrap();
}
```

## 8.2 Channel (Message Passing)

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("Xin chào!").unwrap();
    });
    println!("Nhận: {}", rx.recv().unwrap());
}
```

## 8.3 Arc + Mutex (Shared State)

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            *c.lock().unwrap() += 1;
        }));
    }

    for h in handles { h.join().unwrap(); }
    println!("Counter: {}", *counter.lock().unwrap());
}
```

## 8.4 Async/Await (Giới thiệu)

```rust
// Cần tokio runtime
async fn tai_du_lieu(url: &str) -> String {
    format!("Data from {}", url)
}
```

## 8.5 Bài Tập

1. Map-Reduce song song
2. Producer-Consumer với channel
3. Web Crawler giả lập

---

📖 **Trước đó**: [Chương 7](../chuong-07-traits-lifetimes/README.md) | **Tiếp theo**: [Chương 9](../chuong-09-du-an-thuc-hanh/README.md)
