# Chương 9: Dự Án Thực Hành

> 🎯 Bạn đã biết viết project Python? Giờ làm lại bằng Rust!

## Dự Án 1: Todo CLI ⭐⭐ (Giống Python todo bạn đã biết)

### Python version (quen thuộc):
```python
import json, sys                         # Import json (đọc/ghi file) và sys (args)

def main():                              # Hàm chính
    todos = load()                       # Đọc todos từ file
    if sys.argv[1] == "add":             # Nếu lệnh là "add"
        todos.append({"text": sys.argv[2], "done": False})  # Thêm todo mới
    elif sys.argv[1] == "list":          # Nếu lệnh là "list"
        for t in todos:                  # Duyệt qua từng todo
            print(f"{'✅' if t['done'] else '⬜'} {t['text']}")  # In với icon
    save(todos)                          # Lưu lại file
```

### Rust version (dùng kiến thức đã học):
```rust
use std::fs;                                     // Import fs = file system (đọc/ghi file)
                                                  // Giống import os / pathlib Python
use serde::{Deserialize, Serialize};             // Import serde traits để JSON serialize/deserialize
                                                  // Giống json module Python nhưng mạnh hơn

#[derive(Serialize, Deserialize, Debug)]         // Tự động thêm khả năng chuyển struct <-> JSON
                                                  // Serialize = struct -> JSON, Deserialize = JSON -> struct
struct Task {                                     // Struct Task (giống dict {"text": ..., "done": ...})
    text: String,                                // Nội dung todo
    done: bool,                                  // Đã xong chưa
}

fn load() -> Vec<Task> {                         // Hàm load: trả Vec<Task> (giống list[dict] Python)
    fs::read_to_string("todos.json")             // Đọc file thành String (giống open().read())
        .ok()                                    // Result -> Option (bỏ qua lỗi cụ thể)
        .and_then(|s| serde_json::from_str(&s).ok())  // Parse JSON thành Vec<Task>
                                                       // |s| = closure, giống lambda s: json.loads(s)
        .unwrap_or_default()                     // Nếu lỗi (file chưa có) thì trả Vec rỗng
                                                  // Giống: todos = json.loads(...) or []
}

fn save(tasks: &[Task]) {                        // Hàm save: nhận slice tasks (mượn, không lấy ownership)
                                                  // &[Task] = reference đến array (giống list[Task] Python)
    let json = serde_json::to_string_pretty(tasks).unwrap();  // Chuyển thành JSON đẹp
                                                               // Giống json.dumps(tasks, indent=2)
    fs::write("todos.json", json).unwrap();      // Ghi file (giống open().write())
}

fn main() {                                       // Hàm chính (chạy đầu tiên)
    let args: Vec<String> = std::env::args().collect();  // Lấy command line args
                                                          // Giống sys.argv Python
                                                          // collect() = gom iterator thành Vec
    let mut tasks = load();                      // Load tasks từ file (mut vì sẽ thay đổi)

    match args.get(1).map(|s| s.as_str()) {     // Lấy arg thứ 1, chuyển thành &str để match
                                                  // get(1) trả Option (an toàn hơn args[1])
                                                  // Giống: command = sys.argv[1] if len(sys.argv) > 1
        Some("add") => {                         // Nếu command == "add"
            let text = args[2..].join(" ");      // Nối các args từ vị trí 2 (giống " ".join(sys.argv[2:]))
            tasks.push(Task { text, done: false });  // Thêm Task mới vào Vec
                                                      // Giống todos.append({"text": text, "done": False})
            println!("✅ Đã thêm!");             // Thông báo
        }
        Some("list") => {                        // Nếu command == "list"
            for (i, t) in tasks.iter().enumerate() {  // Duyệt với index (giống enumerate())
                let icon = if t.done { "✅" } else { "⬜" };  // Chọn icon theo trạng thái
                println!("{} [{}] {}", icon, i, t.text);       // In todo
            }
        }
        Some("done") => {                        // Nếu command == "done"
            let idx: usize = args[2].parse().unwrap();  // Parse index từ string (giống int(sys.argv[2]))
            tasks[idx].done = true;              // Đánh dấu hoàn thành
            println!("✅ Hoàn thành!");          // Thông báo
        }
        _ => println!("Dùng: todo [add|list|done] [args]"),  // Default: in hướng dẫn
    }

    save(&tasks);                                // Lưu lại file (&tasks = cho mượn để ghi)
}
```

**Kiến thức dùng:** Struct, Enum (match), Vec, File I/O, Result, serde

---

## Dự Án 2: Grep Clone ⭐⭐ (Giống `grep` trên Linux)

```bash
cargo run -- "pattern" file.txt
```

```rust
use std::{env, fs};                              // Import env (args) và fs (file)
                                                  // Giống import sys, os Python

fn main() {                                       // Hàm chính
    let args: Vec<String> = env::args().collect(); // Lấy args (giống sys.argv)

    if args.len() < 3 {                          // Kiểm tra đủ args không (giống len(sys.argv) < 3)
        eprintln!("Dùng: minigrep <pattern> <file>");  // eprintln! = print to stderr
                                                        // Giống print(..., file=sys.stderr)
        std::process::exit(1);                   // Thoát với code 1 (giống sys.exit(1))
    }

    let pattern = &args[1];                      // Lấy pattern (giống pattern = sys.argv[1])
                                                  // & = mượn (không cần clone)
    let file = &args[2];                         // Lấy tên file (giống file = sys.argv[2])

    let content = fs::read_to_string(file)       // Đọc toàn bộ file thành String
        .expect("Không thể đọc file");           // Nếu lỗi thì crash với message
                                                  // Giống open(file).read() nhưng có error handling

    for (i, line) in content.lines().enumerate() {  // lines() = splitlines() Python
                                                     // enumerate() = thêm index
        if line.contains(pattern.as_str()) {     // contains() = "in" Python (kiểm tra substring)
                                                  // Giống: if pattern in line
            println!("{}:{}", i + 1, line);      // In: số_dòng:nội_dung (i+1 vì đếm từ 1)
        }
    }
}
```

**Kiến thức dùng:** String slice, Iterator, File I/O, Error handling

---

## Dự Án 3: Số Liệu Thống Kê ⭐ (Dễ nhất - bắt đầu từ đây!)

```rust
fn main() {                                       // Hàm chính
    let data = vec![85.0, 90.0, 78.0, 92.0, 88.0, 95.0, 67.0, 73.0];
                                                  // vec![] = tạo Vec<f64> (giống [85.0, 90.0, ...] Python)

    let n = data.len() as f64;                   // len() = len(), as f64 = float() để chia
    let sum: f64 = data.iter().sum();            // iter().sum() = sum(data) Python
    let mean = sum / n;                          // Trung bình = tổng / số lượng
    let max = data.iter().cloned().fold(f64::MIN, f64::max);
                                                  // fold = reduce Python, tìm giá trị lớn nhất
                                                  // cloned() = copy giá trị ra (vì iter cho reference)
                                                  // f64::MIN = giá trị khởi đầu nhỏ nhất
    let min = data.iter().cloned().fold(f64::MAX, f64::min);
                                                  // Tương tự, tìm giá trị nhỏ nhất

    // Median (trung vị)
    let mut sorted = data.clone();               // Clone data để sort (sort sửa trực tiếp)
                                                  // Giống sorted_data = data.copy() Python
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                                                  // sort_by = sorted() với key
                                                  // partial_cmp = so sánh float (float có NaN nên dùng partial)
    let median = if sorted.len() % 2 == 0 {     // Nếu số phần tử chẵn
        (sorted[sorted.len()/2 - 1] + sorted[sorted.len()/2]) / 2.0
                                                  // Trung bình 2 phần tử giữa
    } else {
        sorted[sorted.len()/2]                   // Phần tử chính giữa
    };

    println!("📊 Thống kê:");                    // In tiêu đề
    println!("  Số lượng: {}", data.len());      // len() = len() Python
    println!("  Tổng: {:.1}", sum);              // {:.1} = 1 chữ số thập phân
    println!("  Trung bình: {:.2}", mean);       // {:.2} = 2 chữ số thập phân
    println!("  Trung vị: {:.1}", median);       // In trung vị
    println!("  Max: {:.1}, Min: {:.1}", max, min);  // In max và min
}
```

---

## Thứ Tự Làm (Từ Dễ tới Khó)

| # | Dự Án | Kiến thức cần | Độ khó |
|---|--------|--------------|--------|
| 1 | Thống kê | Vec, Iterator | ⭐ |
| 2 | Grep clone | File I/O, String | ⭐⭐ |
| 3 | Todo CLI | Struct, serde, match | ⭐⭐ |
| 4 | HTTP client | reqwest (async) | ⭐⭐⭐ |
| 5 | Chat server | Thread, Channel | ⭐⭐⭐ |

---

## 🐍 thành 🦀 Bạn Đã Đi Được Bao Xa?

```
Python:  print("Hello")
    ↓
Rust:    println!("Hello");         ← Chương 1 ✅
    ↓
Rust:    let mut x = 5;             ← Chương 2 ✅
    ↓
Rust:    fn foo(s: &str) {}         ← Chương 3 ✅ (Ownership!)
    ↓
Rust:    struct + impl + enum       ← Chương 4 ✅
    ↓
Rust:    Result + ?                 ← Chương 5 ✅
    ↓
Rust:    Vec + HashMap + iter       ← Chương 6 ✅
    ↓
Rust:    trait + generics           ← Chương 7 ✅
    ↓
Rust:    thread + async             ← Chương 8 ✅
    ↓
Rust:    DỰ ÁN THỰC TẾ            ← BẠN ĐANG Ở ĐÂY! 🎉
```

---

🎉 **Chúc mừng! Bạn đã hoàn thành hành trình từ Python sang Rust!**

📖 **Trước đó**: [Chương 8](../chuong-08-concurrency/README.md)
