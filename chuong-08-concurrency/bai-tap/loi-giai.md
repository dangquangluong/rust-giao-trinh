# Lời Giải Chương 8

## Bài 1: Map-Reduce

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let chunk_size = 3;                           // Chia thành phần 3 phần tử
    let mut handles = vec![];

    for chunk in data.chunks(chunk_size) {        // .chunks() chia vector
        let chunk = chunk.to_vec();               // Clone để move vào thread
        handles.push(thread::spawn(move || {      // Tạo thread
            let sum: i32 = chunk.iter().sum();    // Tính tổng phần nhỏ
            println!("  {:?} → {}", chunk, sum);
            sum                                    // Trả về tổng
        }));
    }

    let total: i32 = handles.into_iter()
        .map(|h| h.join().unwrap())               // Lấy kết quả mỗi thread
        .sum();                                    // Gộp tổng
    println!("Tổng cuối: {}", total);             // 78
}
```

## Bài 2: Producer-Consumer

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();               // Tạo channel

    for id in 0..3 {                              // 3 producers
        let tx = tx.clone();                      // Clone sender
        thread::spawn(move || {
            for i in 1..=5 {                      // Mỗi producer gửi 5 số
                tx.send(id * 10 + i).unwrap();    // Gửi số
            }
        });
    }
    drop(tx);                                     // Drop sender gốc

    let tong: i32 = rx.iter().sum();              // Consumer: nhận tất cả + tính tổng
    println!("Tổng: {}", tong);                   // 3 producers × (10+11+12+13+14+15) = ...
}
```

## Bài 3: Shared Vec

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let names = Arc::new(Mutex::new(Vec::new())); // Vec được share
    let mut handles = vec![];

    for id in 0..5 {
        let names = Arc::clone(&names);           // Clone Arc
        handles.push(thread::spawn(move || {
            let mut vec = names.lock().unwrap();   // Khóa Mutex
            vec.push(format!("Thread-{}", id));    // Thêm tên
        }));
    }

    for h in handles { h.join().unwrap(); }
    println!("Kết quả: {:?}", *names.lock().unwrap());
}
```
