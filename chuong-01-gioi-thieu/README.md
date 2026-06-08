# Chương 1: Giới Thiệu Rust & Cài Đặt

## 1.1 Rust Là Gì? (Giải thích cho người biết Python)

Bạn biết Python rồi đúng không? Python dễ viết, dễ đọc, nhưng:
- Chạy **chậm** (vì interpreted)
- Tốn **nhiều RAM** (vì garbage collector)
- Lỗi chỉ phát hiện **lúc chạy** (runtime error)

**Rust** giải quyết tất cả những vấn đề đó:
- Chạy **nhanh như C/C++** (compiled trực tiếp ra mã máy)
- Tốn **ít RAM** (không có garbage collector)
- Lỗi phát hiện **trước khi chạy** (compile-time error)

### Tại sao Rust làm vậy?

> 🤔 Bạn có thể hỏi: "Nếu Rust tốt vậy sao mọi người không dùng Rust thay Python?"
>
> Câu trả lời: Vì Rust **khó viết hơn**. Bạn phải "nói" cho compiler biết nhiều thứ hơn (kiểu dữ liệu, ai sở hữu dữ liệu...). Đổi lại, chương trình chạy nhanh hơn 10-100 lần và gần như không có bug về bộ nhớ.

### Khi nào dùng Python, khi nào dùng Rust?

| Dùng Python khi... | Dùng Rust khi... |
|---------------------|-------------------|
| Viết script nhanh | Cần hiệu năng cao |
| Data science, ML | Viết CLI tool, system tool |
| Prototype, thử nghiệm | Game engine, web server nhanh |
| Không quan tâm tốc độ | WebAssembly |
| Automation | Embedded, IoT |

### Ai đang dùng Rust?

Microsoft, Google, Amazon, Meta, Discord, Cloudflare, Linux kernel... Rust được bình chọn là ngôn ngữ "yêu thích nhất" nhiều năm liên tiếp trên Stack Overflow.

## 1.2 Cài Đặt Rust

### Trên Linux / macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Sau khi cài xong:
```bash
source $HOME/.cargo/env
```

### Trên Windows

1. Tải **rustup-init.exe** từ [https://rustup.rs](https://rustup.rs)
2. Chạy file, làm theo hướng dẫn
3. Cài [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) nếu cần

### Kiểm Tra

```bash
rustc --version
cargo --version
```

> 💡 `rustc` giống `python` (chạy code), `cargo` giống `pip` + `poetry` (quản lý project & dependencies)

## 1.3 So Sánh Công Cụ: Python vs Rust

| Python | Rust | Công dụng |
|--------|------|-----------|
| `python file.py` | `cargo run` | Chạy chương trình |
| `pip install` | `cargo add` | Thêm thư viện |
| `pip freeze` / `requirements.txt` | `Cargo.toml` | Quản lý dependencies |
| `pytest` | `cargo test` | Chạy tests |
| `black` / `autopep8` | `cargo fmt` | Format code |
| `pylint` / `flake8` | `cargo clippy` | Kiểm tra code style |
| `python -m venv` | (không cần) | Rust tự tách project |

## 1.4 Chương Trình Đầu Tiên

### Trong Python:
```python
print("Xin chao, Python!")
```
Chạy: `python hello.py` - xong!

### Trong Rust:
```rust
fn main() {
    println!("Xin chao, Rust! 🦀");
}
```
Chạy: `rustc hello.rs && ./hello`

### Khác biệt gì?

| Python | Rust | Tại sao? |
|--------|------|----------|
| Không cần `main()` | Bắt buộc `fn main()` | Rust cần biết bắt đầu từ đâu |
| `print()` | `println!()` | Dấu `!` nghĩa là macro (không phải hàm) |
| Không có `;` | Kết thúc bằng `;` | Rust dùng `;` phân tách câu lệnh |
| Indentation quan trọng | Dùng `{}` | Rust dùng ngoặc nhọn cho block |

### Cách dùng Cargo (khuyến nghị)

```bash
# Tạo project mới (giống django-admin startproject)
cargo new hello_rust
cd hello_rust

# Cấu trúc:
# hello_rust/
# ├── Cargo.toml    (giống requirements.txt + setup.py)
# └── src/
#     └── main.rs   (code chính)

# Build + chạy
cargo run
```

## 1.5 Giải Thích Chi Tiết

```rust
fn main() {           // fn = function, main = hàm chạy đầu tiên
    println!("Xin chao, Rust! 🦀");  // println! = in ra + xuống dòng
}
```

So sánh với Python:
```python
# Python - không cần def main (nhưng nên có)
if __name__ == "__main__":
    print("Xin chao, Python!")
```

### Format string

**Python (f-string):**
```python
ten = "Ban"
print(f"Xin chao, {ten}!")
```

**Rust:**
```rust
let ten = "Ban";
println!("Xin chao, {}!", ten);
```

> 💡 Rust dùng `{}` thay vì `{ten}`. Từ Rust 1.58+, bạn cũng có thể viết `println!("Xin chao, {ten}!");`

## 1.6 Các Lệnh Cargo Hay Dùng

| Lệnh | Giống Python | Công dụng |
|------|-------------|-----------|
| `cargo new <ten>` | `mkdir` + setup | Tạo project mới |
| `cargo run` | `python main.py` | Build + Chạy |
| `cargo build` | - | Chỉ build (không chạy) |
| `cargo build --release` | - | Build tối ưu (nhanh hơn) |
| `cargo check` | - | Kiểm tra lỗi nhanh |
| `cargo test` | `pytest` | Chạy tests |
| `cargo fmt` | `black .` | Format code |
| `cargo clippy` | `flake8` | Gợi ý code tốt hơn |

## 1.7 Bài Tập

1. Cài đặt Rust trên máy
2. Tạo project `bai-tap-01` bằng Cargo
3. Sửa `main.rs` để in ra tên của bạn
4. Thử: `cargo check`, `cargo build`, `cargo run`
5. Thử sửa code sai (bỏ dấu `;`) rồi xem compiler báo lỗi gì

---

📖 **Tiếp theo**: [Chương 2 - Cú pháp cơ bản](../chuong-02-cu-phap-co-ban/README.md)
