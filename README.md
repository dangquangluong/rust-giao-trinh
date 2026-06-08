# 🦀 Giáo Trình Học Rust - Dành Cho Người Biết Python

> Bạn đã biết Python? Tuyệt! Giáo trình này sẽ dạy Rust bằng cách **so sánh trực tiếp với Python** để bạn không bị ngợp.

## 🐍 → 🦀 Python vs Rust - Khác Nhau Gì?

| Khía cạnh | Python 🐍 | Rust 🦀 |
|-----------|-----------|---------|
| **Tốc độ** | Chậm (interpreted) | Cực nhanh (compiled, ngang C) |
| **Kiểu dữ liệu** | Tự do, kiểm tra lúc chạy | Nghiêm ngặt, kiểm tra lúc compile |
| **Bộ nhớ** | Tự động (Garbage Collector) | Tự động nhưng KHÔNG có GC (Ownership) |
| **Biến** | Mặc định thay đổi được | Mặc định KHÔNG thay đổi được |
| **Null** | Có `None` | KHÔNG có null (dùng `Option`) |
| **Lỗi** | `try/except` (runtime) | `Result` (compile time) |
| **Chạy** | `python file.py` | `cargo run` (cần compile trước) |

### Ví Dụ Nhanh - Cùng 1 Logic:

**Python:**
```python
def chao(ten):
    return f"Xin chào, {ten}!"

print(chao("bạn"))
```

**Rust:**
```rust
fn chao(ten: &str) -> String {
    format!("Xin chào, {}!", ten)
}

fn main() {
    println!("{}", chao("bạn"));
}
```

> 💡 **Đừng lo!** Nhìn phức tạp hơn nhưng khi hiểu rồi bạn sẽ thấy Rust rất logic. Giáo trình này sẽ giải thích TỪNG khác biệt một.

## 📋 Mục Lục

| Chương | Nội Dung | Mức Độ | Tương đương Python |
|--------|----------|--------|-------------------|
| [01](./chuong-01-gioi-thieu/README.md) | Giới thiệu & Cài đặt | ⭐ | `pip install` → `cargo` |
| [02](./chuong-02-cu-phap-co-ban/README.md) | Cú pháp cơ bản | ⭐ | biến, if, for, hàm |
| [03](./chuong-03-ownership-borrowing/README.md) | Ownership & Borrowing | ⭐⭐ | *Python không có!* |
| [04](./chuong-04-struct-enum/README.md) | Struct & Enum | ⭐⭐ | class, dataclass |
| [05](./chuong-05-error-handling/README.md) | Xử lý lỗi | ⭐⭐ | try/except → Result |
| [06](./chuong-06-collections-generics/README.md) | Collections & Generics | ⭐⭐ | list, dict, typing |
| [07](./chuong-07-traits-lifetimes/README.md) | Traits & Lifetimes | ⭐⭐⭐ | ABC, protocol |
| [08](./chuong-08-concurrency/README.md) | Đồng thời | ⭐⭐⭐ | threading, asyncio |
| [09](./chuong-09-du-an-thuc-hanh/README.md) | Dự án thực hành | ⭐⭐⭐ | - |

## 🛤️ Lộ Trình Học (Chậm rãi, không vội!)

```
Tuần 1-2: Chương 1-2 → Quen cú pháp (giống Python nhưng thêm kiểu)
Tuần 3-5: Chương 3   → Ownership (QUAN TRỌNG NHẤT - dành thời gian!)
Tuần 6-7: Chương 4-5 → Struct/Enum/Error (quen dần)
Tuần 8-9: Chương 6-7 → Collections/Traits (nâng cao)
Tuần 10+: Chương 8-9 → Concurrency & Projects
```

> ⚠️ **Lời khuyên quan trọng:** Chương 3 (Ownership) là khái niệm MỚI HOÀN TOÀN - Python không có. Đừng vội, đọc đi đọc lại nhiều lần là bình thường!

## 🔧 Yêu Cầu

- Đã biết Python cơ bản (biến, hàm, class, list/dict)
- Máy tính có thể cài phần mềm
- Kiên nhẫn khi gặp lỗi compiler (ban đầu sẽ nhiều lỗi - đó là bình thường!)

## 📚 Tài Liệu Tham Khảo

- [The Rust Programming Language (The Book)](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings (Bài tập nhỏ)](https://github.com/rust-lang/rustlings)

## 💡 Mẹo Cho Người Từ Python Sang

1. **Rust compiler là bạn** - lỗi compile giúp bạn tránh bug, không phải để hành bạn!
2. **Đừng so sánh tốc độ viết code** - Rust viết chậm hơn Python nhưng chạy nhanh hơn 50-100 lần
3. **Kiểu dữ liệu** - Python tự đoán kiểu, Rust bắt bạn khai báo rõ ràng
4. **Ownership** - Nếu chưa hiểu, cứ dùng `.clone()` trước, sau hiểu sâu rồi tối ưu sau

---

*Chúc bạn chinh phục Rust thành công! Từ Python sang Rust = level up! 🐍 → 🦀*
