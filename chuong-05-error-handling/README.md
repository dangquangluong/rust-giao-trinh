# Chương 5: Xử Lý Lỗi (try/except → Result)

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
def doc_file(path):
    try:
        with open(path) as f:
            return f.read()
    except FileNotFoundError:
        return "File không tồn tại"
    except Exception as e:
        return f"Lỗi: {e}"

# Vấn đề: NẾU QUÊN try/except → crash lúc chạy!
```

## 5.3 Rust: Result<T, E>

```rust
use std::fs;

fn doc_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)  // Trả về Result
}

fn main() {
    // BẮT BUỘC phải xử lý Result!
    match doc_file("hello.txt") {
        Ok(noi_dung) => println!("{}", noi_dung),
        Err(loi) => println!("Lỗi: {}", loi),
    }
}
```

> 📌 **Result giống Option:** `Ok(value)` = có kết quả, `Err(error)` = có lỗi

---

## 5.4 Các Cách Xử Lý Result (Từ Lười → Chuyên Nghiệp)

### Cách 1: `unwrap()` - Lười nhất (giống không dùng try/except)
```rust
let noi_dung = fs::read_to_string("file.txt").unwrap();
// Nếu lỗi → panic! (crash). Chỉ dùng khi test/prototype!
```

### Cách 2: `expect()` - Lười nhưng có message
```rust
let noi_dung = fs::read_to_string("file.txt")
    .expect("Không thể đọc file!");
// Nếu lỗi → panic với message rõ ràng
```

### Cách 3: `unwrap_or()` - Giá trị mặc định (giống Python: x or default)
```rust
let noi_dung = fs::read_to_string("file.txt")
    .unwrap_or(String::from("File rỗng"));
// Nếu lỗi → dùng giá trị mặc định
```

### Cách 4: `match` - Xử lý đầy đủ
```rust
match fs::read_to_string("file.txt") {
    Ok(data) => println!("Nội dung: {}", data),
    Err(e) => println!("Lỗi: {}", e),
}
```

### Cách 5: `?` - Truyền lỗi lên (giống raise lại)
```rust
fn doc_so_tu_file(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;  // Nếu lỗi → return Err ngay
    let so = text.trim().parse::<i32>()?;  // Nếu lỗi → return Err ngay
    Ok(so)
}
```

> 💡 **Toán tử `?`** = "nếu lỗi thì return lỗi luôn, không xử lý ở đây". Giống `raise` trong Python nhưng gọn hơn!

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
def tinh_tuoi(input_str):
    try:
        tuoi = int(input_str)
        if tuoi < 0 or tuoi > 150:
            raise ValueError(f"Tuổi {tuoi} không hợp lệ")
        return tuoi
    except ValueError as e:
        print(f"Lỗi: {e}")
        return None
```

### 🦀 Rust (tương đương):
```rust
fn tinh_tuoi(input_str: &str) -> Result<u8, String> {
    let tuoi: u8 = input_str.trim().parse()
        .map_err(|_| format!("'{}' không phải số", input_str))?;

    if tuoi > 150 {
        return Err(format!("Tuổi {} không hợp lệ", tuoi));
    }

    Ok(tuoi)
}

fn main() {
    let tests = ["25", "abc", "200"];
    for input in tests {
        match tinh_tuoi(input) {
            Ok(tuoi) => println!("✅ '{}' → {} tuổi", input, tuoi),
            Err(e) => println!("❌ '{}' → {}", input, e),
        }
    }
}
```

---

## 5.7 Bài Tập

1. Viết `chia(a: f64, b: f64) -> Result<f64, String>` (lỗi khi b=0)
2. Viết hàm parse "key=value" → tuple, trả Result nếu format sai
3. Chuỗi xử lý: đọc file → parse số → nhân 2 → in kết quả (dùng `?`)

---

📖 **Trước đó**: [Chương 4](../chuong-04-struct-enum/README.md) | **Tiếp theo**: [Chương 6](../chuong-06-collections-generics/README.md)
