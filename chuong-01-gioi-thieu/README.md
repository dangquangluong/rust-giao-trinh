# Chương 1: Giới Thiệu Rust & Cài Đặt

> 🐍 Bạn đã biết Python? Tuyệt! Mình sẽ giải thích Rust bằng cách so sánh với Python.

## 1.1 Rust Là Gì? Tại Sao Học?

Bạn đã biết Python - ngôn ngữ dễ viết, dễ đọc. Vậy tại sao cần Rust?

| | Python 🐍 | Rust 🦀 |
|-|-----------|---------|
| **Viết code** | Nhanh, ngắn gọn | Dài hơn, nhưng an toàn hơn |
| **Chạy** | Chậm (100x chậm hơn C) | Cực nhanh (ngang C/C++) |
| **Lỗi** | Phát hiện lúc chạy (crash!) | Phát hiện lúc compile (trước khi chạy!) |
| **Bộ nhớ** | GC tự dọn (đôi khi chậm) | Không GC, tự quản lý (nhanh hơn) |
| **Dùng cho** | Web, AI, scripting, automation | Game, OS, trình duyệt, blockchain |

> 💡 **Tóm lại:** Python = viết nhanh nhưng chạy chậm. Rust = viết chậm hơn nhưng chạy siêu nhanh + an toàn.

---

## 1.2 Cài Đặt Rust

### Windows
1. Vào [https://rustup.rs](https://rustup.rs)
2. Tải và chạy **rustup-init.exe**
3. Chọn mặc định (nhấn Enter)

### macOS / Linux
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Kiểm tra (giống `python --version`):
```bash
rustc --version    # Compiler Rust
cargo --version    # Package manager (giống pip)
```

---

## 1.3 Cargo = pip + venv + build tool

| Bạn dùng trong Python | Tương đương trong Rust |
|-----------------------|----------------------|
| `pip install requests` | `cargo add requests` (thêm vào Cargo.toml) |
| `python file.py` | `cargo run` |
| `pip freeze > requirements.txt` | `Cargo.toml` (tự động) |
| `python -m venv env` | Không cần! Cargo quản lý tự động |
| `pytest` | `cargo test` |

### Tạo project mới:
```bash
# Python: tạo file .py là xong
# Rust: cần tạo project

cargo new hello_rust     # Giống "mkdir + setup"
cd hello_rust
cargo run                # Build + chạy
```

Cấu trúc được tạo:
```
hello_rust/
├── Cargo.toml    ← giống requirements.txt + setup.py
└── src/
    └── main.rs   ← file code chính (giống main.py)
```

---

## 1.4 Chương Trình Đầu Tiên - Giải Thích TỪNG CHỮ

### 🐍 Python:
```python
print("Xin chào, Python!")
```

### 🦀 Rust:
```rust
fn main() {
    println!("Xin chào, Rust! 🦀");
}
```

### 🔍 Giải thích từng phần:

| Code | Nghĩa là gì | So với Python |
|------|-------------|---------------|
| `fn` | **f**unctio**n** = khai báo hàm | `def` trong Python |
| `main()` | Hàm chính, chạy đầu tiên | Python cũng có `if __name__ == "__main__"` |
| `{` và `}` | Bắt đầu/kết thúc block code | Python dùng `:` + thụt dòng |
| `println!` | In ra màn hình | `print()` trong Python |
| `!` sau println | Đây là **macro** (không phải hàm thường) | Không có trong Python |
| `"..."` | Chuỗi (string) | Giống Python |
| `;` | Kết thúc câu lệnh | Python KHÔNG cần `;` |

### 📌 Tại sao Rust cần nhiều thứ hơn Python?

> Python: `print("hello")` → 1 dòng, xong!
>
> Rust: phải có `fn main() { }` → vì Rust **LUÔN** cần 1 hàm `main` làm điểm bắt đầu. Rust không chạy code "rời" ngoài hàm.

---

## 1.5 Giải Thích Các Từ Khóa Bạn Sẽ Gặp Liên Tục

| Từ khóa Rust | Đọc là | Nghĩa | Tương đương Python |
|-------------|--------|-------|-------------------|
| `fn` | "function" | Khai báo hàm | `def` |
| `let` | "let" | Khai báo biến | `x = 5` (Python tự hiểu) |
| `let mut` | "let mutable" | Biến có thể thay đổi | Mặc định Python đã thay đổi được |
| `println!` | "print line" | In + xuống dòng | `print()` |
| `//` | "comment" | Ghi chú | `#` trong Python |
| `->` | "returns" | Kiểu trả về của hàm | `-> int` (type hint Python) |
| `&` | "reference" | Cho mượn (borrowing) | Không có trong Python |
| `::` | "path" | Truy cập method/module | `.` trong Python |
| `{}` | "placeholder" | Chỗ điền giá trị trong string | `{}` trong f-string |

---

## 1.6 Ví Dụ So Sánh Chi Tiết

### 🐍 Python:
```python
# Khai báo biến
ten = "An"
tuoi = 25

# In ra
print(f"Tôi là {ten}, {tuoi} tuổi")

# Hàm
def cong(a, b):
    return a + b

print(cong(3, 5))
```

### 🦀 Rust (giải thích từng dòng):
```rust
fn main() {                              // fn = def, main = hàm chính
    // Khai báo biến (let = "tôi khai báo biến mới")
    let ten = "An";                      // let ten = ... (giống ten = "An")
    let tuoi = 25;                       // let tuoi = 25

    // In ra (println! = print, {} = chỗ điền giá trị)
    println!("Tôi là {}, {} tuổi", ten, tuoi);

    // Gọi hàm
    println!("{}", cong(3, 5));
}

// fn = def, (a: i32, b: i32) = tham số + kiểu, -> i32 = kiểu trả về
fn cong(a: i32, b: i32) -> i32 {
    a + b    // Dòng cuối không có ; = return (tự trả về)
}
```

---

## 1.7 Tóm Tắt: "Phiên Dịch" Python → Rust

| Bạn viết trong Python | Viết trong Rust | Ghi nhớ |
|-----------------------|----------------|---------|
| `x = 5` | `let x = 5;` | Thêm `let` đầu, `;` cuối |
| `x = 5` (muốn đổi sau) | `let mut x = 5;` | Thêm `mut` nếu muốn thay đổi |
| `def foo():` | `fn foo() {` | `def` → `fn`, `:` → `{` |
| `def foo(a, b):` | `fn foo(a: i32, b: i32) {` | Phải ghi kiểu tham số |
| `return x` | `x` (không `;`) hoặc `return x;` | Dòng cuối bỏ `;` = return |
| `print(f"x={x}")` | `println!("x={}", x);` | `!` và `{}` |
| `# comment` | `// comment` | `#` → `//` |
| `:` + thụt dòng | `{ }` | Block dùng ngoặc nhọn |
| Không cần `;` | **CẦN** `;` cuối mỗi lệnh | Hay quên → compiler nhắc! |

---

## 1.8 Các Lệnh Cargo Hay Dùng

| Lệnh | Làm gì | Giống Python |
|------|--------|-------------|
| `cargo new ten_project` | Tạo project mới | `mkdir + touch` |
| `cargo run` | Compile + chạy | `python main.py` |
| `cargo build` | Chỉ compile (không chạy) | - |
| `cargo check` | Kiểm tra lỗi nhanh | `python -m py_compile` |
| `cargo test` | Chạy tests | `pytest` |
| `cargo add ten_thu_vien` | Thêm thư viện | `pip install` |

---

## 1.9 Bài Tập

1. Cài Rust, chạy `rustc --version` và `cargo --version`
2. Tạo project mới: `cargo new bai_tap_01`
3. Sửa `src/main.rs` để in ra: "Tôi là [tên bạn], đang học Rust!"
4. Chạy: `cargo run`
5. Thử **CỐ TÌNH** viết sai (bỏ `;`, sai tên biến) → đọc lỗi compiler trả về

> 💡 **Mẹo:** Đọc lỗi compiler! Rust có message lỗi RẤT RÕ RÀNG, thậm chí gợi ý cách sửa. Đừng sợ lỗi!

---

📖 **Tiếp theo**: [Chương 2 - Cú pháp cơ bản](../chuong-02-cu-phap-co-ban/README.md)
