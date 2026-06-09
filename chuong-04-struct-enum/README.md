# Chương 4: Struct & Enum (Giống Class & Dataclass Trong Python)

## 4.1 Struct = Giống Dataclass Python

### 🐍 Python:
```python
from dataclasses import dataclass        # Import dataclass decorator

@dataclass                               # Tự tạo __init__, __repr__, __eq__
class SinhVien:                          # Khai báo class
    ten: str                             # Thuộc tính ten, kiểu str
    tuoi: int                            # Thuộc tính tuoi, kiểu int
    diem: float                          # Thuộc tính diem, kiểu float

sv = SinhVien("An", 20, 8.5)            # Tạo object
print(f"{sv.ten} - {sv.diem}")           # Truy cập thuộc tính bằng dấu .
```

### 🦀 Rust:
```rust
struct SinhVien {                        // struct = khai báo kiểu dữ liệu (giống class/dataclass)
                                         // Rust KHÔNG có class, dùng struct thay thế
    ten: String,                         // Thuộc tính ten, kiểu String (phải ghi kiểu!)
    tuoi: u8,                            // Thuộc tính tuoi, kiểu u8 (0-255, tiết kiệm hơn int)
    diem: f64,                           // Thuộc tính diem, kiểu f64 (giống float Python)
}                                        // Kết thúc struct (lưu ý: KHÔNG có ; ở đây)

fn main() {                              // Hàm chính
    let sv = SinhVien {                  // Tạo instance (giống SinhVien("An", 20, 8.5) Python)
                                         // Rust dùng {} và ghi rõ tên field (không theo thứ tự)
        ten: String::from("An"),         // Gán giá trị cho field ten
        tuoi: 20,                        // Gán giá trị cho field tuoi
        diem: 8.5,                       // Gán giá trị cho field diem
    };                                   // ; vì đây là câu lệnh let
    println!("{} - {}", sv.ten, sv.diem); // Truy cập bằng . (giống Python sv.ten)
}
```

> 📌 **Khác biệt:** Rust không có `class`. Dùng `struct` + `impl` thay thế.

---

## 4.2 Thêm Method (impl = def trong class)

### 🐍 Python:
```python
class SinhVien:                          # Khai báo class
    def __init__(self, ten, diem):       # Constructor - chạy khi tạo object
        self.ten = ten                   # Lưu ten vào object
        self.diem = diem                 # Lưu diem vào object

    def xep_loai(self):                  # Method - hàm thuộc class
        if self.diem >= 8.0:             # Truy cập thuộc tính qua self
            return "Giỏi"
        elif self.diem >= 6.5:
            return "Khá"
        return "TB"

sv = SinhVien("An", 8.5)                # Gọi __init__
print(sv.xep_loai())                     # Gọi method: "Giỏi"
```

### 🦀 Rust:
```rust
struct SinhVien {                        // Khai báo struct (chỉ chứa data, chưa có method)
    ten: String,                         // Field ten
    diem: f64,                           // Field diem
}

impl SinhVien {                          // impl = "implement" = thêm method cho struct
                                         // Giống phần "def ..." bên trong class Python

    // "Constructor" (giống __init__ Python)
    fn moi(ten: &str, diem: f64) -> Self {   // Self = kiểu SinhVien (giống self.__class__)
                                              // Rust không có __init__, bạn tự viết hàm tạo
        SinhVien {                            // Tạo và trả về SinhVien mới
            ten: String::from(ten),           // Chuyển &str thành String
            diem,                             // Shorthand: diem: diem (giống Python self.diem = diem)
        }                                     // Không có ; = return (trả về SinhVien này)
    }

    // Method (giống def xep_loai(self) Python)
    fn xep_loai(&self) -> &str {             // &self = mượn self (giống self Python)
                                              // -> &str = trả về chuỗi cố định
        if self.diem >= 8.0 {                // Truy cập field qua self. (giống Python!)
            "Giỏi"                            // Return "Giỏi" (không có ; = return)
        } else if self.diem >= 6.5 {
            "Khá"
        } else {
            "TB"
        }
    }
}

fn main() {                                   // Hàm chính
    let sv = SinhVien::moi("An", 8.5);       // :: = gọi hàm của struct (giống SinhVien("An", 8.5))
                                              // :: dùng cho "associated function" (không có &self)
    println!("{}: {}", sv.ten, sv.xep_loai()); // . dùng gọi method (có &self), giống Python
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
trang_thai = "dang_xu_ly"      # Gõ sai "dang_xu_li" thì bug âm thầm!

# Hoặc dùng Enum (ít người dùng)
from enum import Enum            # Import Enum
class TrangThai(Enum):           # Khai báo Enum class
    DANG_XU_LY = 1              # Mỗi variant có giá trị
    HOAN_THANH = 2
    THAT_BAI = 3
```

### 🦀 Rust (enum mạnh hơn nhiều!):
```rust
// Enum cơ bản (giống Python Enum)
enum TrangThai {                             // enum = khai báo kiểu có nhiều variant
    DangXuLy,                                // Variant 1 (giống DANG_XU_LY = 1 Python)
    HoanThanh,                               // Variant 2
    ThatBai,                                 // Variant 3
}

// Enum CHỨA DỮ LIỆU (Python Enum KHÔNG làm được!)
enum ThongBao {                              // Mỗi variant có thể chứa data khác nhau!
    Text(String),                            // Variant chứa 1 String (giống tuple)
    Hinh { url: String, size: u32 },         // Variant chứa struct (nhiều field)
    Thoat,                                   // Variant không chứa gì
}

fn xu_ly(tb: ThongBao) {                    // Hàm nhận enum làm tham số
    match tb {                               // match = xét từng trường hợp (giống match/case Python 3.10)
        ThongBao::Text(noi_dung) => {        // Nếu là Text, lấy nội dung ra
            println!("Tin: {}", noi_dung);   // :: truy cập variant của enum
        }
        ThongBao::Hinh { url, size } => {    // Nếu là Hinh, destructure lấy url và size
            println!("Ảnh: {} ({}KB)", url, size);
        }
        ThongBao::Thoat => {                 // Nếu là Thoat
            println!("Thoát!");
        }
    }
}
```

> 💡 **Enum Rust = Enum + Union + Tagged data.** Mạnh hơn enum Python rất nhiều!

---

## 4.4 Option<T> - Thay Thế None Của Python

### Vấn đề với None trong Python:
```python
def tim(lst, target):            # Tìm vị trí phần tử trong list
    for i, v in enumerate(lst):  # Duyệt qua từng phần tử
        if v == target:
            return i             # Tìm thấy thì trả vị trí
    return None                  # Không thấy thì trả None

idx = tim([1,2,3], 5)           # idx = None (không tìm thấy 5)
print(idx + 1)                   # ❌ TypeError: None + 1 (runtime error!)
                                 # Python KHÔNG cảnh báo lúc viết code!
```

### Rust ÉP bạn phải xử lý trường hợp "không có":
```rust
fn tim(lst: &[i32], target: i32) -> Option<usize> {  // Option<usize> = có thể có hoặc không
                                                      // Giống Optional[int] trong Python typing
    for (i, &v) in lst.iter().enumerate() {           // iter() = tạo iterator, enumerate() giống Python
        if v == target {                              // So sánh
            return Some(i);                           // Some(i) = "có giá trị i" (giống return i)
        }
    }
    None                                              // Không tìm thấy (giống return None)
}

fn main() {                                           // Hàm chính
    let idx = tim(&[1, 2, 3], 5);                    // Gọi hàm, &[1,2,3] = mượn array

    // BẮT BUỘC phải xử lý cả 2 trường hợp! (Python không bắt buộc)
    match idx {                                       // match = xét từng case
        Some(i) => println!("Tìm thấy tại vị trí {}", i),  // Trường hợp có giá trị
        None => println!("Không tìm thấy!"),                 // Trường hợp không có
    }

    // Hoặc dùng unwrap_or (giống Python: idx if idx is not None else 0)
    let i = idx.unwrap_or(0);                        // Nếu None thì dùng 0
    println!("Vị trí: {}", i);
}
```

> 🤔 **Tại sao?** Rust KHÔNG có null/None. Dùng `Option` để compiler BẮT BẠN xử lý trường hợp rỗng, không bao giờ có NoneType error!

---

## 4.5 Pattern Matching (match = match/case Python 3.10)

### 🐍 Python 3.10+:
```python
match command:                   # match/case (Python 3.10+)
    case "start":                # Nếu command == "start"
        print("Bắt đầu")
    case "stop":                 # Nếu command == "stop"
        print("Dừng")
    case _:                      # _ = default (mọi trường hợp khác)
        print("Không hiểu")
```

### 🦀 Rust (mạnh hơn nhiều!):
```rust
fn main() {                                  // Hàm chính
    let so = 13;                             // Khai báo số để match

    match so {                               // match = xét giá trị so
        1 => println!("Một"),               // Nếu so == 1
        2..=12 => println!("Từ 2 đến 12"),  // 2..=12 = range 2 đến 12 (Python không có trong match)
        13 => println!("Số xui!"),          // Nếu so == 13
        _ => println!("Số khác"),           // _ = default (giống case _ Python)
    }

    // if let: khi chỉ quan tâm 1 trường hợp (gọn hơn match)
    let gia_tri: Option<i32> = Some(42);     // Option chứa giá trị 42
    if let Some(x) = gia_tri {               // Nếu gia_tri là Some, lấy x ra
                                             // Giống Python: if gia_tri is not None: x = gia_tri
        println!("Giá trị: {}", x);         // Dùng x
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
