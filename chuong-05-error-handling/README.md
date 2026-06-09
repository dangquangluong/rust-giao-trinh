# Chương 5: Xử Lý Lỗi (try/except thành Result)

> 🐍 Python dùng `try/except` bắt lỗi lúc chạy. Rust dùng `Result` bắt lỗi lúc **compile**!

## 5.1 So Sánh Tổng Quan

| | Python 🐍 | Rust 🦀 |
|-|-----------|---------|
| Cơ chế | `try/except` (exception) | `Result<T, E>` (enum) |
| Khi nào phát hiện lỗi | **Lúc chạy** (runtime) | **Lúc compile** |
| Quên xử lý lỗi | Bug âm thầm, crash | Compiler **bắt lỗi** ngay |
| Lỗi nghiêm trọng | `raise Exception` | `panic!` |
| Lỗi xử lý được | `try/except` | `Result` + `?` |

---

## 5.2 Python: try/except

```python
def doc_file(path):                      # Hàm đọc file
    try:                                 # Bắt đầu khối try
        with open(path) as f:            # Mở file (tự đóng khi xong)
            return f.read()              # Đọc toàn bộ nội dung
    except FileNotFoundError:            # Bắt lỗi file không tồn tại
        return "File không tồn tại"
    except Exception as e:               # Bắt mọi lỗi khác
        return f"Lỗi: {e}"

# Vấn đề: NẾU QUÊN try/except thì crash lúc chạy!
```

## 5.3 Rust: Result<T, E>

```rust
use std::fs;                                     // Import module fs (file system)
                                                  // Giống from pathlib import Path

fn doc_file(path: &str) -> Result<String, std::io::Error> {
                                                  // Result<String, Error> = trả về String nếu OK,
                                                  // hoặc Error nếu lỗi
                                                  // Giống -> Union[str, Exception] nhưng compiler kiểm tra!
    fs::read_to_string(path)                     // Đọc file thành String, tự trả về Result
                                                  // Giống Path(path).read_text() nhưng trả Result
}

fn main() {                                       // Hàm chính
    // BẮT BUỘC phải xử lý Result! (khác Python - Python cho phép bỏ qua)
    match doc_file("hello.txt") {                // match = xét kết quả
        Ok(noi_dung) => println!("{}", noi_dung), // Ok = thành công, lấy nội dung ra
                                                  // Giống: noi_dung = doc_file(...) khi không lỗi
        Err(loi) => println!("Lỗi: {}", loi),   // Err = có lỗi, lấy thông tin lỗi
                                                  // Giống: except Exception as loi
    }
}
```

> 📌 **Result giống Option:** `Ok(value)` = có kết quả, `Err(error)` = có lỗi

---

## 5.4 Các Cách Xử Lý Result (Từ Lười tới Chuyên Nghiệp)

### Cách 1: `unwrap()` - Lười nhất (giống không dùng try/except)
```rust
let noi_dung = fs::read_to_string("file.txt").unwrap();
                                                  // unwrap() = "tôi chắc chắn không lỗi, cứ lấy giá trị"
                                                  // Nếu lỗi thì panic! (crash giống Python exception không bắt)
                                                  // Chỉ dùng khi test/prototype!
```

### Cách 2: `expect()` - Lười nhưng có message
```rust
let noi_dung = fs::read_to_string("file.txt")
    .expect("Không thể đọc file!");              // expect() = unwrap() + message lỗi rõ ràng
                                                  // Nếu lỗi thì panic với "Không thể đọc file!"
                                                  // Giống assert trong Python
```

### Cách 3: `unwrap_or()` - Giá trị mặc định
```rust
let noi_dung = fs::read_to_string("file.txt")
    .unwrap_or(String::from("File rỗng"));       // Nếu lỗi thì dùng giá trị mặc định
                                                  // Giống Python: noi_dung = doc() or "File rỗng"
```

### Cách 4: `match` - Xử lý đầy đủ
```rust
match fs::read_to_string("file.txt") {           // Xét kết quả đọc file
    Ok(data) => println!("Nội dung: {}", data),  // Thành công: in nội dung
    Err(e) => println!("Lỗi: {}", e),           // Lỗi: in thông báo lỗi
}                                                 // Giống try/except đầy đủ
```

### Cách 5: `?` - Truyền lỗi lên (giống raise lại)
```rust
fn doc_so_tu_file(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
                                                  // Box<dyn Error> = chấp nhận mọi loại lỗi
                                                  // Giống -> int nhưng có thể raise Exception
    let text = fs::read_to_string(path)?;        // ? = nếu Err thì return Err ngay (truyền lỗi lên)
                                                  // Giống: text = open(path).read() (để exception tự bay lên)
    let so = text.trim().parse::<i32>()?;        // parse = int() Python, ? = nếu lỗi thì truyền lên
                                                  // trim() = strip() Python
    Ok(so)                                        // Thành công, wrap trong Ok()
}
```

> 💡 **Toán tử `?`** = "nếu lỗi thì return lỗi luôn, không xử lý ở đây". Giống để exception tự bay lên trong Python!

---

## 5.5 So Sánh Cách Xử Lý

| Cách | Giống Python | Khi nào dùng |
|------|-------------|--------------|
| `unwrap()` | Không try/except | Test, prototype |
| `expect("msg")` | assert + message | Debug |
| `unwrap_or(default)` | `x if x else default` | Có giá trị mặc định |
| `match` | `try/except` đầy đủ | Xử lý cẩn thận |
| `?` | `raise` (truyền lỗi lên) | Trong hàm trả Result |

---

## 5.6 Ví Dụ Thực Tế

### 🐍 Python:
```python
def tinh_tuoi(input_str):                # Hàm parse tuổi từ string
    try:                                 # Bắt đầu try
        tuoi = int(input_str)            # Chuyển string thành int
        if tuoi < 0 or tuoi > 150:       # Kiểm tra hợp lệ
            raise ValueError(f"Tuổi {tuoi} không hợp lệ")  # Raise lỗi
        return tuoi                      # Trả về tuổi
    except ValueError as e:              # Bắt lỗi ValueError
        print(f"Lỗi: {e}")
        return None                      # Trả None khi lỗi
```

### 🦀 Rust (tương đương):
```rust
fn tinh_tuoi(input_str: &str) -> Result<u8, String> {
                                                  // Result<u8, String> = trả u8 nếu OK, String nếu lỗi
                                                  // u8 vì tuổi 0-255 là đủ
    let tuoi: u8 = input_str.trim().parse()      // trim() = strip(), parse() = int()
        .map_err(|_| format!("'{}' không phải số", input_str))?;
                                                  // map_err = chuyển lỗi thành message dễ đọc
                                                  // ? = nếu parse lỗi thì return Err ngay

    if tuoi > 150 {                              // Kiểm tra logic (giống Python)
        return Err(format!("Tuổi {} không hợp lệ", tuoi));
                                                  // return Err() = raise ValueError()
    }

    Ok(tuoi)                                     // Thành công! Wrap trong Ok()
                                                  // Giống return tuoi nhưng rõ ràng là "thành công"
}

fn main() {                                       // Hàm chính
    let tests = ["25", "abc", "200"];            // Array các test case
    for input in tests {                          // Duyệt qua từng test (giống for in Python)
        match tinh_tuoi(input) {                 // Xét kết quả
            Ok(tuoi) => println!("✅ '{}' → {} tuổi", input, tuoi),   // Thành công
            Err(e) => println!("❌ '{}' → {}", input, e),             // Lỗi
        }
    }
}
```

---

## 5.7 Bài Tập

1. Viết `chia(a: f64, b: f64) -> Result<f64, String>` (lỗi khi b=0)
2. Viết hàm parse "key=value" thành tuple, trả Result nếu format sai
3. Chuỗi xử lý: đọc file, parse số, nhân 2, in kết quả (dùng `?`)

---

📖 **Trước đó**: [Chương 4](../chuong-04-struct-enum/README.md) | **Tiếp theo**: [Chương 6](../chuong-06-collections-generics/README.md)
