# Chương 1: Giới Thiệu Rust & Cài Đặt

## 1.1 Rust Là Gì?

**Rust** là ngôn ngữ lập trình hệ thống (systems programming language) được Mozilla phát triển từ năm 2010 và phát hành phiên bản ổn định đầu tiên (1.0) vào năm 2015.

### Đặc Điểm Nổi Bật

| Đặc điểm | Mô tả |
|-----------|--------|
| **An toàn bộ nhớ** | Không có null pointer, không data race |
| **Hiệu năng cao** | Ngang C/C++, không có garbage collector |
| **Zero-cost abstractions** | Abstraction không tốn thêm chi phí runtime |
| **Hệ thống kiểu mạnh** | Phát hiện lỗi tại compile time |
| **Concurrency an toàn** | Ngăn chặn data race tại compile time |

### Tại Sao Nên Học Rust?

1. **An toàn**: Hệ thống ownership giúp tránh bug về bộ nhớ
2. **Nhanh**: Hiệu năng tương đương C/C++
3. **Cộng đồng thân thiện**: Được bình chọn là ngôn ngữ "yêu thích nhất" nhiều năm liên tiếp trên Stack Overflow
4. **Đa dụng**: Web (WebAssembly), hệ thống, game, embedded, CLI tools...
5. **Tương lai**: Được sử dụng bởi Microsoft, Google, Amazon, Meta, Linux kernel...

## 1.2 Cài Đặt Rust

### Trên Linux / macOS

Mở terminal và chạy:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Sau khi cài xong, thêm Rust vào PATH:

```bash
source $HOME/.cargo/env
```

### Trên Windows

1. Tải **rustup-init.exe** từ [https://rustup.rs](https://rustup.rs)
2. Chạy file và làm theo hướng dẫn
3. Cài đặt [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) nếu được yêu cầu

### Kiểm Tra Cài Đặt

```bash
rustc --version
# Kết quả mẫu: rustc 1.XX.0 (abc1234 2024-XX-XX)

cargo --version
# Kết quả mẫu: cargo 1.XX.0 (abc1234 2024-XX-XX)
```

### Cập Nhật Rust

```bash
rustup update
```

## 1.3 Công Cụ Phát Triển

### Cargo - Package Manager & Build Tool

Cargo là công cụ đa năng đi kèm Rust:

- **Quản lý dependencies** (thư viện bên ngoài)
- **Build** (biên dịch code)
- **Test** (chạy test)
- **Publish** (đăng thư viện lên crates.io)

### IDE / Editor Khuyến Nghị

| Editor | Plugin |
|--------|--------|
| **VS Code** | rust-analyzer (khuyến nghị nhất) |
| **IntelliJ IDEA** | Rust plugin |
| **Neovim** | rust-analyzer + LSP |
| **Sublime Text** | Rust Enhanced |

## 1.4 Chương Trình Đầu Tiên: Hello World

### Cách 1: Dùng `rustc` trực tiếp

Tạo file `hello.rs`:

```rust
fn main() {
    println!("Xin chào, Rust! 🦀");
}
```

Biên dịch và chạy:

```bash
rustc hello.rs
./hello
```

### Cách 2: Dùng Cargo (Khuyến nghị)

```bash
# Tạo project mới
cargo new hello_rust
cd hello_rust

# Cấu trúc thư mục được tạo:
# hello_rust/
# ├── Cargo.toml    (file cấu hình project)
# └── src/
#     └── main.rs   (file code chính)

# Build và chạy
cargo run
```

### Giải Thích Code

```rust
fn main() {           // Hàm main - điểm bắt đầu của chương trình
    println!("Xin chào, Rust! 🦀");  // In ra màn hình
}                     // Kết thúc hàm
```

- `fn` - từ khóa khai báo hàm (function)
- `main()` - hàm đặc biệt, luôn chạy đầu tiên
- `println!` - macro in ra màn hình (có dấu `!` nghĩa là macro, không phải function)
- Mỗi câu lệnh kết thúc bằng `;`

## 1.5 Các Lệnh Cargo Thường Dùng

| Lệnh | Công dụng |
|------|-----------|
| `cargo new <tên>` | Tạo project mới |
| `cargo build` | Biên dịch (debug mode) |
| `cargo build --release` | Biên dịch (release mode, tối ưu) |
| `cargo run` | Build + Chạy |
| `cargo check` | Kiểm tra lỗi (nhanh hơn build) |
| `cargo test` | Chạy tests |
| `cargo doc --open` | Tạo và mở documentation |
| `cargo fmt` | Format code |
| `cargo clippy` | Kiểm tra code style & gợi ý |

## 1.6 Bài Tập

1. Cài đặt Rust trên máy tính của bạn
2. Tạo project `bai-tap-01` bằng Cargo
3. Sửa `main.rs` để in ra tên của bạn
4. Thử các lệnh: `cargo check`, `cargo build`, `cargo run`

---

📖 **Tiếp theo**: [Chương 2 - Cú pháp cơ bản](../chuong-02-cu-phap-co-ban/README.md)
