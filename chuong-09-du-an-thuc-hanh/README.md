# Chương 9: Dự Án Thực Hành

> 🎯 Bạn đã biết viết project Python? Giờ làm lại bằng Rust!

## Dự Án 1: Todo CLI ⭐⭐ (Giống Python todo bạn đã biết)

### Python version (quen thuộc):
```python
import json, sys

def main():
    todos = load()
    if sys.argv[1] == "add":
        todos.append({"text": sys.argv[2], "done": False})
    elif sys.argv[1] == "list":
        for t in todos:
            print(f"{'✅' if t['done'] else '⬜'} {t['text']}")
    save(todos)
```

### Rust version (dùng kiến thức đã học):
```rust
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    text: String,
    done: bool,
}

fn load() -> Vec<Task> {
    fs::read_to_string("todos.json")
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save(tasks: &[Task]) {
    let json = serde_json::to_string_pretty(tasks).unwrap();
    fs::write("todos.json", json).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut tasks = load();

    match args.get(1).map(|s| s.as_str()) {
        Some("add") => {
            let text = args[2..].join(" ");
            tasks.push(Task { text, done: false });
            println!("✅ Đã thêm!");
        }
        Some("list") => {
            for (i, t) in tasks.iter().enumerate() {
                let icon = if t.done { "✅" } else { "⬜" };
                println!("{} [{}] {}", icon, i, t.text);
            }
        }
        Some("done") => {
            let idx: usize = args[2].parse().unwrap();
            tasks[idx].done = true;
            println!("✅ Hoàn thành!");
        }
        _ => println!("Dùng: todo [add|list|done] [args]"),
    }

    save(&tasks);
}
```

**Kiến thức dùng:** Struct, Enum (match), Vec, File I/O, Result, serde

---

## Dự Án 2: Grep Clone ⭐⭐ (Giống `grep` trên Linux)

```bash
cargo run -- "pattern" file.txt
```

```rust
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Dùng: minigrep <pattern> <file>");
        std::process::exit(1);
    }

    let pattern = &args[1];
    let file = &args[2];

    let content = fs::read_to_string(file)
        .expect("Không thể đọc file");

    for (i, line) in content.lines().enumerate() {
        if line.contains(pattern.as_str()) {
            println!("{}:{}", i + 1, line);
        }
    }
}
```

**Kiến thức dùng:** String slice, Iterator, File I/O, Error handling

---

## Dự Án 3: Số Liệu Thống Kê ⭐ (Dễ nhất - bắt đầu từ đây!)

```rust
fn main() {
    let data = vec![85.0, 90.0, 78.0, 92.0, 88.0, 95.0, 67.0, 73.0];

    let n = data.len() as f64;
    let sum: f64 = data.iter().sum();
    let mean = sum / n;
    let max = data.iter().cloned().fold(f64::MIN, f64::max);
    let min = data.iter().cloned().fold(f64::MAX, f64::min);

    // Median
    let mut sorted = data.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = if sorted.len() % 2 == 0 {
        (sorted[sorted.len()/2 - 1] + sorted[sorted.len()/2]) / 2.0
    } else {
        sorted[sorted.len()/2]
    };

    println!("📊 Thống kê:");
    println!("  Số lượng: {}", data.len());
    println!("  Tổng: {:.1}", sum);
    println!("  Trung bình: {:.2}", mean);
    println!("  Trung vị: {:.1}", median);
    println!("  Max: {:.1}, Min: {:.1}", max, min);
}
```

---

## Thứ Tự Làm (Từ Dễ → Khó)

| # | Dự Án | Kiến thức cần | Độ khó |
|---|--------|--------------|--------|
| 1 | Thống kê | Vec, Iterator | ⭐ |
| 2 | Grep clone | File I/O, String | ⭐⭐ |
| 3 | Todo CLI | Struct, serde, match | ⭐⭐ |
| 4 | HTTP client | reqwest (async) | ⭐⭐⭐ |
| 5 | Chat server | Thread, Channel | ⭐⭐⭐ |

---

## 🐍→🦀 Bạn Đã Đi Được Bao Xa?

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
