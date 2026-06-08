# Chương 7: Traits & Lifetimes (ABC/Protocol thành Trait)

## 7.1 Trait = ABC / Protocol Trong Python

> 📌 **Trait** = "hợp đồng" quy định một kiểu phải có những method nào.

### 🐍 Python (Abstract Base Class):
```python
from abc import ABC, abstractmethod      # Import ABC

class HinhHoc(ABC):                      # Class trừu tượng (không tạo instance trực tiếp được)
    @abstractmethod                      # Decorator: bắt buộc class con phải implement
    def dien_tich(self) -> float:        # Method trừu tượng (chỉ khai báo, không code)
        pass

    def mo_ta(self):                     # Method có sẵn (default) - class con không cần viết lại
        return f"Diện tích: {self.dien_tich():.2f}"

class HinhTron(HinhHoc):                 # Kế thừa HinhHoc
    def __init__(self, r):               # Constructor
        self.r = r

    def dien_tich(self):                 # Implement method trừu tượng (bắt buộc!)
        return 3.14159 * self.r ** 2

class HinhVuong(HinhHoc):                # Kế thừa HinhHoc
    def __init__(self, a):
        self.a = a

    def dien_tich(self):                 # Implement method trừu tượng
        return self.a ** 2
```

### 🦀 Rust (Trait):
```rust
trait HinhHoc {                                  // trait = khai báo "hợp đồng" (giống ABC Python)
                                                  // Các kiểu implement trait này phải có dien_tich()
    fn dien_tich(&self) -> f64;                  // Method bắt buộc (chỉ khai báo, có ; ở cuối)
                                                  // Giống @abstractmethod Python

    fn mo_ta(&self) -> String {                  // Default method (có code sẵn, không bắt buộc override)
                                                  // Giống method thường trong ABC Python
        format!("Diện tích: {:.2}", self.dien_tich())  // format!() = f"..." Python
    }
}

struct HinhTron { ban_kinh: f64 }                // Struct chứa data (giống class HinhTron)
struct HinhVuong { canh: f64 }                   // Struct chứa data

impl HinhHoc for HinhTron {                      // "HinhTron implement trait HinhHoc"
                                                  // Giống class HinhTron(HinhHoc): trong Python
    fn dien_tich(&self) -> f64 {                 // Implement method bắt buộc
        3.14159 * self.ban_kinh * self.ban_kinh  // Công thức diện tích hình tròn
    }
}

impl HinhHoc for HinhVuong {                     // HinhVuong cũng implement HinhHoc
    fn dien_tich(&self) -> f64 {                 // Implement method bắt buộc
        self.canh * self.canh                    // Công thức diện tích hình vuông
    }
}

fn main() {                                       // Hàm chính
    let tron = HinhTron { ban_kinh: 5.0 };       // Tạo HinhTron (giống HinhTron(5.0) Python)
    let vuong = HinhVuong { canh: 4.0 };         // Tạo HinhVuong

    println!("{}", tron.mo_ta());                 // Gọi default method: "Diện tích: 78.54"
    println!("{}", vuong.mo_ta());                // "Diện tích: 16.00"
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
@dataclass                           # Tự tạo __init__, __repr__, __eq__
class Point:
    x: float
    y: float
```

### 🦀 Rust:
```rust
#[derive(Debug, Clone, PartialEq)]           // Tự implement Debug, Clone, == cho struct
                                              // Giống @dataclass Python (tự tạo __repr__, copy, __eq__)
struct Point {                                // Khai báo struct
    x: f64,                                   // Field x
    y: f64,                                   // Field y
}

fn main() {                                   // Hàm chính
    let p1 = Point { x: 1.0, y: 2.0 };      // Tạo Point
    let p2 = p1.clone();                      // clone() hoạt động nhờ derive(Clone)
                                              // Giống copy.deepcopy(p1) Python
    println!("{:?}", p1);                     // {:?} = debug print nhờ derive(Debug)
                                              // In: Point { x: 1.0, y: 2.0 } (giống __repr__)
    println!("p1 == p2: {}", p1 == p2);      // == hoạt động nhờ derive(PartialEq)
                                              // Giống __eq__ Python
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
from typing import Protocol              # Import Protocol

class Printable(Protocol):               # Khai báo Protocol (structural typing)
    def to_string(self) -> str: ...      # Method signature

def in_ra(obj: Printable):               # Type hint nhưng Python KHÔNG kiểm tra thật!
    print(obj.to_string())               # Runtime mới biết obj có to_string() không
```

### 🦀 Rust (compiler KIỂM TRA thật):
```rust
// Cách 1: impl Trait (đơn giản)
fn in_dien_tich(hinh: &impl HinhHoc) {           // &impl HinhHoc = nhận bất kỳ kiểu nào implement HinhHoc
                                                  // Compiler KIỂM TRA lúc compile (không phải runtime!)
    println!("S = {:.2}", hinh.dien_tich());     // Chắc chắn có dien_tich() vì trait yêu cầu
}

// Cách 2: Generic syntax (tương đương nhưng linh hoạt hơn)
fn in_info<T: HinhHoc>(hinh: &T) {              // <T: HinhHoc> = T phải implement HinhHoc
                                                  // Giống TypeVar('T', bound=HinhHoc) Python
    println!("{}", hinh.mo_ta());                // Gọi method từ trait
}
```

---

## 7.4 Lifetimes - Khái Niệm Chỉ Rust Có

> ⚠️ Đây là phần KHÓ NHẤT. Nhưng đừng lo - hầu hết trường hợp Rust **tự suy luận** được!

### 🤔 Vấn đề gì?

```rust
// ❌ Code này KHÔNG compile
fn longest(x: &str, y: &str) -> &str {          // Trả về reference, nhưng reference của ai?
    if x.len() > y.len() { x } else { y }       // Có thể trả x hoặc y
}                                                 // Rust hỏi: "reference trả về sống bao lâu?"
                                                  // Python không có vấn đề này vì GC quản lý bộ nhớ
```

### ✅ Giải pháp: Thêm lifetime annotation `'a`

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
                                                  // 'a = lifetime parameter (đánh dấu "thời gian sống")
                                                  // Nói compiler: "kết quả sống ít nhất bằng x và y"
                                                  // Python không cần vì GC tự quản lý
    if x.len() > y.len() { x } else { y }       // Trả về chuỗi dài hơn
}

fn main() {                                       // Hàm chính
    let s1 = String::from("dài hơn");            // String sống trong toàn bộ main
    let s2 = String::from("ngắn");               // String sống trong toàn bộ main
    let result = longest(&s1, &s2);              // Gọi hàm, result là reference
                                                  // Compiler biết result sống trong phạm vi s1, s2
    println!("Dài hơn: {}", result);             // OK vì s1, s2 vẫn sống
}
```

### 💡 Giải thích đơn giản:

> `'a` nói với compiler: "reference trả về sống **ít nhất** bằng thời gian ngắn nhất trong 2 tham số"

### 📌 Mẹo cho người mới:

1. **90% trường hợp** Rust tự suy luận lifetime - bạn không cần ghi!
2. Chỉ cần ghi khi **hàm trả về reference** từ nhiều tham số
3. Nếu gặp lỗi lifetime thì thử dùng `String` thay vì `&str` (đơn giản hơn)

---

## 7.5 Bài Tập

1. Tạo trait `Serializable` với method `to_json(&self) -> String`
2. Implement cho struct `User` và `Product`
3. Viết hàm generic `in_list<T: std::fmt::Display>(items: &[T])`

---

📖 **Trước đó**: [Chương 6](../chuong-06-collections-generics/README.md) | **Tiếp theo**: [Chương 8](../chuong-08-concurrency/README.md)
