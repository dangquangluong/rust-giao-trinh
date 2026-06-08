// === PYTHON EQUIVALENT ===
// # Python:
// # import threading
// # from queue import Queue
// #
// # # Threads
// # def worker(id):
// #     import time; time.sleep(0.05)
// #     print(f"Thread {id} xong!")
// #     return id * 10
// #
// # threads = [threading.Thread(target=worker, args=(i,)) for i in range(1,4)]
// # for t in threads: t.start()
// # for t in threads: t.join()
// #
// # # Queue (channel)
// # q = Queue()
// # def producer(i): q.put(f"Msg {i}")
// # for i in range(3): threading.Thread(target=producer, args=(i,)).start()
// # while not q.empty(): print(f"📨 {q.get()}")
// #
// # # Lock (mutex)
// # counter = 0; lock = threading.Lock()
// # def inc():
// #     global counter
// #     with lock: counter += 1
// # threads = [threading.Thread(target=inc) for _ in range(10)]
// # for t in threads: t.start()
// # for t in threads: t.join()
// # print(f"Counter: {counter}")

// ============================================
// Concurrency trong Rust
// Rust threads chạy SONG SONG THẬT (Python bị GIL!)
// Arc = shared pointer giữa threads
// Mutex = lock (giống threading.Lock() Python)
// mpsc::channel = giống Queue Python
// ============================================

// use = giống import Python
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    // =========================================
    // 1. BASIC THREADS (giống threading.Thread Python)
    // =========================================
    println!("=== THREADS ===\n");

    // vec![] = list rỗng để chứa thread handles
    let mut handles = vec![];

    // 1..=3 = range(1, 4) Python
    for id in 1..=3 {
        // thread::spawn() = threading.Thread(target=...).start()
        // move || { } = closure lấy ownership biến (giống lambda Python nhưng mạnh hơn)
        // move = "di chuyển biến id vào thread" (vì thread khác không share được bình thường)
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));  // time.sleep(0.05)
            println!("  Thread {} xong!", id);
            id * 10  // Trả về giá trị (thread trong Python không làm được dễ dàng!)
        }));
    }

    // .join().unwrap() = t.join() Python (đợi thread xong + lấy kết quả)
    // .into_iter() = lấy ownership từ vector
    // .map() + .collect() = giống list comprehension
    let results: Vec<i32> = handles.into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    println!("  Kết quả: {:?}", results);  // [10, 20, 30]

    // =========================================
    // 2. CHANNEL (giống Queue Python)
    // =========================================
    println!("\n=== CHANNEL (giống Queue Python) ===\n");

    // mpsc::channel() = tạo (sender, receiver) (giống Queue nhưng tách 2 đầu)
    // tx = transmitter (gửi), rx = receiver (nhận)
    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        let tx_c = tx.clone();  // Clone sender để mỗi thread có 1 bản
        thread::spawn(move || {
            // .send() = giống q.put() Python
            tx_c.send(format!("Msg {}", i)).unwrap();
        });
    }

    // drop(tx) = xóa sender gốc (để rx biết khi nào hết message)
    // Giống đóng Queue khi producer xong
    drop(tx);

    // for msg in rx = giống while not q.empty(): msg = q.get()
    // Tự dừng khi tất cả sender bị drop
    for msg in rx {
        println!("  📨 {}", msg);
    }

    // =========================================
    // 3. ARC + MUTEX (giống threading.Lock Python)
    // =========================================
    println!("\n=== ARC + MUTEX (giống Lock Python) ===\n");

    // Arc = Atomic Reference Count = "shared pointer an toàn giữa threads"
    //   Giống: biến global mà nhiều thread cùng truy cập
    // Mutex = Mutual Exclusion = "khóa" (giống threading.Lock())
    //   Chỉ 1 thread truy cập tại 1 thời điểm
    // Arc<Mutex<i32>> = "số nguyên được bảo vệ, share giữa threads"
    let counter = Arc::new(Mutex::new(0));

    let mut hs = vec![];
    for _ in 0..10 {
        // Arc::clone() = tạo thêm 1 "pointer" cùng trỏ đến data
        // KHÔNG copy data, chỉ tăng reference count
        let c = Arc::clone(&counter);

        hs.push(thread::spawn(move || {
            // .lock().unwrap() = giống "with lock:" Python
            // Lấy quyền truy cập exclusive
            // * = dereference (lấy giá trị bên trong để sửa)
            *c.lock().unwrap() += 1;
            // Lock tự giải phóng khi ra khỏi scope (giống with: tự __exit__)
        }));
    }

    // Đợi tất cả threads xong
    for h in hs {
        h.join().unwrap();
    }

    // Kết quả: 10 (mỗi thread tăng 1, tổng 10)
    println!("  Counter: {}", *counter.lock().unwrap());
}
