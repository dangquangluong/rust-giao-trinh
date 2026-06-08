# Chương 8: Concurrency (threading thành std::thread)

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
import threading                         # Import module threading

def chao(ten):                           # Hàm sẽ chạy trong thread
    print(f"Xin chào từ thread: {ten}")

t = threading.Thread(target=chao, args=("Python",))  # Tạo thread với hàm target
t.start()                                # Bắt đầu chạy thread
t.join()                                 # Đợi thread chạy xong (blocking)
```

### 🦀 Rust:
```rust
use std::thread;                                 // Import module thread (giống import threading)

fn main() {                                       // Hàm chính
    let handle = thread::spawn(|| {              // thread::spawn() = tạo thread mới
                                                  // || {} = closure (giống lambda Python)
                                                  // Giống threading.Thread(target=lambda: ...)
        println!("Xin chào từ thread con!");     // Code chạy trong thread mới
    });                                           // handle = JoinHandle (giống Thread object Python)

    println!("Thread chính vẫn chạy");           // Thread chính tiếp tục chạy song song
    handle.join().unwrap();                       // join() = đợi thread con xong (giống t.join() Python)
                                                  // unwrap() vì join() trả Result
}
```

---

## 8.3 Chia Sẻ Dữ Liệu Giữa Threads

### 🐍 Python (dễ nhưng dễ bug):
```python
import threading                         # Import threading
counter = 0                              # Biến global chia sẻ giữa threads
lock = threading.Lock()                  # Lock để tránh race condition

def tang():                              # Hàm chạy trong mỗi thread
    global counter                       # Dùng biến global
    for _ in range(100000):              # Lặp 100000 lần
        with lock:                       # Khóa trước khi sửa (chỉ 1 thread vào)
            counter += 1                 # Tăng counter (an toàn nhờ lock)

threads = [threading.Thread(target=tang) for _ in range(5)]  # Tạo 5 threads
for t in threads: t.start()              # Khởi động tất cả
for t in threads: t.join()               # Đợi tất cả xong
print(counter)                           # 500000 (đúng nhờ lock)
```

### 🦀 Rust (compiler BẮT BẠN dùng lock!):
```rust
use std::sync::{Arc, Mutex};                     // Arc = shared ownership, Mutex = lock
                                                  // Giống from threading import Lock
use std::thread;                                  // Import thread

fn main() {                                       // Hàm chính
    // Arc = Atomic Reference Count (chia sẻ ownership giữa threads)
    // Mutex = mutual exclusion = lock (chỉ 1 thread truy cập cùng lúc)
    let counter = Arc::new(Mutex::new(0));       // Tạo counter = 0, bọc trong Mutex + Arc
                                                  // Giống: counter = 0 + lock = Lock() Python
                                                  // Arc cho phép nhiều thread cùng "sở hữu"
    let mut handles = vec![];                    // Vec chứa thread handles (giống list threads Python)

    for _ in 0..5 {                              // Tạo 5 threads (giống range(5) Python)
        let counter = Arc::clone(&counter);      // Clone Arc (tăng reference count, không copy data)
                                                  // Mỗi thread cần "bản sao" Arc để truy cập counter
        let handle = thread::spawn(move || {     // move = thread lấy ownership biến counter (clone)
                                                  // || = closure, move = "chuyển biến vào closure"
            for _ in 0..100_000 {                // Lặp 100000 lần (100_000 = 100000, _ dễ đọc)
                let mut num = counter.lock().unwrap();  // lock() = with lock: Python (khóa Mutex)
                                                       // unwrap() vì lock() trả Result
                                                       // num = MutexGuard (tự unlock khi hết scope)
                *num += 1;                       // *num = truy cập giá trị bên trong Mutex
                                                  // * = dereference (lấy giá trị từ reference)
            }                                     // num hết scope -> tự động unlock! (RAII)
        });
        handles.push(handle);                    // Lưu handle (giống threads.append(t))
    }

    for h in handles {                           // Đợi tất cả threads xong
        h.join().unwrap();                       // join() = đợi thread xong (giống t.join())
    }
    println!("Counter: {}", *counter.lock().unwrap());  // In kết quả: 500000
                                                         // lock() + unwrap() + * để lấy giá trị
}
```

> 🤔 **Tại sao phức tạp hơn Python?** Vì Rust **đảm bảo** không có race condition tại compile time. Python cho phép bạn viết code race condition mà không cảnh báo!

---

## 8.4 Channel = Queue Python

### 🐍 Python:
```python
from queue import Queue                  # Import Queue (thread-safe)
from threading import Thread             # Import Thread

q = Queue()                              # Tạo queue để giao tiếp giữa threads

def producer():                          # Hàm producer (gửi message)
    for i in range(5):
        q.put(f"Message {i}")            # Đưa message vào queue

def consumer():                          # Hàm consumer (nhận message)
    while True:
        msg = q.get()                    # Lấy message từ queue (blocking)
        if msg is None: break            # None = tín hiệu dừng
        print(f"Nhận: {msg}")

Thread(target=producer).start()          # Chạy producer
Thread(target=consumer).start()          # Chạy consumer
```

### 🦀 Rust:
```rust
use std::sync::mpsc;                             // mpsc = multiple producer, single consumer
                                                  // Giống Queue Python nhưng 1 chiều
use std::thread;                                  // Import thread

fn main() {                                       // Hàm chính
    let (tx, rx) = mpsc::channel();              // Tạo channel: tx = sender, rx = receiver
                                                  // Giống: q = Queue() nhưng tách thành 2 đầu
                                                  // tx = "transmitter" (gửi), rx = "receiver" (nhận)

    // Producer (chạy trong thread riêng)
    thread::spawn(move || {                      // Tạo thread, move tx vào thread
        for i in 0..5 {                          // Gửi 5 messages
            tx.send(format!("Message {}", i)).unwrap();  // send() = q.put() Python
                                                          // format!() = f"Message {i}"
        }
    });                                           // tx bị drop khi thread xong -> channel đóng

    // Consumer (thread chính nhận)
    for msg in rx {                               // rx implement Iterator, lặp cho đến khi channel đóng
                                                  // Giống while msg := q.get(): nhưng tự dừng khi hết
        println!("Nhận: {}", msg);               // In message nhận được
    }
}
```

---

## 8.5 Async/Await (Giới Thiệu)

### 🐍 Python:
```python
import asyncio                           # Import asyncio

async def tai_du_lieu(url):              # async def = hàm bất đồng bộ
    await asyncio.sleep(1)               # await = đợi (nhường quyền cho task khác)
    return f"Data from {url}"            # Trả kết quả

async def main():                        # Hàm main async
    results = await asyncio.gather(      # gather = chạy đồng thời nhiều task
        tai_du_lieu("url1"),
        tai_du_lieu("url2"),
    )
    print(results)

asyncio.run(main())                      # Chạy event loop
```

### 🦀 Rust (cần thư viện tokio):
```rust
// Cargo.toml: tokio = { version = "1", features = ["full"] }
use tokio::time::{sleep, Duration};              // Import sleep và Duration từ tokio
                                                  // Giống from asyncio import sleep

async fn tai_du_lieu(url: &str) -> String {      // async fn = async def Python
                                                  // -> String = kiểu trả về
    sleep(Duration::from_secs(1)).await;         // .await = await Python (đợi 1 giây)
                                                  // Duration::from_secs(1) = tạo duration 1s
    format!("Data from {}", url)                 // Trả về String (giống f"Data from {url}")
}

#[tokio::main]                                    // Macro biến main thành async
                                                  // Giống asyncio.run(main()) Python
async fn main() {                                 // async main
    let (r1, r2) = tokio::join!(                 // join! = asyncio.gather() (chạy đồng thời)
        tai_du_lieu("url1"),                     // Task 1
        tai_du_lieu("url2"),                     // Task 2
    );                                            // Destructure kết quả vào r1, r2
    println!("{}, {}", r1, r2);                  // In kết quả cả hai task
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

1. Tạo 5 threads, mỗi thread tính tổng 1 phần của Vec, rồi gộp kết quả
2. Producer-Consumer: 3 producer gửi số, 1 consumer tính tổng
3. (Nâng cao) Viết async web fetcher giả lập

---

📖 **Trước đó**: [Chương 7](../chuong-07-traits-lifetimes/README.md) | **Tiếp theo**: [Chương 9](../chuong-09-du-an-thuc-hanh/README.md)
