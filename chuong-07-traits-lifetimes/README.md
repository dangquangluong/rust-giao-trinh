# Chương 7: Traits & Lifetimes (ABC/Protocol → Trait)

## 7.1 Trait = ABC / Protocol Trong Python

> 📌 **Trait** = "hợp đồng" quy định một kiểu phải có những method nào.

### 🐍 Python (Abstract Base Class):
```python
from abc import ABC, abstractmethod

class HinhHoc(ABC):
    @abstractmethod
    def dien_tich(self) -> float:
        pass

    def mo_ta(self):  # Method có sẵn (default)
        return f"Diện tích: {self.dien_tich():.2f}"

class HinhTron(HinhHoc):
    def __init__(self, r):
        self.r = r

    def dien_tich(self):
        return 3.14159 * self.r ** 2

class HinhVuong(HinhHoc):
    def __init__(self, a):
        self.a = a

    def dien_tich(self):
        return self.a ** 2
```

### 🦀 Rust (Trait):
```rust
trait HinhHoc {
    fn dien_tich(&self) -> f64;      // Bắt buộc implement

    fn mo_ta(&self) -> String {      // Default method (có sẵn)
        format!("Diện tích: {:.2}", self.dien_tich())
    }
}

struct HinhTron { ban_kinh: f64 }
struct HinhVuong { canh: f64 }

impl HinhHoc for HinhTron {
    fn dien_tich(&self) -> f64 {
        3.14159 * self.ban_kinh * self.ban_kinh
    }
}

impl HinhHoc for HinhVuong {
    fn dien_tich(&self) -> f64 {
        self.canh * self.canh
    }
}

fn main() {
    let tron = HinhTron { ban_kinh: 5.0 };
    let vuong = HinhVuong { canh: 4.0 };

    println!("{}", tron.mo_ta());   // "Diện tích: 78.54"
    println!("{}", vuong.mo_ta());  // "Diện tích: 16.00"
}
```

### So sánh:

| Python | Rust |
|--------|------|
| `class ABC` + `@abstractmethod` | `trait TenTrait { fn ...; }` |
| `class Foo(ABC):` | `impl TenTrait for Foo {}` |
| `def method(self):` | `fn method(&self)` |
| Duck typing | Trait bounds (compile-time check) |

---

## 7.2 Derive - "Auto-implement" Traits Phổ Biến

### 🐍 Python:
```python
@dataclass   # Tự tạo __init__, __repr__, __eq__
class Point:
    x: float
    y: float
```

### 🦀 Rust:
```rust
#[derive(Debug, Clone, PartialEq)]  // Tự implement Debug, Clone, ==
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1.clone();               // clone() nhờ derive(Clone)
    println!("{:?}", p1);               // Debug print nhờ derive(Debug)
    println!("p1 == p2: {}", p1 == p2); // So sánh nhờ derive(PartialEq)
}
```

| Derive | Giống Python | Ý nghĩa |
|--------|-------------|---------|
| `Debug` | `__repr__` | In dạng debug `{:?}` |
| `Clone` | `copy.deepcopy` | Tạo bản sao |
| `PartialEq` | `__eq__` | So sánh `==` |
| `Default` | Giá trị mặc định | `Point::default()` |

---

## 7.3 Trait Bound = Type Hint Nhưng Nghiêm Ngặt

### 🐍 Python (chỉ là gợi ý, không bắt buộc):
```python
from typing import Protocol

class Printable(Protocol):
    def to_string(self) -> str: ...

def in_ra(obj: Printable):  # Python KHÔNG kiểm tra thật!
    print(obj.to_string())
```

### 🦀 Rust (compiler KIỂM TRA thật):
```rust
fn in_dien_tich(hinh: &impl HinhHoc) {
    // Chỉ chấp nhận kiểu implement HinhHoc
    println!("S = {:.2}", hinh.dien_tich());
}

// Hoặc dùng generic syntax
fn in_info<T: HinhHoc>(hinh: &T) {
    println!("{}", hinh.mo_ta());
}
```

---

## 7.4 Lifetimes - Khái Niệm Chỉ Rust Có

> ⚠️ Đây là phần KHÓ NHẤT. Nhưng đừng lo - hầu hết trường hợp Rust **tự suy luận** được!

### 🤔 Vấn đề gì?

```rust
// ❌ Code này KHÔNG compile
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
// Rust hỏi: "reference trả về sống bao lâu?"
```

### ✅ Giải pháp: Thêm lifetime annotation `'a`

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("dài hơn");
    let s2 = String::from("ngắn");
    let result = longest(&s1, &s2);
    println!("Dài hơn: {}", result);
}
```

### 💡 Giải thích đơn giản:

> `'a` nói với compiler: "reference trả về sống **ít nhất** bằng thời gian ngắn nhất trong 2 tham số"

### 📌 Mẹo cho người mới:

1. **90% trường hợp** Rust tự suy luận lifetime - bạn không cần ghi!
2. Chỉ cần ghi khi **hàm trả về reference** từ nhiều tham số
3. Nếu gặp lỗi lifetime → thử dùng `String` thay vì `&str` (đơn giản hơn)

---

## 7.5 Bài Tập

1. Tạo trait `Serializable` với method `to_json(&self) -> String`
2. Implement cho struct `User` và `Product`
3. Viết hàm generic `in_list<T: std::fmt::Display>(items: &[T])`

---

📖 **Trước đó**: [Chương 6](../chuong-06-collections-generics/README.md) | **Tiếp theo**: [Chương 8](../chuong-08-concurrency/README.md)
