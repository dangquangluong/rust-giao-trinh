# Chương 8: Concurrency (threading → std::thread)

> 🐍 Python có GIL nên threading không chạy song song thật sự. Rust KHÔNG có GIL!

## 8.1 So Sánh Tổng Quan

| | Python 🐍 | Rust 🦀 |
|-|-----------|---------|
| Threading | `threading` (bị GIL) | `std::thread` (song song thật!) |
| Multi-process | `multiprocessing` | Không cần (thread đã song song) |
| Async | `asyncio` | `tokio` |
| An toàn | Dễ bị race condition | Compiler **ngăn** race condition |
| Tốc độ | Chậm do GIL | Nhanh (native threads) |

---

## 8.2 Tạo Thread

### 🐍 Python:
```python
import threading

def chao(ten):
    print(f"Xin chào từ thread: {ten}")

t = threading.Thread(target=chao, args=("Python",))
t.start()
t.join()
```

### 🦀 Rust:
```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Xin chào từ thread con!");
    });

    println!("Thread chính vẫn chạy");
    handle.join().unwrap();  // Đợi thread con xong
}
```

---

## 8.3 Chia Sẻ Dữ Liệu Giữa Threads

### 🐍 Python (dễ nhưng dễ bug):
```python
import threading
counter = 0
lock = threading.Lock()

def tang():
    global counter
    for _ in range(100000):
        with lock:
            counter += 1

threads = [threading.Thread(target=tang) for _ in range(5)]
for t in threads: t.start()
for t in threads: t.join()
print(counter)  # 500000 (nhờ lock)
```

### 🦀 Rust (compiler BẮT BẠN dùng lock!):
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc = Atomic Reference Count (share giữa threads)
    // Mutex = lock (chỉ 1 thread truy cập lúc)
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100_000 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for h in handles { h.join().unwrap(); }
    println!("Counter: {}", *counter.lock().unwrap()); // 500000
}
```

> 🤔 **Tại sao phức tạp hơn Python?** Vì Rust **đảm bảo** không có race condition tại compile time. Python cho phép bạn viết code race condition mà không cảnh báo!

---

## 8.4 Channel = Queue Python

### 🐍 Python:
```python
from queue import Queue
from threading import Thread

q = Queue()

def producer():
    for i in range(5):
        q.put(f"Message {i}")

def consumer():
    while True:
        msg = q.get()
        if msg is None: break
        print(f"Nhận: {msg}")

Thread(target=producer).start()
Thread(target=consumer).start()
```

### 🦀 Rust:
```rust
use std::sync::mpsc;  // multiple producer, single consumer
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Producer
    thread::spawn(move || {
        for i in 0..5 {
            tx.send(format!("Message {}", i)).unwrap();
        }
    });

    // Consumer (thread chính)
    for msg in rx {
        println!("Nhận: {}", msg);
    }
}
```

---

## 8.5 Async/Await (Giới Thiệu)

### 🐍 Python:
```python
import asyncio

async def tai_du_lieu(url):
    await asyncio.sleep(1)  # Giả lập I/O
    return f"Data from {url}"

async def main():
    results = await asyncio.gather(
        tai_du_lieu("url1"),
        tai_du_lieu("url2"),
    )
    print(results)

asyncio.run(main())
```

### 🦀 Rust (cần thư viện tokio):
```rust
// Cargo.toml: tokio = { version = "1", features = ["full"] }
use tokio::time::{sleep, Duration};

async fn tai_du_lieu(url: &str) -> String {
    sleep(Duration::from_secs(1)).await;
    format!("Data from {}", url)
}

#[tokio::main]
async fn main() {
    let (r1, r2) = tokio::join!(
        tai_du_lieu("url1"),
        tai_du_lieu("url2"),
    );
    println!("{}, {}", r1, r2);
}
```

---

## 8.6 Khi Nào Dùng Gì?

| Tình huống | Python | Rust |
|------------|--------|------|
| Tính toán nặng (CPU) | `multiprocessing` | `std::thread` |
| I/O (web, file) | `asyncio` | `tokio` |
| Share data | `Lock` | `Arc<Mutex<T>>` |
| Gửi message | `Queue` | `mpsc::channel` |

---

## 8.7 Bài Tập

1. Tạo 5 threads, mỗi thread tính tổng 1 phần của Vec → gộp kết quả
2. Producer-Consumer: 3 producer gửi số, 1 consumer tính tổng
3. (Nâng cao) Viết async web fetcher giả lập

---

📖 **Trước đó**: [Chương 7](../chuong-07-traits-lifetimes/README.md) | **Tiếp theo**: [Chương 9](../chuong-09-du-an-thuc-hanh/README.md)
