# Chương 4: Struct & Enum (Giống Class & Dataclass Trong Python)

## 4.1 Struct = Giống Dataclass Python

### 🐍 Python:
```python
from dataclasses import dataclass

@dataclass
class SinhVien:
    ten: str
    tuoi: int
    diem: float

sv = SinhVien("An", 20, 8.5)
print(f"{sv.ten} - {sv.diem}")
```

### 🦀 Rust:
```rust
struct SinhVien {
    ten: String,
    tuoi: u8,
    diem: f64,
}

fn main() {
    let sv = SinhVien {
        ten: String::from("An"),
        tuoi: 20,
        diem: 8.5,
    };
    println!("{} - {}", sv.ten, sv.diem);
}
```

> 📌 **Khác biệt:** Rust không có `class`. Dùng `struct` + `impl` thay thế.

---

## 4.2 Thêm Method (impl = def trong class)

### 🐍 Python:
```python
class SinhVien:
    def __init__(self, ten, diem):
        self.ten = ten
        self.diem = diem

    def xep_loai(self):
        if self.diem >= 8.0:
            return "Giỏi"
        elif self.diem >= 6.5:
            return "Khá"
        return "TB"

sv = SinhVien("An", 8.5)
print(sv.xep_loai())  # "Giỏi"
```

### 🦀 Rust:
```rust
struct SinhVien {
    ten: String,
    diem: f64,
}

impl SinhVien {
    // "Constructor" (giống __init__)
    fn moi(ten: &str, diem: f64) -> Self {
        SinhVien {
            ten: String::from(ten),
            diem,
        }
    }

    // Method (giống def trong class)
    fn xep_loai(&self) -> &str {
        if self.diem >= 8.0 {
            "Giỏi"
        } else if self.diem >= 6.5 {
            "Khá"
        } else {
            "TB"
        }
    }
}

fn main() {
    let sv = SinhVien::moi("An", 8.5);
    println!("{}: {}", sv.ten, sv.xep_loai());
}
```

### So sánh nhanh:

| Python | Rust | Ghi chú |
|--------|------|---------|
| `class Foo:` | `struct Foo {}` + `impl Foo {}` | Tách data và behavior |
| `def __init__(self):` | `fn moi() -> Self` | Không có tên đặc biệt |
| `self.x` | `self.x` (trong `&self`) | Giống nhau! |
| `Foo()` | `Foo::moi()` | `::` = method của struct |
| `foo.method()` | `foo.method()` | Giống nhau! |

---

## 4.3 Enum - "Menu Có Sẵn Các Lựa Chọn"

> 🍽️ **Ví dụ đời thường:** Enum giống **menu quán ăn** - chỉ có những món đã liệt kê, không gọi bừa được!

### 🐍 Python (thường dùng string):
```python
# Python thường dùng string, dễ bị typo
trang_thai = "dang_xu_ly"  # Gõ sai "dang_xu_li" → bug!

# Hoặc dùng Enum (ít người dùng)
from enum import Enum
class TrangThai(Enum):
    DANG_XU_LY = 1
    HOAN_THANH = 2
    THAT_BAI = 3
```

### 🦀 Rust (enum mạnh hơn nhiều!):
```rust
// Enum cơ bản
enum TrangThai {
    DangXuLy,
    HoanThanh,
    ThatBai,
}

// Enum CHỨA DỮ LIỆU (Python không có!)
enum ThongBao {
    Text(String),                    // Chứa 1 String
    Hinh { url: String, size: u32 }, // Chứa nhiều field
    Thoat,                           // Không chứa gì
}

fn xu_ly(tb: ThongBao) {
    match tb {
        ThongBao::Text(noi_dung) => println!("Tin: {}", noi_dung),
        ThongBao::Hinh { url, size } => println!("Ảnh: {} ({}KB)", url, size),
        ThongBao::Thoat => println!("Thoát!"),
    }
}
```

> 💡 **Enum Rust = Enum + Union + Tagged data.** Mạnh hơn enum Python rất nhiều!

---

## 4.4 Option<T> - Thay Thế None Của Python

### Vấn đề với None trong Python:
```python
def tim(lst, target):
    for i, v in enumerate(lst):
        if v == target:
            return i
    return None  # Có thể quên check None → crash!

idx = tim([1,2,3], 5)
print(idx + 1)  # ❌ TypeError: None + 1 (runtime error!)
```

### Rust ÉP bạn phải xử lý trường hợp "không có":
```rust
fn tim(lst: &[i32], target: i32) -> Option<usize> {
    for (i, &v) in lst.iter().enumerate() {
        if v == target {
            return Some(i);  // Có giá trị
        }
    }
    None  // Không có
}

fn main() {
    let idx = tim(&[1, 2, 3], 5);

    // BẮT BUỘC phải xử lý cả 2 trường hợp!
    match idx {
        Some(i) => println!("Tìm thấy tại vị trí {}", i),
        None => println!("Không tìm thấy!"),
    }

    // Hoặc dùng unwrap_or (giống Python: x if x else default)
    let i = idx.unwrap_or(0);
    println!("Vị trí: {}", i);
}
```

> 🤔 **Tại sao?** Rust KHÔNG có null/None. Dùng `Option` để compiler BẮT BẠN xử lý trường hợp rỗng → không bao giờ có NoneType error!

---

## 4.5 Pattern Matching (match = match/case Python 3.10)

### 🐍 Python 3.10+:
```python
match command:
    case "start":
        print("Bắt đầu")
    case "stop":
        print("Dừng")
    case _:
        print("Không hiểu")
```

### 🦀 Rust (mạnh hơn nhiều!):
```rust
fn main() {
    let so = 13;

    match so {
        1 => println!("Một"),
        2..=12 => println!("Từ 2 đến 12"),
        13 => println!("Số xui!"),
        _ => println!("Số khác"),  // _ = default
    }

    // if let: khi chỉ quan tâm 1 trường hợp
    let gia_tri: Option<i32> = Some(42);
    if let Some(x) = gia_tri {
        println!("Giá trị: {}", x);
    }
}
```

---

## 4.6 Bài Tập

1. Tạo struct `HinhChuNhat` với method `dien_tich()` và `chu_vi()`
2. Tạo enum `PhuongTien { XeDap, XeMay(String), OTo { hang: String, so_cho: u8 } }`
3. Viết hàm `chia(a: f64, b: f64) -> Option<f64>` (trả None khi chia cho 0)

---

📖 **Trước đó**: [Chương 3](../chuong-03-ownership-borrowing/README.md) | **Tiếp theo**: [Chương 5](../chuong-05-error-handling/README.md)
